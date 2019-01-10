#![feature(custom_attribute)]

mod types;

use self::types::*;

use serde_json;
use regex::Regex;
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

        for enum_ in data.enums.iter() {
            let name = if let Some(caps) = enum_name_regex.captures(&enum_.id) {
                caps.get(1).unwrap().as_str()
            } else {
                panic!("Failed to convert {:?}", enum_);
            };

            // We want to do some automated renaming and use the new
            // names multiple times. So we create a new version of the
            // enum variants.
            let variants: Vec<EnumVariant> = enum_
                .variants
                .iter()
                .map(|vari| {
                    let vari_id = if let Some(caps) = vari_name_regex.captures(&vari.id) {
                        caps.get(1).unwrap().as_str()
                    } else {
                        vari.id.as_str()
                    };
                    let vari_lit = if vari.lit == "-1" {
                        "::std::u32::MAX"
                    } else {
                        vari.lit.as_str()
                    };
                    EnumVariant {
                        id: vari_id.to_string(),
                        lit: vari_lit.to_string(),
                    }
                })
                .collect();

            writeln!(out, "#[repr(transparent)]").unwrap();
            writeln!(out, "#[derive(Debug, Eq, PartialEq, Copy, Clone)]").unwrap();
            writeln!(out, "pub struct Raw{}(pub u32);\n", name).unwrap();

            writeln!(out).unwrap();

            for vari in variants.iter() {
                writeln!(
                    out,
                    "pub const {1}_{0}: Raw{1} = Raw{1}({2});",
                    vari.id, name, vari.lit
                )
                .unwrap();
            }

            writeln!(out).unwrap();

            writeln!(out, "#[repr(u32)]").unwrap();
            writeln!(out, "#[derive(Debug, Eq, PartialEq, Copy, Clone)]").unwrap();
            writeln!(out, "pub enum {} {{", name).unwrap();
            for vari in variants.iter() {
                writeln!(out, "    {} = {},", vari.id, vari.lit).unwrap();
            }
            writeln!(out, "}}").unwrap();

            writeln!(out).unwrap();

            writeln!(out, "impl {} {{", name).unwrap();
            writeln!(out, "    #[inline]").unwrap();
            writeln!(out, "    fn from_raw(val: Raw{}) -> Option<Self> {{", name).unwrap();
            writeln!(out, "         match val {{").unwrap();
            for vari in variants.iter() {
                writeln!(
                    out,
                    "             {0}_{1} => Some({0}::{1}),",
                    name, vari.id
                )
                .unwrap();
            }
            writeln!(out, "             _ => None,").unwrap();
            writeln!(out, "         }}").unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();

            writeln!(out).unwrap();

            writeln!(out, "impl From<Raw{0}> for {0} {{", name).unwrap();
            writeln!(out, "    fn from(val: Raw{}) -> Self {{", name).unwrap();
            writeln!(out, "        {}::from_raw(val).unwrap_or_else(|| {{", name).unwrap();
            writeln!(
                out,
                "            panic!(\"Invalid value {{}} for {}.\");",
                name
            )
            .unwrap();
            writeln!(out, "        }})").unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();

            writeln!(out).unwrap();
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
