#![recursion_limit = "128"]

use std::{
    collections::{BTreeMap, HashMap},
    env,
    fs::File,
    io::{Seek, SeekFrom},
    rc::Rc,
};

use heck::{ShoutySnakeCase, SnakeCase};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitByteStr};
use xml::{
    attribute::OwnedAttribute,
    reader::{EventReader, ParserConfig, XmlEvent},
};

fn main() {
    let mut args = env::args_os();
    args.next().unwrap();
    let mut source = File::open(args.next().expect("missing registry XML path"))
        .expect("failed to open registry XML file");
    source.seek(SeekFrom::Start(3)).unwrap(); // Skip byte order marker

    let mut parser = Parser::new(source);
    parser.parse();
    println!("{}", parser.generate());
}

struct Parser {
    reader: EventReader<File>,
    enums: BTreeMap<String, Enum>,
    bitmasks: BTreeMap<String, Enum>,
    /// *FlagBits -> *Flags
    bitvalues: HashMap<String, String>,
    handles: Vec<String>,
    structs: BTreeMap<String, Struct>,
    api_constants: Vec<(String, usize)>,
    commands: BTreeMap<String, Command>,
    extensions: Vec<Extension>,
}

impl Parser {
    fn new(source: File) -> Self {
        Self {
            reader: ParserConfig::new()
                .trim_whitespace(true)
                .ignore_comments(true)
                .create_reader(source),
            enums: BTreeMap::new(),
            bitmasks: BTreeMap::new(),
            bitvalues: HashMap::new(),
            handles: Vec::new(),
            structs: BTreeMap::new(),
            api_constants: Vec::new(),
            commands: BTreeMap::new(),
            extensions: Vec::new(),
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
                    "comment" | "tags" => self.finish_element(),
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
                                self.enums.get_mut(extends).unwrap().values.push(EnumValue {
                                    name: name.into(),
                                    value,
                                    comment: attr(&attributes, "comment").map(|x| x.into()),
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
        self.extensions.push(Extension {
            name: ext_name,
            version: ext_version.unwrap(),
        });
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
                StartElement { name, .. } => match &name.local_name[..] {
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
                        params.push(self.parse_var("param"));
                    }
                    _ => {
                        eprintln!("unimplemented command element: {}", name.local_name);
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
            Some("include") | Some("basetype") | Some("define") | Some("funcpointer") | None => {
                // Intentionally skipped
                self.finish_element();
            }
            Some(other) => {
                eprintln!("unimplemented type category: {}", other);
                self.finish_element();
            }
        }
    }

    fn parse_struct(&mut self, attrs: &[OwnedAttribute]) {
        let name = attr(attrs, "name").unwrap();
        let mut members = Vec::new();
        loop {
            use XmlEvent::*;
            match self.reader.next().expect("failed to parse XML") {
                StartElement { name, .. } => match &name.local_name[..] {
                    "member" => {
                        members.push(self.parse_var("member"));
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
        self.structs.insert(name.into(), Struct { members, extension: None });
    }

    fn parse_var(&mut self, elt_name: &'static str) -> Member {
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
                            self.handles.push(name);
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
                            item.values.push(EnumValue {
                                name: name.into(),
                                value: value.parse().unwrap(),
                                comment: comment.map(|x| x.into()),
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
                            bitmask.values.push(EnumValue {
                                name: name.into(),
                                value: bitpos.parse().unwrap(),
                                comment: comment.map(|x| x.into()),
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
        if let XmlEvent::Characters(x) = self.reader.next().expect("parse error") {
            x
        } else {
            panic!("expected characters")
        }
    }

    fn generate(&self) -> TokenStream {
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
            let display = if name == "XrResult" {
                let cases = e.values.iter().map(|v| {
                    let ident = xr_enum_value_name(&name, &v.name);
                    let reason = v.comment.as_ref().map_or_else(
                        || ident.to_string(),
                        |x| {
                            let mut reason = x.trim().to_lowercase();
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
                #display
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
                pub struct #ident(i32);
                impl #ident {
                    #(#values)*

                    pub fn from_raw(x: i32) -> Self { Self(x) }
                    pub fn into_raw(self) -> i32 { self.0 }
                }
            }
        });

        let handles = self.handles.iter().map(|name| {
            let ident = xr_ty_name(&name);
            quote! {
                #[repr(transparent)]
                #[derive(Debug, Copy, Clone, Eq, PartialEq)]
                pub struct #ident(u64);
                impl #ident {
                    pub fn from_raw(x: u64) -> Self { Self(x) }
                    pub fn into_raw(self) -> u64 { self.0 }
                }
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
                let doc = format!("From {}", ext);
                quote! { #[doc = #doc] }
            } else {
                quote! {}
            };
            let mut conditions = Vec::new();
            if name.contains("Android") {
                conditions.push(quote! { target_os = "android" });
            }
            if name.contains("Vulkan") {
                conditions.push(quote! { feature = "ash" });
            }
            if name.contains("D3D") {
                conditions.push(quote! { feature = "d3d" });
            }
            if name.contains("OpenGLES") {
                conditions.push(quote! { feature = "opengles" });
            } else if name.contains("OpenGL") {
                conditions.push(quote! { feature = "opengl" });
            }
            if name.contains("OpenGLXcb") {
                conditions.push(quote! { feature = "xcb" });
            }
            if name.contains("OpenGLXlib") {
                conditions.push(quote! { any(feature = "x11", feature = "x11-dl") });
            }
            let conditions = if conditions.is_empty() {
                quote! {}
            } else {
                quote! { #[cfg(all(#(#conditions),*))] }
            };
            quote! {
                #[repr(C)]
                #[derive(Copy, Clone)]
                #ext_note
                #conditions
                pub struct #ident {
                    #(#members)*
                }
            }
        });

        let exts = self.extensions.iter().map(|ext| {
            assert!(ext.name.starts_with("XR_"));
            let name_ident = Ident::new(&format!("{}_EXTENSION_NAME", ext.name[3..].to_uppercase()), Span::call_site());
            let mut name_lit = ext.name.as_bytes().to_vec();
            name_lit.push(0);
            let name_lit = LitByteStr::new(&name_lit, Span::call_site());
            let version_ident = Ident::new(&format!("{}_SPEC_VERSION", &ext.name[3..].to_uppercase()), Span::call_site());
            let version_lit = ext.version;
            // TODO: &'static CStr
            quote! {
                pub const #version_ident: u32 = #version_lit;
                pub const #name_ident: &'static [u8] = #name_lit;
            }
        });

        let (pfns, protos) = self.commands.iter().map(|(name, command)| {
            let ident = xr_command_name(&name);
            let params = command.params.iter().map(|param| {
                let ident = xr_var_name(&param.name);
                let ty = xr_var_ty(&param);
                quote! {
                    #ident: #ty
                }
            });
            let ext_note = if let Some(ref ext) = command.extension {
                let doc = format!("From {}", ext);
                quote! { #[doc = #doc] }
            } else {
                quote! {}
            };
            let mut conditions = Vec::new();
            if name.contains("Win32") {
                conditions.push(quote! { target_os = "windows" });
            }
            if name.contains("Android") {
                conditions.push(quote! { target_os = "android" });
            }
            if name.contains("Vulkan") {
                conditions.push(quote! { feature = "ash" });
            }
            if name.contains("D3D") {
                conditions.push(quote! { feature = "d3d" });
            }
            if name.contains("OpenGLES") {
                conditions.push(quote! { feature = "opengles" });
            } else if name.contains("OpenGL") {
                conditions.push(quote! { feature = "opengl" });
            }
            if name.contains("Timespec") {
                conditions.push(quote! { feature = "libc" });
            }
            let conditions = if conditions.is_empty() {
                quote! {}
            } else {
                quote! {
                    #[cfg(all(#(#conditions),*))]
                }
            };
            let conditions2 = conditions.clone();
            let params1 = params.clone();
            let params2 = params;
            let pfn_def = quote! {
                #conditions
                #ext_note
                pub type #ident = unsafe extern "C" fn(#(#params1),*) -> Result;
            };
            let raw_ident = Ident::new(&name, Span::call_site());
            let proto = if command.extension.is_some() {
                quote! {}
            } else {
                quote! {
                    #conditions2
                    fn #raw_ident(#(#params2),*) -> Result;
                }
            };
            (pfn_def, proto)
        }).unzip::<_, _, Vec<_>, Vec<_>>();

        quote! {
            use std::fmt;
            use std::os::raw::c_void;

            use crate::support::*;
            use crate::*;
            #(#consts)*
            #(#enums)*
            #(#bitmasks)*
            #(#handles)*
            #(#structs)*
            #(#exts)*

            /// Function pointer prototypes
            pub mod pfn {
                use super::*;

                pub type VoidFunction = Option<unsafe extern "system" fn()>;
                pub type DebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
                    DebugUtilsMessageSeverityFlagsEXT,
                    DebugUtilsMessageTypeFlagsEXT,
                    *const DebugUtilsMessengerCallbackDataEXT,
                    *mut c_void,
                ) -> Bool32;

                #(#pfns)*
            }

            #[cfg(feature = "prototypes")]
            extern "C" {
                #(#protos)*
            }
        }
    }
}

struct Command {
    params: Vec<Member>,
    extension: Option<Rc<str>>,
}

struct Enum {
    comment: Option<String>,
    values: Vec<EnumValue>,
}

impl Enum {
    pub fn new() -> Self {
        Self {
            comment: None,
            values: Vec::new(),
        }
    }
}

struct EnumValue {
    name: String,
    value: i32,
    comment: Option<String>,
}

struct Struct {
    members: Vec<Member>,
    extension: Option<Rc<str>>,
}

struct Member {
    name: String,
    is_const: bool,
    ptr_depth: usize,
    ty: String,
    static_array_len: Option<String>,
}

struct Extension {
    name: Rc<str>,
    version: u32,
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
    let (prefix_len, extra_prefix) = match ty {
        "XrStructureType" => ("XR_TYPE_".len(), None),
        "XrPerfSettingsNotificationLevel" => ("XR_PERF_SETTINGS_NOTIF_LEVEL_".len(), None),
        "XrResult" => ("XR_".len(), None),
        "XrActionType" => {
            if name.starts_with("XR_OUTPUT") {
                ("XR_OUTPUT_ACTION_TYPE_".len(), Some("OUTPUT"))
            } else {
                ("XR_INPUT_ACTION_TYPE_".len(), Some("INPUT"))
            }
        }
        _ => (ty.to_shouty_snake_case().len() + 1, None),
    };
    let end = if ext.len() != 0 {
        name.len() - ext.len() - 1
    } else {
        name.len()
    };
    let name = &name[prefix_len..end];
    let name = if let Some(x) = extra_prefix {
        format!("{}_{}", x, name)
    } else {
        name.into()
    };
    Ident::new(&name, Span::call_site())
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

fn xr_var_ty(member: &Member) -> TokenStream {
    let ty = if member.ty.starts_with("Xr") {
        let ident = xr_ty_name(&member.ty);
        quote! { #ident }
    } else if member.ty.starts_with("PFN_") {
        let ident = xr_command_name(&member.ty["PFN_".len()..]);
        quote! { pfn::#ident }
    } else if member.ty.starts_with("Vk") {
        let ident = Ident::new(&member.ty[2..], Span::call_site());
        quote! { vk::#ident }
    } else {
        let ty = match &member.ty[..] {
            "uint64_t" => "u64",
            "uint32_t" => "u32",
            "uint16_t" => "u16",
            "uint8_t" => "u8",
            "char" => "u8",
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
        assert!(len.starts_with("XR_MAX_"));
        let len = Ident::new(&len[3..], Span::call_site());
        quote! { [#ty; #len] }
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
