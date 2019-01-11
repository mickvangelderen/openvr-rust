#![feature(custom_attribute)]

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

    {
        let mut output_file = File::create(&output_file_path).expect("Failed to open file.");
        let out = &mut output_file;

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
            let enum_id = if let Some(caps) = enum_name_regex.captures(&enum_.id) {
                caps.get(1).unwrap().as_str()
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
                .map(|vari| {
                    let vari_id =
                        to_class_case(if let Some(caps) = vari_name_regex.captures(&vari.id) {
                            caps.get(1).unwrap().as_str()
                        } else {
                            vari.id.as_str()
                        });

                    Vari {
                        sys_id: vari.id.clone(),
                        vari_id,
                        lit: if vari.lit == "-1" {
                            "::std::u32::MAX"
                        } else {
                            vari.lit.as_str()
                        }
                        .to_string(),
                    }
                })
                .collect();

            writeln!(out, "#[repr(u32)]").unwrap();
            writeln!(out, "#[derive(Debug, Eq, PartialEq, Copy, Clone)]").unwrap();
            writeln!(out, "pub enum {} {{", enum_id).unwrap();
            for vari in variants.iter() {
                writeln!(
                    out,
                    "    {} = sys::{}, // {}",
                    vari.vari_id, vari.sys_id, vari.lit
                )
                .unwrap();
            }
            writeln!(out, "}}").unwrap();

            writeln!(out).unwrap();

            writeln!(out, "impl Enum for {} {{", enum_id).unwrap();
            writeln!(out, "    #[inline]").unwrap();
            writeln!(
                out,
                "    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {{",
            )
            .unwrap();

            writeln!(out, "         let raw = val.0;").unwrap();
            writeln!(out, "         match raw {{").unwrap();
            for vari in variants.iter() {
                writeln!(
                    out,
                    "             sys::{0} => Ok({1}::{2}),",
                    vari.sys_id, enum_id, vari.vari_id
                )
                .unwrap();
            }
            writeln!(out, "             _ => Err(Invalid(raw)),").unwrap();
            writeln!(out, "         }}").unwrap();
            writeln!(out, "    }}").unwrap();

            writeln!(out).unwrap();

            writeln!(out, "    fn into_unchecked(self) -> Unchecked<Self> {{",).unwrap();
            writeln!(out, "         Unchecked(self as Self::Raw)").unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();

            writeln!(out).unwrap();

            writeln!(out, r#"
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
"#, enum_id = enum_id).unwrap();
        }
    }

    let mut rustfmt_cmd = std::process::Command::new("rustfmt");

    rustfmt_cmd
        .arg("--emit")
        .arg("files")
        .arg(&output_file_path);

    println!("Running {:?}", &rustfmt_cmd);

    match rustfmt_cmd.status() {
        Ok(status) => {
            if !status.success() {
                println!("rustfmt failed.");
            }
        }
        Err(error) => {
            eprintln!("Failed to spawn rustfmt process: {}", error);
        }
    }
}
