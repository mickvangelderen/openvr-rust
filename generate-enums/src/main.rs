#![feature(custom_attribute)]

mod extra;
mod types;

use self::types::*;

use inflector::cases::classcase::to_class_case;
use regex::Regex;
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let input_file_path = Path::new("input/openvr_api.json");
    let output_file_path = Path::new("output/enums.rs");
    let input_file = File::open(&input_file_path).expect("Failed to open file.");
    let data: Root = serde_json::from_reader(input_file).expect("Failed to parse JSON.");
    let extra_map = extra::create_extra();

    {
        let mut output_file = File::create(&output_file_path).expect("Failed to open file.");
        let out = &mut output_file;

        let enum_namespace_regex = Regex::new(r"^vr::(\w*)$").unwrap();
        let enum_name_regex = Regex::new(r"^vr::E?(?:VR)?(\w+)$").unwrap();
        let vari_name_regex = Regex::new(r"^[^_]*_(.*)$").unwrap();

        writeln!(
            out,
            r#"use openvr_sys as sys;
use std::error;
use std::fmt;

pub trait Enum: Sized {{
    type Raw: fmt::Display;

    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>>;
    fn into_unchecked(self) -> Unchecked<Self>;
}}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Invalid<E: Enum>(pub(crate) E::Raw);

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Unchecked<E: Enum>(pub(crate) E::Raw);
"#
        )
        .unwrap();

        for enum_ in data.enums.iter() {
            let enum_sys_id = if let Some(caps) = enum_namespace_regex.captures(&enum_.id) {
                caps.get(1).unwrap().as_str()
            } else {
                panic!("Failed to convert {:?}", enum_);
            };

            let extra = extra_map.get(&enum_sys_id).unwrap();

            let enum_id = if let Some(caps) = enum_name_regex.captures(&enum_.id) {
                to_class_case(caps.get(1).unwrap().as_str())
            } else {
                panic!("Failed to convert {:?}", enum_);
            };

            struct Vari {
                sys_id: String,
                vari_id: String,
                lit: String,
            }

            // We want to do some automated renaming and use the new
            // names multiple times. So we create a new version of the
            // enum variants.
            let variants: Vec<Vari> = enum_
                .variants
                .iter()
                .filter_map(|vari| {
                    if extra.vari_blacklist.iter().any(|&id| id == &vari.id) {
                        None
                    } else {
                        let vari_id =
                            to_class_case(if let Some(caps) = vari_name_regex.captures(&vari.id) {
                                caps.get(1).unwrap().as_str()
                            } else {
                                vari.id.as_str()
                            });

                        Some(Vari {
                            sys_id: format!("{}{}", extra.vari_prefix, vari.id),
                            vari_id,
                            lit: vari.lit.clone(),
                        })
                    }
                })
                .collect();

            writeln!(out, "/// {}.", enum_sys_id).unwrap();
            if extra.vari_blacklist.len() > 0 {
                writeln!(out, "/// Omitted variants:").unwrap();
                for x in &extra.vari_blacklist {
                    writeln!(out, "///  - {}", x).unwrap();
                }
            }

            writeln!(
                out,
                r##"#[repr({raw_type})]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum {enum_id} {{"##,
                enum_id = enum_id,
                raw_type = extra.raw_type.as_str(),
            )
            .unwrap();
            for vari in variants.iter() {
                writeln!(
                    out,
                    r#"    /// {sys_id} = {lit}.
    {vari_id} = sys::{sys_id},"#,
                    sys_id = vari.sys_id,
                    vari_id = vari.vari_id,
                    lit = vari.lit
                )
                .unwrap();
            }

            writeln!(
                out,
                r#"}}

impl Enum for {enum_id} {{
    type Raw = {raw_type};

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {{
        let raw = val.0;
        match raw {{"#,
                enum_id = enum_id,
                raw_type = extra.raw_type.as_str(),
            )
            .unwrap();

            for vari in variants.iter() {
                writeln!(
                    out,
                    "             sys::{0} => Ok({1}::{2}),",
                    vari.sys_id, enum_id, vari.vari_id
                )
                .unwrap();
            }

            writeln!(
                out,
                r#"            _ => Err(Invalid(raw)),
        }}
    }}

    fn into_unchecked(self) -> Unchecked<Self> {{
        Unchecked(self as Self::Raw)
    }}
}}

impl fmt::Display for Invalid<{enum_id}> {{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
        write!(f, "The value {{}} does not represent any variant of {enum_id}.", self.0)
    }}
}}

impl error::Error for Invalid<{enum_id}> {{
    fn description(&self) -> &str {{
        "Value does not represent any variant of {enum_id}."
    }}
}}
"#,
                enum_id = enum_id
            )
            .unwrap();
        }
    }
}
