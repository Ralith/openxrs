#![recursion_limit = "256"]

use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    rc::Rc,
};

use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use indexmap::{IndexMap, IndexSet};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use syn::{Ident, LitByteStr};
use xml::{
    attribute::OwnedAttribute,
    reader::{EventReader, ParserConfig, XmlEvent},
};

fn main() {
    let mut args = env::args_os();
    args.next().unwrap();

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source =
        File::open(args.next().map(PathBuf::from).unwrap_or_else(|| {
            manifest_dir.join("../sys/OpenXR-SDK/specification/registry/xr.xml")
        }))
        .expect("failed to open registry XML file");

    let mut parser = Parser::new(source);
    parser.parse();
    let mut sys_out = File::create(manifest_dir.join("../sys/src/generated.rs")).unwrap();
    let mut hl_out = File::create(manifest_dir.join("../openxr/src/generated.rs")).unwrap();
    write!(sys_out, "{}", parser.generate_sys()).unwrap();
    write!(hl_out, "{}", parser.generate_hl()).unwrap();
}

struct Parser {
    reader: EventReader<File>,
    enums: IndexMap<String, Enum<i32>>,
    bitmasks: IndexMap<String, Enum<u64>>,
    /// *FlagBits -> *Flags
    bitvalues: HashMap<String, String>,
    handles: IndexSet<String>,
    structs: IndexMap<String, Struct>,
    struct_aliases: Vec<(String, String)>,
    api_constants: Vec<(String, usize)>,
    api_aliases: Vec<(String, String)>,
    commands: IndexMap<String, Command>,
    cmd_aliases: Vec<(String, String)>,
    extensions: IndexMap<String, Tag>,
    disabled_exts: HashSet<Rc<str>>,
    api_version: Option<(u16, u16, u32)>,
    base_headers: IndexMap<String, Vec<String>>,
}

impl Parser {
    fn new(source: File) -> Self {
        Self {
            reader: ParserConfig::new()
                .trim_whitespace(true)
                .ignore_comments(true)
                .create_reader(source),
            enums: IndexMap::new(),
            bitmasks: IndexMap::new(),
            bitvalues: HashMap::new(),
            handles: IndexSet::new(),
            structs: IndexMap::new(),
            struct_aliases: Vec::new(),
            api_constants: Vec::new(),
            api_aliases: Vec::new(),
            commands: IndexMap::new(),
            cmd_aliases: Vec::new(),
            extensions: IndexMap::new(),
            // TODO: Handle these extensions
            disabled_exts: [
                "XR_MSFT_scene_marker",
                "XR_MSFT_scene_understanding",
                "XR_MSFT_scene_understanding_serialization",
            ]
            .iter()
            .copied()
            .map(Into::into)
            .collect(),
            api_version: None,
            base_headers: IndexMap::new(),
        }
    }

