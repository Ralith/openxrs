#![recursion_limit = "256"]

use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
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
    let mut source = File::open(args.next().map(PathBuf::from).unwrap_or_else(|| {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../sys/OpenXR-SDK-Source/specification/registry/xr.xml")
    }))
    .expect("failed to open registry XML file");
    // Hack around leading garbage in 1.0 revision of XML
    source.seek(SeekFrom::Start(3)).unwrap();

    let mut parser = Parser::new(source);
    parser.parse();
    let mut sys_out = File::create("../sys/src/generated.rs").unwrap();
    let mut hl_out = File::create("../openxr/src/generated.rs").unwrap();
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
    api_constants: Vec<(String, usize)>,
    commands: IndexMap<String, Command>,
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
            api_constants: Vec::new(),
            commands: IndexMap::new(),
            extensions: IndexMap::new(),
            disabled_exts: HashSet::new(),
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
                            self.commands.get_mut(cmd).unwrap().extension = Some(ext_name.clone());
                            commands.push(cmd.into());
                        }
                        "enum" => {
                            let name = attr(&attributes, "name").unwrap();
                            if let Some(extends) = attr(&attributes, "extends") {
                                const EXT_BASE: i32 = 1_000_000_000;
                                const EXT_BLOCK_SIZE: i32 = 1000;
                                let offset =
                                    attr(&attributes, "offset").unwrap().parse::<i32>().unwrap();
                                let sign = if attr(&attributes, "dir").map_or(false, |x| x == "-") {
                                    -1
                                } else {
                                    1
                                };
                                let value =
                                    sign * (EXT_BASE + (ext_number - 1) * EXT_BLOCK_SIZE + offset);
                                self.enums.get_mut(extends).unwrap().values.push(Constant {
                                    name: name.into(),
                                    value,
                                    comment: attr(&attributes, "comment").map(|x| tidy_comment(x.trim())),
                                });
                            } else if name.ends_with("SPEC_VERSION") {
                                let value = attr(&attributes, "value").unwrap();
                                ext_version = Some(value.parse().unwrap());
                            } else if name.ends_with("EXTENSION_NAME") {
                                let value = attr(&attributes, "value").unwrap();
                                assert_eq!(&ext_name[..], &value[1..value.len() - 1]);
                            } else {
                                eprintln!("unrecognized extension constant {}", name);
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
            self.disabled_exts.insert(ext_name.into());
        } else {
            let (tag, _) = split_ext_tag(&ext_name);
            self.extensions
                .get_mut(tag)
                .unwrap()
                .extensions
                .push(Extension {
                    name: ext_name,
                    version: ext_version.unwrap(),
                    commands,
                });
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

    fn parse_command(&mut self, _attrs: &[OwnedAttribute]) {
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
        self.commands.insert(
            command_name.unwrap(),
            Command {
                params,
                extension: None,
            },
        );
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
                    if define_ty.is_some() {
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
        match define_name.as_ref().map(|x| &x[..]) {
            Some("XR_CURRENT_API_VERSION") => {
                let version = define_val.unwrap();
                assert!(version.starts_with("("));
                assert!(version.ends_with(")"));
                let version = &version[1..version.len() - 1];
                let mut iter = version.split(", ").map(|x| x.parse::<u64>().unwrap());
                self.api_version = Some((
                    iter.next().unwrap() as u16,
                    iter.next().unwrap() as u16,
                    iter.next().unwrap() as u32,
                ));
            }
            _ => {}
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
        let parent: Option<String> = attr(attrs, "parentstruct").map(|x| x.into());
        if let Some(ref parent) = parent {
            self.base_headers
                .entry(parent.clone())
                .or_insert_with(Vec::new)
                .push(struct_name.into());
        }
        self.structs.insert(
            struct_name.into(),
            Struct {
                members,
                ty,
                extension: None,
                parent,
                mut_next,
            },
        );
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
                    } else if ch.starts_with("[") && ch.len() > 1 {
                        assert!(ch.ends_with("]"));
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
            len: attr(attrs, "len").map(|x| x.split(",").map(|x| x.into()).collect()),
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
        if let Some(comment) = attr(attrs, "comment") {
            if let Some(item) = self.enums.get_mut(name) {
                item.comment = Some(comment.into());
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
                                value: value.parse().unwrap(),
                                comment: comment.map(|x| tidy_comment(x)),
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
                                value: bitpos.parse().unwrap(),
                                comment: comment.map(|x| tidy_comment(x)),
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

    fn generate_sys(&self) -> TokenStream {
        let consts = self.api_constants.iter().map(|(name, value)| {
            let ident = Ident::new(&name[3..], Span::call_site());
            quote! {
                pub const #ident: usize = #value;
            }
        });

        let enums = self.enums.iter().map(|(name, e)| {
            let ident = xr_ty_name(name);
            let doc = if let Some(comment) = e.comment.as_ref() {
                quote! {#[doc = #comment]}
            } else {
                quote! {}
            };
            let values = e.values.iter().map(|v| {
                let value_name = xr_enum_value_name(&name, &v.name);
                let value = v.value;
                let doc = if let Some(comment) = v.comment.as_ref() {
                    quote! {#[doc = #comment]}
                } else {
                    quote! {}
                };
                quote! {
                    #doc
                    pub const #value_name: #ident = #ident(#value);
                }
            });
            let debug_cases = e.values.iter().map(|v| {
                let ident = xr_enum_value_name(&name, &v.name);
                let name = ident.to_string();
                quote! {
                    Self::#ident => Some(#name)
                }
            });
            let result_extras = if name == "XrResult" {
                let cases = e.values.iter().map(|v| {
                    let ident = xr_enum_value_name(&name, &v.name);
                    let reason = v.comment.as_ref().map_or_else(
                        || ident.to_string(),
                        |x| {
                            let mut reason = x.to_string();
                            reason.get_mut(0..1).unwrap().make_ascii_lowercase();
                            assert!(reason.ends_with("."));
                            reason.truncate(reason.len() - 1);
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
                #doc
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
                quote! {#[doc = #comment]}
            } else {
                quote! {}
            };
            let values = bitmask.values.iter().map(|v| {
                let value_name = xr_bitmask_value_name(&name, &v.name);
                let value = v.value;
                let doc = if let Some(comment) = v.comment.as_ref() {
                    quote! {#[doc = #comment]}
                } else {
                    quote! {}
                };
                quote! {
                    #doc
                    pub const #value_name: #ident = #ident(1 << #value);
                }
            });
            quote! {
                #doc
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
            let ident = xr_ty_name(&name);
            quote! {
                #[repr(transparent)]
                #[derive(Debug, Copy, Clone, Eq, PartialEq)]
                pub struct #ident(u64);
                handle!(#ident);
            }
        });

        let structs = self.structs.iter().map(|(name, s)| {
            let ident = xr_ty_name(&name);
            let members = s.members.iter().map(|m| {
                let ident = xr_var_name(&m.name);
                let ty = xr_var_ty(&m);
                quote! {
                    pub #ident: #ty,
                }
            });
            let ext_note = if let Some(ref ext) = s.extension {
                if self.disabled_exts.contains(ext) {
                    return quote! {};
                }
                let doc = format!("From {}", ext);
                quote! { #[doc = #doc] }
            } else {
                quote! {}
            };
            let conditions = conditions(&name);
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
            quote! {
                #[repr(C)]
                #[derive(Copy, Clone)]
                #ext_note
                #conditions
                pub struct #ident {
                    #(#members)*
                }
                #ty
            }
        });

        let (pfns, protos) = self
            .commands
            .iter()
            .map(|(name, command)| {
                let ident = xr_command_name(&name);
                let params = command.params.iter().map(|param| {
                    let ident = xr_var_name(&param.name);
                    let ty = xr_arg_ty(&param);
                    quote! {
                        #ident: #ty
                    }
                });
                let ext_note = if let Some(ref ext) = command.extension {
                    if self.disabled_exts.contains(ext) {
                        return (quote! {}, quote! {});
                    }
                    let doc = format!("From {}", ext);
                    quote! { #[doc = #doc] }
                } else {
                    quote! {}
                };
                let conditions = conditions(&name);
                let conditions2 = conditions.clone();
                let params2 = params.clone();
                let pfn_def = quote! {
                    #conditions
                    #ext_note
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
                let conds = conditions(&ext.name);
                // TODO: &'static CStr name
                quote! {
                    #conds
                    pub const #version_ident: u32 = #version_lit;
                    #conds
                    pub const #name_ident: &'static [u8] = #name_lit;
                }
            })
        });

        let (major, minor, patch) = self.api_version.unwrap();

        quote! {
            //! Automatically generated code; do not edit!

            #![allow(non_upper_case_globals)]
            use std::fmt;
            use std::mem::MaybeUninit;
            use std::os::raw::{c_void, c_char};
            use libc::timespec;

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

            #[cfg(feature = "prototypes")]
            extern "system" {
                #(#protos)*
            }
        }
    }

    /// Generate high-level code
    fn generate_hl(&self) -> TokenStream {
        let (instance_pfn_fields, instance_pfn_inits) = self.commands.iter().map(|(name, command)| {
            if command.extension.is_some() {
                return (quote! {}, quote! {});
            }
            let pfn_ident = xr_command_name(&name);
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
                    &format!("{}{}", ext_name.to_camel_case(), tag_name),
                    Span::call_site(),
                );
                let conds = conditions(&ext.name);
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
                    { if self.#field_ident { out.push(raw::#ty_ident::NAME.as_ptr() as *const _ as _); } }
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
            let raw_ident = xr_ty_name(&raw_name);
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
            struct_meta.insert(name, self.compute_meta(&s));
        }

        let polymorphic_builders = self.base_headers.iter().filter_map(|(name, children)| {
            if name == "XrSwapchainImageBaseHeader" || name == "XrEventDataBaseHeader" {
                return None;
            }
            Some(self.generate_polymorphic_builders(&struct_meta, &simple_structs, name, children))
        });

        let whitelist = [
            "XrCompositionLayerProjectionView",
            "XrSwapchainSubImage",
            "XrFrameWaitInfo",
            "XrFrameBeginInfo",
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

            use std::os::raw::c_char;
            pub use sys::{#(#reexports),*};

            use crate::*;

            #[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
            pub struct ExtensionSet {
                #(#ext_set_fields)*
            }

            impl ExtensionSet {
                pub(crate) fn from_properties(properties: &[sys::ExtensionProperties]) -> Self {
                    let mut out = Self::default();
                    for ext in properties {
                        match crate::fixed_str_bytes(&ext.extension_name) {
                            #(#ext_set_inits)*
                            _ => {}
                        }
                    }
                    out
                }

                pub(crate) fn names(&self) -> Vec<*const c_char> {
                    let mut out = Vec::new();
                    #(#ext_set_names)*
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
                pub unsafe fn from_raw(raw: *const sys::EventDataBuffer) -> Option<Self> {
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

    fn compute_meta(&self, s: &Struct) -> StructMeta {
        let mut out = StructMeta::default();
        for member in &s.members {
            out.has_pointer |= member.ptr_depth != 0 || self.handles.contains(&member.ty);
            out.has_graphics |= member.ty == "XrSession" || member.ty == "XrSwapchain";
            if let Some(x) = self.structs.get(&member.ty) {
                out |= self.compute_meta(x);
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
        assert!(base_name.starts_with("Xr") && base_name.ends_with("BaseHeader"));
        let base_ident = Ident::new(
            &base_name[2..base_name.len() - "Header".len()],
            Span::call_site(),
        );
        let mut base_meta = StructMeta::default();
        for name in children {
            base_meta |= *meta.get(&name[..]).unwrap();
        }

        let (type_params, type_args, marker, marker_init) = base_meta.type_params();
        let builders = children.iter().map(|name| {
            let ident = xr_ty_name(name);
            let s = self.structs.get(name).unwrap();
            let inits = self.generate_builder_inits(s);
            let setters = self.generate_setters(meta, simple, s);
            quote! {
                #[derive(Copy, Clone)]
                #[repr(transparent)]
                pub struct #ident #type_params {
                    inner: sys::#ident,
                    #marker
                }
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

                    #(#setters)*
                }
                impl #type_params Deref for #ident #type_args {
                    type Target = #base_ident #type_args;

                    #[inline]
                    fn deref(&self) -> &Self::Target {
                        unsafe { mem::transmute(&self.inner) }
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
            } else {
                quote! {}
            };
            let (ty, init) = match &m.ty[..] {
                "XrBool32" => (
                    quote! { bool },
                    quote! { self.inner.#ident = value.into(); },
                ),
                x if self.handles.contains(x) => {
                    assert!(m.len.is_none());
                    let ty = xr_var_ty(&m);
                    (
                        quote! { &'a #ty #type_args },
                        quote! { self.inner.#ident = value.as_raw(); },
                    )
                }
                _ => {
                    if let Some(_) = m.static_array_len {
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
                        let ty = xr_var_ty(&inner);
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
                        let ty = xr_var_ty(&inner);
                        (
                            quote! { &'a #ty #type_args },
                            quote! { self.inner.#ident = value as *const _ as _; },
                        )
                    } else if self.structs.contains_key(&m.ty) && !simple.contains(&m.ty[..]) {
                        let ty = xr_var_ty(&m);
                        (
                            quote! { #ty #type_args },
                            quote! { self.inner.#ident = value.inner; },
                        )
                    } else {
                        (xr_var_ty(&m), quote! { self.inner.#ident = value; })
                    }
                }
            };
            Some(quote! {
                #[inline]
                pub fn #ident(mut self, value: #ty) -> Self {
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
        let conds = conditions(&name);
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

                #(#setters)*
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
            } else if self.handles.contains(&m.ty) {
                let ty = xr_var_ty(&m);
                (quote! { sys::#ty }, quote! { (self.0).#ident })
            } else {
                (xr_var_ty(&m), quote! { (self.0).#ident })
            };
            Some(quote! {
                #[inline]
                pub fn #ident(&self) -> #ty {
                    #value
                }
            })
        });

        quote! {
            #[derive(Copy, Clone)]
            pub struct #ident<'a>(&'a sys::#raw_ident);

            impl<'a> #ident<'a> {
                #[inline]
                pub fn new(inner: &'a sys::#raw_ident) -> Self {
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

#[derive(Debug, Copy, Clone)]
struct StructMeta {
    has_pointer: bool,
    has_graphics: bool,
}

impl StructMeta {
    fn type_params(&self) -> (TokenStream, TokenStream, TokenStream, TokenStream) {
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

impl Default for StructMeta {
    fn default() -> Self {
        Self {
            has_pointer: false,
            has_graphics: false,
        }
    }
}

impl std::ops::BitOrAssign for StructMeta {
    fn bitor_assign(&mut self, rhs: Self) {
        self.has_pointer |= rhs.has_pointer;
        self.has_graphics |= rhs.has_graphics;
    }
}

struct Tag {
    extensions: Vec<Extension>,
}

struct Command {
    params: Vec<Member>,
    extension: Option<Rc<str>>,
}

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

struct Constant<T> {
    name: String,
    value: T,
    comment: Option<String>,
}

struct Struct {
    members: Vec<Member>,
    extension: Option<Rc<str>>,
    ty: Option<String>,
    parent: Option<String>,
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
    let trimmed = if raw.starts_with("Xr") {
        &raw[2..]
    } else {
        panic!("not an XR type: {}", raw);
    };
    Ident::new(trimmed, Span::call_site())
}

fn split_ty_ext(name: &str) -> (&str, &str) {
    let ext = name.rfind(|x: char| x.is_lowercase()).unwrap() + 1;
    name.split_at(ext)
}

fn xr_enum_value_name(ty: &str, name: &str) -> Ident {
    let (ty, ext) = split_ty_ext(ty);
    let prefix_len = match ty {
        "XrStructureType" => "XR_TYPE_".len(),
        "XrPerfSettingsNotificationLevel" => "XR_PERF_SETTINGS_NOTIF_LEVEL_".len(),
        "XrResult" => "XR_".len(),
        _ => ty.to_shouty_snake_case().len() + 1,
    };
    let end = if ext.len() != 0 {
        name.len() - ext.len() - 1
    } else {
        name.len()
    };
    Ident::new(&name[prefix_len..end], Span::call_site())
}

fn xr_bitmask_value_name(ty: &str, name: &str) -> Ident {
    let (ty, ext) = split_ty_ext(ty);
    assert!(ty.ends_with("Flags"));
    let ty = &ty[0..ty.len() - "Flags".len()];
    let prefix_len = ty.to_shouty_snake_case().len() + 1;
    let end = if ext.len() != 0 {
        name.len() - ext.len() - 1
    } else {
        name.len()
    };
    assert!(name[..end].ends_with("_BIT"));
    let end = end - "_BIT".len();
    Ident::new(&name[prefix_len..end], Span::call_site())
}

fn xr_var_name(raw: &str) -> Ident {
    let raw = match raw {
        "type" => "ty",
        _ => raw,
    };
    Ident::new(&raw.to_snake_case(), Span::call_site())
}

fn xr_arg_ty(member: &Member) -> TokenStream {
    if member.static_array_len.is_some() {
        let mut clone = member.clone();
        clone.static_array_len = None;
        clone.ptr_depth += 1;
        xr_var_ty(&clone)
    } else {
        xr_var_ty(member)
    }
}

fn xr_var_ty(member: &Member) -> TokenStream {
    let ty = if member.ty.starts_with("Xr") {
        let ident = xr_ty_name(&member.ty);
        quote! { #ident }
    } else if member.ty.starts_with("PFN_") {
        let ident = xr_command_name(&member.ty["PFN_".len()..]);
        quote! { Option<pfn::#ident> }
    } else {
        let ty = match &member.ty[..] {
            "uint64_t" => "u64",
            "uint32_t" => "u32",
            "uint16_t" => "u16",
            "uint8_t" => "u8",
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
        let ident = Ident::new(&ty, Span::call_site());
        quote! { #ident }
    };
    let mut ty = if let Some(ref len) = member.static_array_len {
        if let Ok(len) = len.parse::<usize>() {
            quote! { [#ty; #len] }
        } else {
            assert!(len.starts_with("XR_MAX_"));
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
    assert!(raw.starts_with("xr"));
    Ident::new(&raw[2..], Span::call_site())
}

fn conditions(name: &str) -> TokenStream {
    let name = name.to_lowercase();
    let mut conditions = Vec::new();
    if name.contains("win32") || name.contains("d3d") {
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
    let tag_end = name.find("_").unwrap();
    let (tag, tail) = name.split_at(tag_end);
    (tag, &tail[1..])
}

fn c_name(name: &str) -> LitByteStr {
    let mut name = name.as_bytes().to_vec();
    name.push(0);
    LitByteStr::new(&name, Span::call_site())
}

fn tidy_comment(s: &str) -> String {
    let strip_macros = Regex::new(r"\S+:(\S+)").unwrap();
    let strip_links = Regex::new(r"<<\S+, ?([^>]+)>>").unwrap();

    let s = s.trim();
    let s = strip_macros.replace_all(s, "$1");
    strip_links.replace_all(&s, "$1").into()
}