    fn parse(&mut self) {
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement {
                    name, attributes, ..
                } => match &name.local_name[..] {
                    "types" => self.parse_types(),
                    "registry" => {}
                    "comment" => self.finish_element(),
                    "tags" => {
                        self.parse_tags();
                    }
                    "enums" => {
                        self.parse_enum_values(&attributes);
                    }
                    "commands" => {
                        self.parse_commands();
                    }
                    "extensions" => {
                        self.parse_extensions();
                    }
                    _ => {
                        eprintln!("unimplemented root element: {}", name.local_name);
                        self.finish_element();
                    }
                },
                EndDocument => {
                    break;
                }
                _ => {}
            }
        }
    }

    fn parse_tags(&mut self) {
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement {
                    name, attributes, ..
                } => match &name.local_name[..] {
                    "tag" => {
                        let name = attr(&attributes, "name").unwrap();
                        self.extensions.insert(
                            name.into(),
                            Tag {
                                extensions: Vec::new(),
                            },
                        );
                        self.finish_element();
                    }
                    _ => {
                        eprintln!("unimplemented extensions element: {}", name.local_name);
                        self.finish_element();
                    }
                },
                EndElement { name } => {
                    if name.local_name == "tags" {
                        break;
                    }
                    eprintln!("unexpected end element: {}", name);
                }
                EndDocument => {
                    panic!("unexpected end of document");
                }
                _ => {}
            }
        }
    }

    fn parse_extensions(&mut self) {
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement {
                    name, attributes, ..
                } => match &name.local_name[..] {
                    "extension" => {
                        if let StartElement { name, .. } =
                            self.reader.next().expect("failed to parse XML")
                        {
                            if name.local_name == "require" {
                                self.parse_extension_required(&attributes);
                            } else {
                                eprintln!("unexpected extension element: {}", name.local_name);
                            }
                        } else {
                            eprintln!("unexpected extension event");
                        }
                        self.finish_element();
                    }
                    _ => {
                        eprintln!("unimplemented extensions element: {}", name.local_name);
                        self.finish_element();
                    }
                },
                EndElement { name } => {
                    if name.local_name == "extensions" {
                        break;
                    }
                    eprintln!("unexpected end element: {}", name);
                }
                EndDocument => {
                    panic!("unexpected end of document");
                }
                _ => {}
            }
        }
    }

    fn parse_extension_required(&mut self, attrs: &[OwnedAttribute]) {
        let ext_name = Rc::<str>::from(attr(attrs, "name").unwrap());
        let ext_number = attr(attrs, "number").unwrap().parse::<i32>().unwrap();
        let mut ext_version = None;
        let mut commands = Vec::new();
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement {
                    name, attributes, ..
                } => {
                    match &name.local_name[..] {
                        "command" => {
                            let cmd = attr(&attributes, "name").unwrap();
                            if let Some(command) = self.commands.get_mut(cmd) {
                                command.extension = Some(ext_name.clone());
                            }
                            commands.push(cmd.into());
                        }
                        "enum" => {
                            let name = attr(&attributes, "name").unwrap();
                            if let Some(extends) = attr(&attributes, "extends") {
                                const EXT_BASE: i32 = 1_000_000_000;
                                const EXT_BLOCK_SIZE: i32 = 1000;

                                let value = if let Some(offset) = attr(&attributes, "offset") {
                                    let offset = offset.parse::<i32>().unwrap();
                                    let sign =
                                        if attr(&attributes, "dir").map_or(false, |x| x == "-") {
                                            -1
                                        } else {
                                            1
                                        };
                                    ConstantValue::Literal(
                                        sign * (EXT_BASE
                                            + (ext_number - 1) * EXT_BLOCK_SIZE
                                            + offset),
                                    )
                                } else if let Some(bitpos) = attr(&attributes, "bitpos") {
                                    ConstantValue::Literal(bitpos.parse::<i32>().unwrap())
                                } else {
                                    ConstantValue::Alias(attr(&attributes, "alias").unwrap().into())
                                };
                                let bitmasks = &mut self.bitmasks;
                                if let Some(e) = self.enums.get_mut(extends) {
                                    e.values.push(Constant {
                                        name: name.into(),
                                        value,
                                        comment: attr(&attributes, "comment")
                                            .and_then(tidy_comment),
                                    });
                                } else if let Some(e) = self
                                    .bitvalues
                                    .get(extends)
                                    .and_then(|x| bitmasks.get_mut(x))
                                {
                                    e.values.push(Constant {
                                        name: name.into(),
                                        value: match value {
                                            ConstantValue::Literal(x) => {
                                                ConstantValue::Literal(x as u64)
                                            }
                                            ConstantValue::Alias(x) => ConstantValue::Alias(x),
                                        },
                                        comment: attr(&attributes, "comment")
                                            .and_then(tidy_comment),
                                    });
                                } else {
                                    eprintln!("extension to unrecognized type {}", extends);
                                }
                            } else if let Some(alias) = attr(&attributes, "alias") {
                                self.api_aliases.push((name.into(), alias.into()));
                            } else if name.ends_with("SPEC_VERSION") {
                                let value = attr(&attributes, "value").unwrap();
                                ext_version = Some(value.parse().unwrap());
                            } else if name.ends_with("EXTENSION_NAME") {
                                let value = attr(&attributes, "value").unwrap();
                                assert_eq!(&ext_name[..], &value[1..value.len() - 1]);
                            } else {
                                let value = attr(&attributes, "value").unwrap();
                                self.api_constants
                                    .push((name.into(), value.parse().unwrap()));
                            }
                        }
                        "type" => {
                            let ty = attr(&attributes, "name").unwrap();
                            if let Some(s) = self.structs.get_mut(ty) {
                                s.extension = Some(ext_name.clone());
                            }
                        }
                        _ => {
                            eprintln!("unimplemented extension element: {}", name.local_name);
                        }
                    }
                    self.finish_element();
                }
                EndElement { name } => {
                    if name.local_name == "require" {
                        break;
                    }
                    eprintln!("unexpected end element: {}", name);
                }
                EndDocument => {
                    panic!("unexpected end of document");
                }
                _ => {}
            }
        }
        if attr(attrs, "supported").map_or(false, |x| x == "disabled") {
            self.disabled_exts.insert(ext_name);
        } else {
            let provisional = attr(attrs, "provisional").map_or(false, |x| x == "true");

            let (tag_name, _) = split_ext_tag(&ext_name);
            let ext = Extension {
                name: ext_name.clone(),
                version: ext_version.unwrap(),
                commands,
            };

            if let Some(tag) = self.extensions.get_mut(tag_name) {
                tag.extensions.push(ext);
            } else if provisional {
                self.extensions.insert(
                    tag_name.into(),
                    Tag {
                        extensions: vec![ext],
                    },
                );
            } else {
                eprintln!("ignoring extension with unlisted tag: {}", ext_name);
            }
        }
    }

    fn parse_commands(&mut self) {
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement {
                    name, attributes, ..
                } => match &name.local_name[..] {
                    "command" => {
                        self.parse_command(&attributes);
                    }
                    _ => {
                        eprintln!("unimplemented commands element: {}", name.local_name);
                    }
                },
                EndElement { name } => {
                    if name.local_name == "commands" {
                        break;
                    }
                    eprintln!("unexpected end element: {}", name);
                }
                EndDocument => {
                    panic!("unexpected end of document");
                }
                _ => {}
            }
        }
    }

    fn parse_command(&mut self, attrs: &[OwnedAttribute]) {
        let mut command_name = None;
        let mut params = Vec::new();
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement {
                    name, attributes, ..
                } => match &name.local_name[..] {
                    "proto" => {
                        if let StartElement { name, .. } =
                            self.reader.next().expect("failed to parse XML")
                        {
                            if name.local_name == "type" {
                                assert_eq!(self.parse_characters(), "XrResult");
                                self.finish_element()
                            } else {
                                panic!("unexpected element following proto: {}", name);
                            }
                        } else {
                            panic!("unexpected event following proto");
                        }
                        if let StartElement { name, .. } =
                            self.reader.next().expect("failed to parse XML")
                        {
                            if name.local_name == "name" {
                                command_name = Some(self.parse_characters());
                                self.finish_element();
                            } else {
                                panic!("unexpected element following proto: {}", name);
                            }
                        } else {
                            panic!("unexpected event following proto");
                        }
                        self.finish_element();
                    }
                    "param" => {
                        params.push(self.parse_var("param", &attributes));
                    }
                    "implicitexternsyncparams" => {
                        self.finish_element();
                    }
                    _ => {
                        eprintln!("unimplemented command element: {}", name.local_name);
                        self.finish_element();
                    }
                },
                EndElement { name } => {
                    if name.local_name == "command" {
                        break;
                    }
                    eprintln!("unexpected end element: {}", name);
                }
                EndDocument => {
                    panic!("unexpected end of document");
                }
                _ => {}
            }
        }
        if let Some(name) = command_name {
            self.commands.insert(
                name,
                Command {
                    params,
                    extension: None,
                },
            );
        } else {
            let name = attr(attrs, "name").unwrap();
            let alias_of = attr(attrs, "alias").unwrap();
            self.cmd_aliases.push((name.into(), alias_of.into()));
        }
    }

    fn parse_types(&mut self) {
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement {
                    name, attributes, ..
                } => match &name.local_name[..] {
                    "type" => {
                        self.parse_type(&attributes);
                    }
                    "comment" => {
                        self.finish_element();
                    }
                    _ => {
                        eprintln!("unimplemented type element: {}", name.local_name);
                    }
                },
                EndElement { name } => {
                    if name.local_name == "types" {
                        break;
                    }
                    eprintln!("unexpected end element: {}", name);
                }
                EndDocument => {
                    panic!("unexpected end of document");
                }
                _ => {}
            }
        }
    }

    fn parse_type(&mut self, attrs: &[OwnedAttribute]) {
        let name = attr(attrs, "name");
        match attr(attrs, "category") {
            Some("enum") => {
                let name = name.unwrap();
                if !split_ty_ext(name).0.ends_with("FlagBits") {
                    self.enums.insert(name.into(), Enum::new());
                }
                self.finish_element();
            }
            Some("bitmask") => {
                self.parse_bitmask(attrs);
            }
            Some("handle") => {
                self.parse_handle();
            }
            Some("struct") => {
                self.parse_struct(attrs);
            }
            Some("define") => {
                self.parse_define();
            }
            Some("include") | Some("basetype") | Some("funcpointer") | None => {
                // Intentionally skipped
                self.finish_element();
            }
            Some(other) => {
                eprintln!("unimplemented type category: {}", other);
                self.finish_element();
            }
        }
    }

    fn parse_define(&mut self) {
        let mut define_name = None;
        let mut define_ty = None;
        let mut define_val = None;
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement { name, .. } => {
                    match &name.local_name[..] {
                        "name" => {
                            define_name = Some(self.parse_characters());
                        }
                        "type" => {
                            define_ty = Some(self.parse_characters());
                        }
                        x => {
                            panic!("unexpected element: {}", x);
                        }
                    }
                    self.finish_element();
                }
                Characters(ch) => {
                    if define_ty.is_some() || define_name.is_some() {
                        define_val = Some(ch);
                    }
                }
                EndElement { name } => {
                    if name.local_name == "type" {
                        break;
                    }
                    eprintln!("unexpected end element in define type: {}", name);
                }
                _ => {}
            }
        }
        if let (Some(name), Some(val)) = (&define_name, &define_val) {
            if let Ok(val) = val.parse::<usize>() {
                self.api_constants.push((name.into(), val))
            }
        }
        if define_name.as_ref().map(|x| &x[..]) == Some("XR_CURRENT_API_VERSION") {
            let version = define_val.unwrap();
            assert!(version.starts_with('('));
            assert!(version.ends_with(')'));
            let version = &version[1..version.len() - 1];
            let mut iter = version.split(", ").map(|x| x.parse::<u64>().unwrap());
            self.api_version = Some((
                iter.next().unwrap() as u16,
                iter.next().unwrap() as u16,
                iter.next().unwrap() as u32,
            ));
        }
    }

    fn parse_struct(&mut self, attrs: &[OwnedAttribute]) {
        let struct_name = attr(attrs, "name").unwrap();
        let mut members = Vec::new();
        let mut ty = None;
        let mut mut_next = false;
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement {
                    name, attributes, ..
                } => match &name.local_name[..] {
                    "member" => {
                        let m = self.parse_var("member", &attributes);
                        if m.name == "type" && m.ty == "XrStructureType" {
                            ty = attr(&attributes, "values").map(|x| x.into());
                        } else if m.name == "next" && !m.is_const {
                            mut_next = true;
                        }
                        members.push(m);
                    }
                    _ => {
                        eprintln!("unimplemented struct element: {}", name.local_name);
                        self.finish_element();
                    }
                },
                EndElement { name } => {
                    if name.local_name == "type" {
                        break;
                    }
                    eprintln!("unexpected end element in bitmask type: {}", name);
                }
                _ => {}
            }
        }
        let parent = attr(attrs, "parentstruct");
        if let Some(parent) = parent {
            self.base_headers
                .entry(parent.into())
                .or_default()
                .push(struct_name.into());
        }
        if let Some(target) = attr(attrs, "alias") {
            self.struct_aliases
                .push((struct_name.into(), target.into()));
        } else {
            self.structs.insert(
                struct_name.into(),
                Struct {
                    members,
                    ty,
                    extension: None,
                    mut_next,
                },
            );
        }
    }

    fn parse_var(&mut self, elt_name: &'static str, attrs: &[OwnedAttribute]) -> Member {
        let mut member_name = None;
        let mut member_ty = None;
        let mut is_const = false;
        let mut ptr_depth = 0;
        let mut static_array_len = None;
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement { name, .. } => {
                    match &name.local_name[..] {
                        "type" => {
                            member_ty = Some(self.parse_characters());
                        }
                        "name" => {
                            member_name = Some(self.parse_characters());
                        }
                        "enum" => {
                            static_array_len = Some(self.parse_characters());
                        }
                        x => {
                            panic!("unexpected element: {}", x);
                        }
                    }
                    self.finish_element();
                }
                Characters(ch) => {
                    if member_ty.is_none() {
                        is_const = ch.starts_with("const");
                    } else if member_name.is_none() {
                        ptr_depth = ch.chars().filter(|&x| x == '*').count();
                    } else if ch.starts_with('[') && ch.len() > 1 {
                        assert!(ch.ends_with(']'));
                        static_array_len = Some(ch[1..ch.len() - 1].into());
                    }
                }
                EndElement { name } => {
                    if name.local_name == elt_name {
                        break;
                    }
                    eprintln!("unexpected end element in bitmask type: {}", name);
                }
                _ => {}
            }
        }
        Member {
            name: member_name.unwrap(),
            is_const,
            ptr_depth,
            ty: member_ty.unwrap(),
            static_array_len,
            len: attr(attrs, "len").map(|x| x.split(',').map(|x| x.into()).collect()),
        }
    }

    fn parse_handle(&mut self) {
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement { name, .. } => {
                    match &name.local_name[..] {
                        "name" => {
                            let name = self.parse_characters();
                            self.handles.insert(name);
                        }
                        "type" => {
                            assert_eq!(self.parse_characters(), "XR_DEFINE_HANDLE");
                        }
                        _ => {
                            eprintln!("unimplemented handle element: {}", name.local_name);
                        }
                    }
                    self.finish_element();
                }
                EndElement { name } => {
                    if name.local_name == "type" {
                        break;
                    }
                    eprintln!("unexpected end element in handle type: {}", name);
                }
                _ => {}
            }
        }
    }

    fn parse_bitmask(&mut self, attrs: &[OwnedAttribute]) {
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement { name, .. } => {
                    match &name.local_name[..] {
                        "name" => {
                            let name = self.parse_characters();
                            self.bitvalues
                                .insert(attr(attrs, "bitvalues").unwrap().into(), name.clone());
                            self.bitmasks.insert(name, Enum::new());
                        }
                        "type" => {
                            assert_eq!(self.parse_characters(), "XrFlags64");
                        }
                        _ => {
                            eprintln!("unimplemented bitmask element: {}", name.local_name);
                        }
                    }
                    self.finish_element();
                }
                EndElement { name } => {
                    if name.local_name == "type" {
                        break;
                    }
                    eprintln!("unexpected end element in bitmask type: {}", name);
                }
                _ => {}
            }
        }
    }

    fn parse_enum_values(&mut self, attrs: &[OwnedAttribute]) {
        let name = attr(attrs, "name").unwrap();
        let ty = attr(attrs, "type");
        if let Some(comment) = attr(attrs, "comment").and_then(tidy_comment) {
            if let Some(item) = self.enums.get_mut(name) {
                item.comment = Some(comment);
            }
        }

        match ty {
            Some("enum") => loop {
                use XmlEvent::*;
                let item = self.enums.get_mut(name).unwrap();
                match self.reader.next().expect("failed to parse XML") {
                    StartElement {
                        name, attributes, ..
                    } => {
                        if name.local_name == "enum" {
                            let name = attr(&attributes, "name").unwrap();
                            let value = attr(&attributes, "value").unwrap();
                            let comment = attr(&attributes, "comment");
                            item.values.push(Constant {
                                name: name.into(),
                                value: ConstantValue::Literal(value.parse().unwrap()),
                                comment: comment.and_then(tidy_comment),
                            });
                        }
                        self.finish_element();
                    }
                    EndElement { name } => match &name.local_name[..] {
                        "enums" => {
                            break;
                        }
                        _ => {
                            panic!("unexpected end elt");
                        }
                    },
                    _ => {}
                }
            },
            Some("bitmask") => loop {
                use XmlEvent::*;
                let bitmask = self
                    .bitmasks
                    .get_mut(self.bitvalues.get(name).expect("unknown FlagBits"))
                    .expect("unknown bitmask");
                match self.reader.next().expect("failed to parse XML") {
                    StartElement {
                        name, attributes, ..
                    } => {
                        if name.local_name == "enum" {
                            let name = attr(&attributes, "name").unwrap();
                            let bitpos = attr(&attributes, "bitpos").unwrap();
                            let comment = attr(&attributes, "comment");
                            bitmask.values.push(Constant {
                                name: name.into(),
                                value: ConstantValue::Literal(bitpos.parse().unwrap()),
                                comment: comment.and_then(tidy_comment),
                            });
                        }
                        self.finish_element();
                    }
                    EndElement { name } => match &name.local_name[..] {
                        "enums" => {
                            break;
                        }
                        _ => {
                            panic!("unexpected end elt");
                        }
                    },
                    _ => {}
                }
            },
            None => {
                assert_eq!(name, "API Constants");
                loop {
                    use XmlEvent::*;
                    match self.reader.next().expect("failed to parse XML") {
                        StartElement {
                            name, attributes, ..
                        } => {
                            if name.local_name == "enum" {
                                let name = attr(&attributes, "name").unwrap();
                                if name != "XR_TRUE" && name != "XR_FALSE" {
                                    let value = attr(&attributes, "value").unwrap();
                                    self.api_constants
                                        .push((name.into(), value.parse().unwrap()));
                                }
                            }
                            self.finish_element();
                        }
                        EndElement { name } => match &name.local_name[..] {
                            "enums" => {
                                break;
                            }
                            _ => {
                                panic!("unexpected end elt: {}", name);
                            }
                        },
                        _ => {}
                    }
                }
            }
            Some(other) => {
                eprintln!("unimplemented enum type: {}", other);
                self.finish_element();
            }
        }
    }

    fn finish_element(&mut self) {
        let mut depth: u32 = 0;
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement { .. } => {
                    depth += 1;
                }
                EndElement { .. } => {
                    if let Some(x) = depth.checked_sub(1) {
                        depth = x;
                    } else {
                        break;
                    }
                }
                EndDocument => {
                    panic!("unexpected end of document");
                }
                _ => {}
            }
        }
    }

    fn parse_characters(&mut self) -> String {
        match self.reader.next().expect("parse error") {
            XmlEvent::Characters(x) => x,
            e => panic!("unexpected event: {:?}", e),
        }
    }

    fn spec_link(&self, anchor: &str) -> String {
        let (major, minor, _) = self.api_version.unwrap();
        format!(
            "https://www.khronos.org/registry/OpenXR/specs/{}.{}/html/xrspec.html#{}",
            major, minor, anchor
        )
    }

    fn doc_link(&self, name: &str) -> String {
        let name = if let Some(stripped) = name.strip_suffix("Flags") {
            format!("{}FlagBits", stripped)
        } else {
            name.into()
        };
        format!("[{}]({})", name, self.spec_link(&name))
    }

    fn generate_sys(&self) -> TokenStream {
        let consts = self.api_constants.iter().map(|(name, value)| {
            let ident = Ident::new(&name[3..], Span::call_site());
            quote! {
                pub const #ident: usize = #value;
            }
        });

        let enums = self.enums.iter().map(|(name, e)| {
            let ident = xr_ty_name(name);
            let values = e.values.iter().map(|v| {
                let value_name = xr_enum_value_name(name, &v.name);
                let value = match v.value {
                    ConstantValue::Literal(x) => quote! {Self(#x)},
                    ConstantValue::Alias(ref x) => {
                        let ident = xr_enum_value_name(name, x);
                        quote! { Self::#ident }
                    }
                };
                let doc = if let Some(comment) = v.comment.as_ref() {
                    quote! {#[doc = #comment]}
                } else {
                    quote! {}
                };
                quote! {
                    #doc
                    pub const #value_name: #ident = #value;
                }
            });
            let doc = if let Some(comment) = e.comment.as_ref() {
                format!("{} - see {}", comment, self.doc_link(name))
            } else {
                format!("See {}", self.doc_link(name))
            };
            let debug_cases = e.values.iter().filter_map(|v| {
                if matches!(v.value, ConstantValue::Alias(_)) {
                    // Skip unreachable cases
                    return None;
                }
                let ident = xr_enum_value_name(name, &v.name);
                let name = ident.to_string();
                Some(quote! {
                    Self::#ident => Some(#name)
                })
            });
            let result_extras = if name == "XrResult" {
                let cases = e.values.iter().map(|v| {
                    let ident = xr_enum_value_name(name, &v.name);
                    let reason = v.comment.as_ref().map_or_else(
                        || ident.to_string(),
                        |x| {
                            let mut reason = x.to_string();
                            reason.get_mut(0..1).unwrap().make_ascii_lowercase();
                            reason = reason.trim_end_matches('.').to_string();
                            reason
                        },
                    );
                    quote! {
                        Self::#ident => Some(#reason)
                    }
                });
                quote! {
                    impl fmt::Display for #ident {
                        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                            let reason = match *self {
                                #(#cases,)*
                                _ => None,
                            };
                            if let Some(reason) = reason {
                                fmt.pad(reason)
                            } else {
                                write!(fmt, "unknown error (code {})", self.0)
                            }
                        }
                    }
                    impl std::error::Error for #ident {}
                }
            } else {
                quote! {}
            };
            quote! {
                #[doc = #doc]
                #[repr(transparent)]
                #[derive(Copy, Clone, Eq, PartialEq)]
                pub struct #ident(i32);
                impl #ident {
                    #(#values)*

                    pub fn from_raw(x: i32) -> Self { Self(x) }
                    pub fn into_raw(self) -> i32 { self.0 }
                }
                impl fmt::Debug for #ident {
                    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                        let name = match *self {
                            #(#debug_cases,)*
                            _ => None,
                        };
                        fmt_enum(fmt, self.0, name)
                    }
                }
                #result_extras
            }
        });

        let bitmasks = self.bitmasks.iter().map(|(name, bitmask)| {
            let ident = xr_ty_name(name);
            let doc = if let Some(comment) = bitmask.comment.as_ref() {
                format!("{} - see {}", comment, self.doc_link(name))
            } else {
                format!("See {}", self.doc_link(name))
            };
            let values = bitmask.values.iter().map(|v| {
                let value_name = xr_bitmask_value_name(name, &v.name);
                let value = match v.value {
                    ConstantValue::Literal(x) => quote! {Self(1 << #x)},
                    ConstantValue::Alias(ref x) => {
                        // avoids collision between XR_SWAPCHAIN_USAGE_INPUT_ATTACHMENT_BIT_{MND,KHR}
                        if bitmask
                            .values
                            .iter()
                            .any(|v| value_name == xr_bitmask_value_name(name, &v.name))
                        {
                            return quote! {};
                        }
                        let ident = xr_enum_value_name(name, x);
                        quote! { Self::#ident }
                    }
                };
                let doc = if let Some(comment) = v.comment.as_ref() {
                    quote! {#[doc = #comment]}
                } else {
                    quote! {}
                };
                quote! {
                    #doc
                    pub const #value_name: #ident = #value;
                }
            });
            quote! {
                #[doc = #doc]
                #[repr(transparent)]
                #[derive(Debug, Copy, Clone, Eq, PartialEq)]
                pub struct #ident(u64);
                impl #ident {
                    #(#values)*

                }
                bitmask!(#ident);
            }
        });

        let handles = self.handles.iter().map(|name| {
            let ident = xr_ty_name(name);
            let doc = format!("See {}", self.doc_link(name));
            quote! {
                #[doc = #doc]
                #[repr(transparent)]
                #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
                pub struct #ident(u64);
                handle!(#ident);
            }
        });

        let structs = self.structs.iter().map(|(name, s)| {
            let ident = xr_ty_name(name);
            let members = s.members.iter().map(|m| {
                let ident = xr_var_name(&m.name);
                let ty = xr_var_ty(self.api_aliases.as_ref(), m);
                quote! {
                    pub #ident: #ty,
                }
            });
            let doc = if let Some(ref ext) = s.extension {
                if self.disabled_exts.contains(ext) {
                    return quote! {};
                }
                format!(
                    "See {} - defined by [{}]({})",
                    self.doc_link(name),
                    ext,
                    self.spec_link(ext)
                )
            } else {
                format!("See {}", self.doc_link(name))
            };
            let conditions = conditions(name, s.extension.as_ref().map(|x| &x[..]));
            let ty = if let Some(ref ty) = s.ty {
                let conditions2 = conditions.clone();
                let ty = xr_enum_value_name("XrStructureType", ty);
                let out = if s.mut_next {
                    quote! {
                        /// Construct a partially-initialized value suitable for passing to OpenXR
                        #[inline]
                        pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
                            let mut x = MaybeUninit::<Self>::uninit();
                            unsafe {
                                (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                                    ty: Self::TYPE,
                                    next,
                                });
                            }
                            x
                        }
                    }
                } else {
                    quote! {}
                };
                quote! {
                    #conditions2
                    impl #ident {
                        pub const TYPE: StructureType = StructureType::#ty;
                        #out
                    }
                }
            } else {
                quote! {}
            };
            let meta = self.compute_meta(name, s);
            let derives = if (meta.has_pointer || meta.has_array) && meta.has_unprintable {
                quote! { #[derive(Copy, Clone)] }
            } else if meta.has_pointer || meta.has_array {
                quote! { #[derive(Copy, Clone, Debug)] }
            } else if meta.has_non_default {
                quote! { #[derive(Copy, Clone, Debug, PartialEq)] }
            } else {
                quote! { #[derive(Copy, Clone, Debug, Default, PartialEq)] }
            };
            let aliases = self.struct_aliases.iter().map(|(alias, alias_to)| {
                if alias_to != name {
                    return quote! {};
                }
                let alias_ident = xr_ty_name(alias);
                quote! { pub type #alias_ident = #ident; }
            });
            quote! {
                #[repr(C)]
                #derives
                #[doc = #doc]
                #conditions
                pub struct #ident {
                    #(#members)*
                }
                #ty
                #(#aliases)*
            }
        });

        let commands = self.commands.iter().chain(
            self.cmd_aliases
                .iter()
                .map(|(name, target)| (name, self.commands.get(target).unwrap())),
        );

        let (pfns, protos) = commands
            .map(|(name, command)| {
                let ident = xr_command_name(name);
                let params = command.params.iter().map(|param| {
                    let ident = xr_var_name(&param.name);
                    let ty = xr_arg_ty(self.api_aliases.as_ref(), param);
                    quote! {
                        #ident: #ty
                    }
                });
                let doc = if let Some(ref ext) = command.extension {
                    if self.disabled_exts.contains(ext) {
                        return (quote! {}, quote! {});
                    }
                    format!(
                        "See {} - defined by [{}]({})",
                        self.doc_link(name),
                        ext,
                        self.spec_link(ext)
                    )
                } else {
                    format!("See {}", self.doc_link(name))
                };
                let conditions = conditions(name, command.extension.as_ref().map(|x| &x[..]));
                let conditions2 = conditions.clone();
                let params2 = params.clone();
                let pfn_def = quote! {
                    #conditions
                    #[doc = #doc]
                    pub type #ident = unsafe extern "system" fn(#(#params),*) -> Result;
                };
                let proto = if command.extension.is_some() {
                    quote! {}
                } else {
                    let fn_ident =
                        Ident::new(&ident.to_string().to_snake_case(), Span::call_site());
                    quote! {
                        #conditions2
                        #[link_name = #name]
                        pub fn #fn_ident(#(#params2),*) -> Result;
                    }
                };
                (pfn_def, proto)
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let ext_consts = self.extensions.iter().flat_map(|(_, tag)| {
            tag.extensions.iter().map(move |ext| {
                assert!(ext.name.starts_with("XR_"));
                let trimmed = &ext.name[3..];
                let name_ident = Ident::new(
                    &format!("{}_EXTENSION_NAME", trimmed.to_uppercase()),
                    Span::call_site(),
                );
                let name_lit = c_name(&ext.name);
                let version_ident =
                    Ident::new(&format!("{}_SPEC_VERSION", trimmed), Span::call_site());
                let version_lit = ext.version;
                let conds = conditions(&ext.name, Some(&ext.name));
                // TODO: &'static CStr name
                quote! {
                    #conds
                    pub const #version_ident: u32 = #version_lit;
                    #conds
                    pub const #name_ident: &[u8] = #name_lit;
                }
            })
        });

        let (major, minor, patch) = self.api_version.unwrap();

        quote! {
            //! Automatically generated code; do not edit!

            #![allow(non_upper_case_globals, clippy::unreadable_literal, clippy::identity_op, unused, clippy::derive_partial_eq_without_eq)]
            use std::fmt;
            use std::mem::MaybeUninit;
            use std::os::raw::{c_void, c_char};
            use libc::{timespec, wchar_t};

            use crate::support::*;
            use crate::platform::*;
            use crate::*;

            pub const CURRENT_API_VERSION: Version = Version::new(#major, #minor, #patch);

            #(#consts)*
            #(#enums)*
            #(#bitmasks)*
            #(#handles)*
            #(#structs)*

            /// Function pointer prototypes
            pub mod pfn {
                use super::*;
                pub use crate::platform::EglGetProcAddressMNDX;

                pub type VoidFunction = unsafe extern "system" fn();
                pub type DebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
                    DebugUtilsMessageSeverityFlagsEXT,
                    DebugUtilsMessageTypeFlagsEXT,
                    *const DebugUtilsMessengerCallbackDataEXT,
                    *mut c_void,
                ) -> Bool32;

                #(#pfns)*
            }

            #(#ext_consts)*

            #[cfg(feature = "linked")]
            extern "system" {
                #(#protos)*
            }
        }
    }

    /// Generate high-level code
    #[allow(clippy::cognitive_complexity)] // TODO
    fn generate_hl(&self) -> TokenStream {
        const BLACKLISTED_COMMANDS: [&str; 3] = [
            "xrNegotiateLoaderRuntimeInterface",
            "xrNegotiateLoaderApiLayerInterface",
            "xrCreateApiLayerInstance",
        ];
        let (instance_pfn_fields, instance_pfn_inits) = self.commands.iter().map(|(name, command)| {
            if command.extension.is_some() || BLACKLISTED_COMMANDS.contains(&name.as_str()) {
                return (quote! {}, quote! {});
            }
            let pfn_ident = xr_command_name(name);
            let field_ident = Ident::new(&pfn_ident.to_string().to_snake_case(), Span::call_site());
            let field = quote! {
                pub #field_ident: pfn::#pfn_ident,
            };
            let c_name = c_name(name);
            let init = quote! {
                #field_ident: mem::transmute(entry.get_instance_proc_addr(instance, CStr::from_bytes_with_nul_unchecked(#c_name))?),
            };
            (field, init)
        }).unzip::<_, _, Vec<_>, Vec<_>>();

        let mut exts = Vec::new();
        let mut ext_fields = Vec::new();
        let mut ext_field_inits = Vec::new();
        let mut ext_set_names = Vec::new();
        let mut ext_set_fields = Vec::new();
        let mut ext_set_inits = Vec::new();
        for (tag_name, tag) in &self.extensions {
            for ext in &tag.extensions {
                if self.disabled_exts.contains(&ext.name) {
                    continue;
                }
                let (pfns, pfn_inits) = ext.commands.iter().map(|cmd| {
                    let pfn_ident = xr_command_name(cmd);
                    let camel_name = pfn_ident.to_string();
                    let field_ident = Ident::new(&camel_name[..camel_name.len()-tag_name.len()].to_snake_case(), Span::call_site());
                    let pfn = quote! {
                        pub #field_ident: pfn::#pfn_ident
                    };
                    let c_name = c_name(cmd);
                    let init = quote! {
                        #field_ident: mem::transmute(entry.get_instance_proc_addr(instance, CStr::from_bytes_with_nul_unchecked(#c_name))?),
                    };
                    (pfn, init)
                }).unzip::<_, _, Vec<_>, Vec<_>>();

                let name_ident = Ident::new("NAME", Span::call_site());
                let version_ident = Ident::new("VERSION", Span::call_site());
                assert!(ext.name.starts_with("XR_"));
                let trimmed = &ext.name[3..];
                let name_const = Ident::new(
                    &format!("{}_EXTENSION_NAME", trimmed.to_uppercase()),
                    Span::call_site(),
                );
                let version_const =
                    Ident::new(&format!("{}_SPEC_VERSION", trimmed), Span::call_site());
                let ext_name = split_ext_tag(&ext.name).1;
                let ty_ident = Ident::new(
                    &format!("{}{}", ext_name.to_upper_camel_case(), tag_name),
                    Span::call_site(),
                );
                let conds = conditions(&ext.name, Some(&ext.name));
                let conds2 = conds.clone();
                let conds3 = conds.clone();
                let conds4 = conds.clone();
                let conds5 = conds.clone();
                let conds6 = conds.clone();
                let conds7 = conds.clone();
                let load = if ext.commands.is_empty() {
                    quote! {}
                } else {
                    quote! {
                        /// Load the extension's function pointer table
                        ///
                        /// # Safety
                        ///
                        /// `instance` must be a valid instance handle.
                        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
                            Ok(Self {
                                #(#pfn_inits)*
                            })
                        }
                    }
                };
                exts.push(quote! {
                    #conds
                    #[derive(Copy, Clone)]
                    pub struct #ty_ident {
                        #(#pfns,)*
                    }

                    #conds2
                    impl #ty_ident {
                        pub const #version_ident: u32 = sys::#version_const;
                        pub const #name_ident: &'static [u8] = sys::#name_const;
                        #load
                    }
                });
                let field_ident = Ident::new(&trimmed.to_snake_case(), Span::call_site());
                ext_fields.push(quote! {
                    #conds3
                    pub #field_ident: Option<raw::#ty_ident>,
                });
                ext_field_inits.push(if ext.commands.is_empty() {
                    quote! {
                        #conds4
                        #field_ident: if required.#field_ident { Some(raw::#ty_ident {}) } else { None },
                    }
                } else {
                    quote! {
                        #conds4
                        #field_ident: if required.#field_ident { Some(raw::#ty_ident::load(entry, instance)?) } else { None },
                    }
                });
                ext_set_names.push(quote! {
                    #conds5
                    { if self.#field_ident { out.push(raw::#ty_ident::NAME.into()); } }
                });
                ext_set_inits.push(quote! {
                    #conds6
                    raw::#ty_ident::NAME => { out.#field_ident = true; }
                });
                ext_set_fields.push(quote! {
                    #conds7
                    pub #field_ident: bool,
                });
            }
        }

        let simple_structs = self
            .structs
            .iter()
            .filter_map(|(name, s)| {
                if s.extension
                    .as_ref()
                    .map_or(false, |ext| self.disabled_exts.contains(ext))
                {
                    return None;
                }
                if self.is_simple_struct(s) {
                    Some(&name[..])
                } else {
                    None
                }
            })
            .collect::<IndexSet<&str>>();

        let reexports = simple_structs
            .iter()
            .cloned()
            .chain(
                self.enums
                    .keys()
                    .filter(|&x| x != "XrResult")
                    .map(|x| &x[..]),
            )
            .chain(self.bitmasks.keys().map(|x| &x[..]))
            .map(xr_ty_name);

        let mut event_cases = Vec::new();
        let mut event_decodes = Vec::new();
        let mut event_readers = Vec::new();
        for (raw_name, evt) in self.structs.iter().filter(|(name, _)| {
            name.starts_with("XrEventData")
                && !name.ends_with("BaseHeader")
                && !name.ends_with("Buffer")
        }) {
            let raw_ident = xr_ty_name(raw_name);
            let name = &raw_name["XrEventData".len()..];
            let ident = Ident::new(name, Span::call_site());
            event_cases.push(if evt.members.len() <= 2 {
                assert_eq!(evt.members.len(), 2);
                quote! { #ident }
            } else {
                quote! {
                    #ident(#ident<'a>)
                }
            });
            let tag = xr_enum_value_name("XrStructureType", evt.ty.as_ref().unwrap());
            event_decodes.push(if evt.members.len() <= 2 {
                quote! {
                    sys::StructureType::#tag => Event::#ident,
                }
            } else {
                quote! {
                    sys::StructureType::#tag => {
                        let typed = &*(raw as *const sys::#raw_ident);
                        Event::#ident(#ident::new(typed))
                    }
                }
            });
            event_readers.push(self.generate_reader(&ident, &raw_ident, evt));
        }

        let mut struct_meta = HashMap::<&str, StructMeta>::new();
        for (name, s) in &self.structs {
            if name.starts_with("XrBase") {
                continue;
            }
            struct_meta.insert(name, self.compute_meta(name, s));
        }

        let polymorphic_builders = self.base_headers.iter().filter_map(|(name, children)| {
            if name == "XrSwapchainImageBaseHeader"
                || name == "XrEventDataBaseHeader"
                || name == "XrLoaderInitInfoBaseHeaderKHR"
            {
                return None;
            }
            Some(self.generate_polymorphic_builders(&struct_meta, &simple_structs, name, children))
        });

        let whitelist = [
            "XrCompositionLayerProjectionView",
            "XrSwapchainSubImage",
            "XrActionSetCreateInfo",
            "XrActionCreateInfo",
        ]
        .iter()
        .cloned()
        .collect::<HashSet<&str>>();
        let builders = self.structs.iter().filter_map(|(name, s)| {
            if whitelist.contains(&name[..]) {
                Some(self.generate_builder(&struct_meta, &simple_structs, name, s))
            } else {
                None
            }
        });

        quote! {
            //! Automatically generated code; do not edit!

            #![allow(clippy::wrong_self_convention, clippy::transmute_ptr_to_ptr)]
            use std::borrow::Cow;
            use std::ffi::CStr;
            use std::mem::MaybeUninit;
            pub use sys::{#(#reexports),*};
            pub use sys::platform::{EGLenum, VkFilter, VkSamplerMipmapMode, VkSamplerAddressMode, VkComponentSwizzle};

            use crate::*;

            /// A subset of known extensions
            #[derive(Debug, Clone, Eq, PartialEq, Default)]
            #[non_exhaustive]
            pub struct ExtensionSet {
                #(#ext_set_fields)*
                /// Extensions unknown to the high-level bindings
                pub other: Vec<String>,
            }

            impl ExtensionSet {
                pub(crate) fn from_properties(properties: &[sys::ExtensionProperties]) -> Self {
                    let mut out = Self::default();
                    for ext in properties {
                        match crate::fixed_str_bytes(&ext.extension_name) {
                            #(#ext_set_inits)*
                            bytes => {
                                let cstr = CStr::from_bytes_with_nul(bytes)
                                    .expect("extension names should be null terminated strings");
                                let string = cstr
                                    .to_str()
                                    .expect("extension names should be valid UTF-8")
                                    .to_string();
                                out.other.push(string);
                            }
                        }
                    }
                    out
                }

                pub(crate) fn names(&self) -> Vec<Cow<'static, [u8]>> {
                    let mut out = Vec::new();
                    #(#ext_set_names)*
                    for name in &self.other {
                        let mut bytes = Vec::with_capacity(name.len() + 1);
                        bytes.extend_from_slice(name.as_bytes());
                        bytes.push(0);
                        out.push(bytes.into());
                    }
                    out
                }
            }

            /// Extensions used internally by the bindings
            #[derive(Default, Copy, Clone)]
            pub struct InstanceExtensions {
                #(#ext_fields)*
            }

            impl InstanceExtensions {
                /// Load extension function pointer tables
                ///
                /// # Safety
                ///
                /// `instance` must be a valid instance handle.
                pub unsafe fn load(entry: &Entry, instance: sys::Instance, required: &ExtensionSet) -> Result<Self> {
                    Ok(Self {
                        #(#ext_field_inits)*
                    })
                }
            }

            #[derive(Copy, Clone)]
            #[non_exhaustive]
            pub enum Event<'a> {
                #(#event_cases,)*
            }

            impl<'a> Event<'a> {
                /// Decode an event
                ///
                /// Returns `None` if the event type is not recognized, e.g. if it's from an unknown extension
                ///
                /// # Safety
                ///
                /// `raw` must refer to an `EventDataBuffer` populated by a successful call to
                /// `xrPollEvent`, which has not been moved since.
                pub unsafe fn from_raw(raw: &'a MaybeUninit<sys::EventDataBuffer>) -> Option<Self> {
                    let raw = raw.as_ptr();
                    Some(match (raw as *const sys::BaseInStructure).read().ty {
                        #(#event_decodes)*
                        _ => { return None; }
                    })
                }
            }

            #(#event_readers)*

            pub mod raw {
                use std::{mem, ffi::CStr};
                use sys::pfn;
                use crate::{Entry, Result};

                #[derive(Copy, Clone)]
                pub struct Instance {
                    #(#instance_pfn_fields)*
                }

                impl Instance {
                    /// Load the core function pointer table
                    ///
                    /// # Safety
                    ///
                    /// `instance` must be a valid instance handle.
                    pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
                        Ok(Self {
                            #(#instance_pfn_inits)*
                        })
                    }
                }

                #(#exts)*
            }

            #[allow(unused)]
            pub(crate) mod builder {
                use std::{mem, marker::PhantomData, ops::Deref};

                use crate::*;

                #(#builders)*
                #(#polymorphic_builders)*
            }
        }
    }

    fn compute_meta(&self, name: &str, s: &Struct) -> StructMeta {
        let mut out = StructMeta::default();
        for member in &s.members {
            out.has_unprintable |= member.ty.starts_with("PFN") || member.ty == "LUID";
            out.has_pointer |= member.ptr_depth != 0 || self.handles.contains(&member.ty);
            out.has_graphics |= member.ty == "XrSession" || member.ty == "XrSwapchain";
            out.has_array |= member.static_array_len.is_some();
            out.has_non_default |= member.ty == "XrTime";
            if member.ty != name {
                if let Some(x) = self.structs.get(&member.ty) {
                    out |= self.compute_meta(&member.ty, x);
                }
                if self.enums.contains_key(&member.ty) {
                    out.has_non_default = true;
                }
            }
        }
        out
    }

    fn generate_polymorphic_builders(
        &self,
        meta: &HashMap<&str, StructMeta>,
        simple: &IndexSet<&str>,
        base_name: &str,
        children: &[String],
    ) -> TokenStream {
        let sys_ident = xr_ty_name(base_name);
        let base_ident = base_header_ty(base_name);
        let mut base_meta = StructMeta::default();
        for name in children {
            base_meta |= *meta.get(&name[..]).unwrap();
        }

        let (type_params, type_args, marker, marker_init) = base_meta.type_params();
        let builders = children.iter().map(|name| {
            if name == "XrCompositionLayerPassthroughHTC" {
                // XrCompositionLayerPassthroughHTC has problems with its setters so we skip for now.
                return quote! {};
            }
            let ident = xr_ty_name(name);
            let s = self.structs.get(name).unwrap();
            let conds = conditions(name, s.extension.as_ref().map(|x| &x[..]));
            let inits = self.generate_builder_inits(s);
            let setters = self.generate_setters(meta, simple, s);
            quote! {
                #conds
                #[derive(Copy, Clone)]
                #[repr(transparent)]
                pub struct #ident #type_params {
                    inner: sys::#ident,
                    #marker
                }
                #conds
                impl #type_params #ident #type_args {
                    #[inline]
                    pub fn new() -> Self {
                        Self {
                            inner: sys::#ident {
                                #inits
                                ..unsafe { mem::zeroed() }
                            },
                            #marker_init
                        }
                    }

                    /// Initialize with the supplied raw values
                    ///
                    /// # Safety
                    ///
                    /// The guarantees normally enforced by this builder (e.g. lifetimes) must be
                    /// preserved.
                    #[inline]
                    pub unsafe fn from_raw(inner: sys::#ident) -> Self {
                        Self {
                            inner,
                            #marker_init
                        }
                    }

                    #[inline]
                    pub fn into_raw(self) -> sys::#ident {
                        self.inner
                    }

                    #[inline]
                    pub fn as_raw(&self) -> &sys::#ident {
                        &self.inner
                    }

                    #setters
                }
                #conds
                impl #type_params Deref for #ident #type_args {
                    type Target = #base_ident #type_args;

                    #[inline]
                    fn deref(&self) -> &Self::Target {
                        unsafe { mem::transmute(&self.inner) }
                    }
                }
                #conds
                impl #type_params Default for #ident #type_args {
                    fn default() -> Self {
                        Self::new()
                    }
                }
            }
        });

        quote! {
            #[repr(transparent)]
            pub struct #base_ident #type_params {
                _inner: sys::#sys_ident,
                #marker
            }
            #(#builders)*
        }
    }

    fn generate_setters(
        &self,
        meta: &HashMap<&str, StructMeta>,
        simple: &IndexSet<&str>,
        s: &Struct,
    ) -> TokenStream {
        let lens = s
            .members
            .iter()
            .filter_map(|m| m.len.as_ref())
            .flat_map(|x| x.iter().filter(|&x| x != "null-terminated"))
            .map(|x| &x[..])
            .collect::<HashSet<&str>>();
        let setters = s.members.iter().filter_map(|m| {
            if m.name == "type" || m.name == "next" || lens.contains(&m.name[..]) {
                return None;
            }
            let ident = xr_var_name(&m.name);
            let type_args = if let Some(meta) = meta.get(&m.ty[..]) {
                let (_, type_args, _, _) = meta.type_params();
                type_args
            } else if m.ty == "XrSwapchain" || m.ty == "XrSession" {
                quote! { <G> }
            } else if m.ty == "XrAction" {
                quote! { <ATY> }
            } else {
                quote! {}
            };
            let (ty, init) = match &m.ty[..] {
                "XrResult" => (
                    quote! { sys::Result },
                    quote! { self.inner.#ident = value; },
                ),
                "XrBool32" => (
                    quote! { bool },
                    quote! { self.inner.#ident = value.into(); },
                ),
                x if self.handles.contains(x) => {
                    assert!(m.len.is_none());
                    let ty = xr_var_ty(self.api_aliases.as_ref(), m);
                    (
                        quote! { &'a #ty #type_args },
                        quote! { self.inner.#ident = value.as_raw(); },
                    )
                }
                x if self.base_headers.contains_key(x) => {
                    let ty = base_header_ty(x);
                    (
                        quote! { &'a #ty #type_args },
                        quote! { self.inner.#ident = value as *const _ as _; },
                    )
                }
                _ => {
                    if m.static_array_len.is_some() {
                        if m.ty != "char" {
                            return None;
                        }
                        (
                            quote! { &str },
                            quote! { place_cstr(&mut self.inner.#ident, value); },
                        )
                    } else if let Some(ref len) = m.len {
                        let mut inner = m.clone();
                        inner.ptr_depth -= 1;
                        assert_eq!(inner.ptr_depth, 0, "nested arrays are unimplemented");
                        let ty = xr_var_ty(self.api_aliases.as_ref(), &inner);
                        let len = xr_var_name(&len[0]);
                        (
                            quote! { &'a [#ty #type_args] },
                            quote! {
                                self.inner.#ident = value.as_ptr() as *const _ as _;
                                self.inner.#len = value.len() as u32;
                            },
                        )
                    } else if m.ptr_depth != 0 {
                        let mut inner = m.clone();
                        inner.ptr_depth -= 1;
                        let ty = xr_var_ty(self.api_aliases.as_ref(), &inner);
                        if m.is_const {
                            (
                                quote! { &'a #ty #type_args },
                                quote! { self.inner.#ident = value as *const _ as _; },
                            )
                        } else {
                            (
                                quote! { &'a mut #ty #type_args },
                                quote! { self.inner.#ident = value as *mut _ as _; },
                            )
                        }
                    } else if self.structs.contains_key(&m.ty) && !simple.contains(&m.ty[..]) {
                        let ty = xr_var_ty(self.api_aliases.as_ref(), m);
                        (
                            quote! { #ty #type_args },
                            quote! { self.inner.#ident = value.inner; },
                        )
                    } else {
                        (
                            xr_var_ty(self.api_aliases.as_ref(), m),
                            quote! { self.inner.#ident = value; },
                        )
                    }
                }
            };

            let fn_type_params = match &m.ty[..] {
                "XrAction" => quote! { <ATY: ActionTy> },
                _ => quote! {},
            };
            Some(quote! {
                #[inline]
                pub fn #ident#fn_type_params(mut self, value: #ty) -> Self {
                    #init
                    self
                }
            })
        });
        quote! {
            #(#setters)*
        }
    }

    fn generate_builder_inits(&self, s: &Struct) -> TokenStream {
        let inits = s.members.iter().filter_map(|m| {
            let ident = xr_var_name(&m.name);
            if m.name == "type" {
                let tag = xr_enum_value_name("XrStructureType", s.ty.as_ref().unwrap());
                return Some(quote! { #ident: sys::StructureType::#tag });
            }
            None
        });
        quote! {
            #(#inits,)*
        }
    }

    fn generate_builder(
        &self,
        meta: &HashMap<&str, StructMeta>,
        simple: &IndexSet<&str>,
        name: &str,
        s: &Struct,
    ) -> TokenStream {
        let setters = self.generate_setters(meta, simple, s);
        let ident = xr_ty_name(name);
        let (type_params, type_args, marker, marker_init) = meta.get(name).unwrap().type_params();
        let inits = self.generate_builder_inits(s);
        let conds = conditions(name, s.extension.as_ref().map(|x| &x[..]));
        let conds2 = conds.clone();
        quote! {
            #[derive(Copy, Clone)]
            #[repr(transparent)]
            #conds
            pub struct #ident #type_params {
                inner: sys::#ident,
                #marker
            }
            #conds2
            impl #type_params #ident #type_args {
                #[inline]
                pub fn new() -> Self {
                    Self {
                        inner: sys::#ident {
                            #inits
                            ..unsafe { mem::zeroed() }
                        },
                        #marker_init
                    }
                }

                /// Initialize with the supplied raw values
                ///
                /// # Safety
                ///
                /// The guarantees normally enforced by this builder (e.g. lifetimes) must be
                /// preserved.
                #[inline]
                pub unsafe fn from_raw(inner: sys::#ident) -> Self {
                    Self {
                        inner,
                        #marker_init
                    }
                }

                #[inline]
                pub fn into_raw(self) -> sys::#ident {
                    self.inner
                }

                #[inline]
                pub fn as_raw(&self) -> &sys::#ident {
                    &self.inner
                }

                #setters
            }

            impl #type_params Default for #ident #type_args {
                fn default() -> Self {
                    Self::new()
                }
            }
        }
    }

    fn generate_reader(&self, ident: &Ident, raw_ident: &Ident, s: &Struct) -> TokenStream {
        let lens = s
            .members
            .iter()
            .filter_map(|m| m.len.as_ref())
            .flat_map(|x| x.iter().filter(|&x| x != "null-terminated"))
            .map(|x| &x[..])
            .collect::<HashSet<&str>>();
        let readers = s.members.iter().filter_map(|m| {
            if m.name == "type" || m.name == "next" || lens.contains(&m.name[..]) {
                return None;
            }
            let ident = xr_var_name(&m.name);
            let (ty, value) = if m.ty == "XrBool32" {
                (quote! { bool }, quote! { (self.0).#ident.into() })
            } else if m.ty == "XrResult" {
                //  prevent name collision with std Result
                (quote! { sys::Result }, quote! { (self.0).#ident })
            } else if self.handles.contains(&m.ty) {
                let ty = xr_var_ty(self.api_aliases.as_ref(), m);
                (quote! { sys::#ty }, quote! { (self.0).#ident })
            } else if m.ty == "XrViveTrackerPathsHTCX" {
                (
                    quote! {
                        ViveTrackerPathsHTCX
                    },
                    quote! {
                        (*unsafe { self.0.#ident.as_ref() }.unwrap()).into()
                    },
                )
            } else if m.ty == "XrLocalizationMapML" {
                (
                    quote! {
                        LocalizationMapML
                    },
                    quote! {
                          LocalizationMapML::from_raw(self.0.#ident)
                    },
                )
            } else {
                (
                    xr_var_ty(self.api_aliases.as_ref(), m),
                    quote! { (self.0).#ident },
                )
            };
            Some(quote! {
                #[inline]
                pub fn #ident(self) -> #ty {
                    #value
                }
            })
        });

        let sys_raw_ident_str = format!("[sys::{}]", raw_ident);
        quote! {
            #[derive(Copy, Clone)]
            pub struct #ident<'a>(&'a sys::#raw_ident);

            impl<'a> #ident<'a> {
                #[inline]
                /// # Safety
                /// `inner` must be valid event data according to the OpenXR spec. Refer to
                #[doc = #sys_raw_ident_str]
                /// for more information.
                pub unsafe fn new(inner: &'a sys::#raw_ident) -> Self {
                    Self(inner)
                }

                #(#readers)*
            }
        }
    }

    /// Determine whether a struct is simple enough to be reexported directly from the high-level API
    fn is_simple_struct(&self, s: &Struct) -> bool {
        s.members.iter().all(|x| {
            x.ptr_depth == 0
                && x.static_array_len.is_none()
                && x.ty != "XrBool32"
                && self
                    .structs
                    .get(&x.ty)
                    .map_or(true, |x| self.is_simple_struct(x))
                && !self.handles.contains(&x.ty)
        })
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct StructMeta {
    has_unprintable: bool,
    has_pointer: bool,
    has_array: bool,
    has_graphics: bool,
    has_non_default: bool,
}

impl StructMeta {
    fn type_params(self) -> (TokenStream, TokenStream, TokenStream, TokenStream) {
        let mut params = Vec::new();
        let mut args = Vec::new();
        if self.has_pointer {
            params.push(quote! { 'a });
            args.push(quote! { 'a });
        }
        if self.has_graphics {
            params.push(quote! { G: Graphics });
            args.push(quote! { G });
        }
        let phantom = if self.has_pointer && self.has_graphics {
            quote! { &'a G }
        } else if self.has_pointer {
            quote! { &'a () }
        } else if self.has_graphics {
            quote! { G }
        } else {
            quote! {}
        };
        if params.is_empty() {
            (quote! {}, quote! {}, quote! {}, quote! {})
        } else {
            (
                quote! { <#(#params),*> },
                quote! { <#(#args),*> },
                quote! { _marker: PhantomData<#phantom> },
                quote! { _marker: PhantomData },
            )
        }
    }
}

impl std::ops::BitOrAssign for StructMeta {
    fn bitor_assign(&mut self, rhs: Self) {
        self.has_unprintable |= rhs.has_unprintable;
        self.has_pointer |= rhs.has_pointer;
        self.has_array |= rhs.has_array;
        self.has_graphics |= rhs.has_graphics;
        self.has_non_default |= self.has_non_default;
    }
}

struct Tag {
    extensions: Vec<Extension>,
}

struct Command {
    params: Vec<Member>,
    extension: Option<Rc<str>>,
}

#[derive(Debug)]
struct Enum<T> {
    comment: Option<String>,
    values: Vec<Constant<T>>,
}

impl<T> Enum<T> {
    pub fn new() -> Self {
        Self {
            comment: None,
            values: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Constant<T> {
    name: String,
    value: ConstantValue<T>,
    comment: Option<String>,
}

#[derive(Debug)]
enum ConstantValue<T> {
    Literal(T),
    Alias(String),
}

struct Struct {
    members: Vec<Member>,
    extension: Option<Rc<str>>,
    ty: Option<String>,
    mut_next: bool,
}

#[derive(Debug, Clone)]
struct Member {
    name: String,
    is_const: bool,
    ptr_depth: usize,
    ty: String,
    static_array_len: Option<String>,
    len: Option<Vec<String>>,
}

struct Extension {
    name: Rc<str>,
    version: u32,
    commands: Vec<String>,
}

fn attr<'a>(attrs: &'a [OwnedAttribute], name: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find(|x| x.name.local_name == name)
        .map(|x| &x.value[..])
}

fn xr_ty_name(raw: &str) -> Ident {
    let trimmed = raw
        .strip_prefix("Xr")
        .unwrap_or_else(|| panic!("not an XR type: {}", raw));
    Ident::new(trimmed, Span::call_site())
}

fn split_ty_ext(name: &str) -> (&str, &str) {
    let ext = name.rfind(|x: char| !x.is_uppercase()).unwrap() + 1;
    name.split_at(ext)
}

fn xr_enum_value_name(ty: &str, name: &str) -> Ident {
    let (ty, ext) = split_ty_ext(ty);
    let prefix_len = match ty {
        "XrStructureType" => "XR_TYPE_".len(),
        "XrPerfSettingsNotificationLevel" => "XR_PERF_SETTINGS_NOTIF_LEVEL_".len(),
        "XrResult" => "XR_".len(),
        "XrMarkerArucoDict" => "XR_MARKER_ARUCO_".len(),
        "XrMarkerAprilTagDict" => "XR_MARKER_APRIL_TAG_".len(),
        _ => ty.to_shouty_snake_case().len() + 1,
    };
    let end = if !ext.is_empty() {
        name.len() - ext.len() - 1
    } else {
        name.len()
    };
    Ident::new(&name[prefix_len..end], Span::call_site())
}

fn xr_bitmask_value_name(ty: &str, name: &str) -> Ident {
    let (ty, _) = split_ty_ext(ty);
    assert!(ty.ends_with("Flags"));
    let ty = &ty[0..ty.len() - "Flags".len()];
    let prefix_len = ty.to_shouty_snake_case().len() + 1;
    let end = name.rfind("_BIT").unwrap();

    if prefix_len == end + 1 {
        //  some BITs have no name, i.e. XR_PASSTHROUGH_CAPABILITY_BIT_FB
        //  in this case, return PASSTHROUGH_CAPABILITY
        return Ident::new(&name["XR_".len()..end], Span::call_site());
    }
    Ident::new(&name[prefix_len..end], Span::call_site())
}

fn xr_var_name(raw: &str) -> Ident {
    let raw = match raw {
        "type" => "ty",
        _ => raw,
    };
    Ident::new(&raw.to_snake_case(), Span::call_site())
}

fn xr_arg_ty(api_aliases: &[(String, String)], member: &Member) -> TokenStream {
    if member.static_array_len.is_some() {
        let mut clone = member.clone();
        clone.static_array_len = None;
        clone.ptr_depth += 1;
        xr_var_ty(api_aliases, &clone)
    } else {
        xr_var_ty(api_aliases, member)
    }
}

fn xr_var_ty(api_aliases: &[(String, String)], member: &Member) -> TokenStream {
    let ty = if member.ty.starts_with("Xr") {
        let ident = xr_ty_name(&member.ty);
        quote! { #ident }
    } else if member.ty.starts_with("PFN_") {
        if member.ty.starts_with("PFN_vk") {
            let ident = Ident::new(
                &format!("Vk{}", &member.ty["PFN_vk".len()..]),
                Span::call_site(),
            );
            quote! { Option<#ident> }
        } else {
            let ident = xr_command_name(&member.ty["PFN_".len()..]);
            quote! { Option<pfn::#ident> }
        }
    } else {
        let ty = match &member.ty[..] {
            "uint64_t" => "u64",
            "uint32_t" => "u32",
            "uint16_t" => "u16",
            "uint8_t" => "u8",
            "size_t" => "usize",
            "char" => "c_char",
            "int64_t" => "i64",
            "int32_t" => "i32",
            "int16_t" => "i16",
            "int8_t" => "i8",
            "float" => "f32",
            "double" => "f64",
            "void" => "c_void",
            x => x,
        };
        let ident = Ident::new(ty, Span::call_site());
        quote! { #ident }
    };
    let mut ty = if let Some(ref len) = member.static_array_len {
        // If the len is a constant, we see if it is an aliased constant. Since we do not emit
        // aliased constant declarations, we must replace the alias with the original constant name
        // that we do emit.
        let len = match api_aliases.iter().find(|(alias, _)| alias == len) {
            Some((_, og)) => og,
            _ => len,
        };
        if let Ok(len) = len.parse::<usize>() {
            quote! { [#ty; #len] }
        } else {
            assert!(len.starts_with("XR_"));
            let len = Ident::new(&len[3..], Span::call_site());
            quote! { [#ty; #len] }
        }
    } else {
        quote! { #ty }
    };
    for _ in 0..member.ptr_depth {
        ty = if member.is_const {
            quote! { *const #ty }
        } else {
            quote! { *mut #ty }
        }
    }
    ty
}

fn xr_command_name(raw: &str) -> Ident {
    let start = if raw.starts_with("xr") { 2 } else { 0 };
    Ident::new(&raw[start..], Span::call_site())
}

fn conditions(name: &str, ext: Option<&str>) -> TokenStream {
    let name = name.to_lowercase();
    let mut conditions = Vec::new();
    if name.contains("win32")
        || name.contains("d3d")
        || ext == Some("XR_MSFT_holographic_window_attachment")
        || ext == Some("XR_MSFT_perception_anchor_interop")
    {
        conditions.push(quote! { windows });
    }
    if name.contains("android") {
        conditions.push(quote! { target_os = "android" });
    }
    match conditions.len() {
        0 => quote! {},
        1 => quote! { #[cfg(#(#conditions)*)] },
        _ => quote! { #[cfg(all(#(#conditions),*))] },
    }
}

fn split_ext_tag(name: &str) -> (&str, &str) {
    assert_eq!(&name[..3], "XR_");
    let name = &name[3..];
    let tag_end = name.find('_').unwrap();
    let (tag, tail) = name.split_at(tag_end);
    (tag, &tail[1..])
}

fn c_name(name: &str) -> LitByteStr {
    let mut name = name.as_bytes().to_vec();
    name.push(0);
    LitByteStr::new(&name, Span::call_site())
}

fn tidy_comment(s: &str) -> Option<String> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let strip_macros = Regex::new(r"\S+:(\S+)").unwrap();
    let strip_links = Regex::new(r"<<\S+, ?([^>]+)>>").unwrap();

    let s = strip_macros.replace_all(s, "$1");
    Some(strip_links.replace_all(&s, "$1").into())
}

fn base_header_ty(base_name: &str) -> Ident {
    let base_header_pos = base_name.find("BaseHeader").unwrap();
    assert!(base_name.starts_with("Xr"));
    Ident::new(
        &base_name[2..base_header_pos + "Base".len()],
        Span::call_site(),
    )
}
