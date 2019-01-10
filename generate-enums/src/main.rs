#![feature(custom_attribute)]

mod types;

use self::types::*;

use regex::Regex;
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use inflector::cases::screamingsnakecase::to_screaming_snake_case;
use inflector::cases::classcase::to_class_case;

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

        writeln!(out, "use openvr_sys as sys;").unwrap();

        writeln!(out).unwrap();

        for enum_ in data.enums.iter() {
            let enum_id = if let Some(caps) = enum_name_regex.captures(&enum_.id) {
                caps.get(1).unwrap().as_str()
            } else {
                panic!("Failed to convert {:?}", enum_);
            };

            struct Vari {
                sys_id: String,
                const_id: String,
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
                    let vari_id = to_class_case(if let Some(caps) = vari_name_regex.captures(&vari.id) {
                        caps.get(1).unwrap().as_str()
                    } else {
                        vari.id.as_str()
                    });

                    Vari {
                    sys_id: vari.id.clone(),
                    const_id: to_screaming_snake_case(&format!("{}_{}", enum_id, vari_id)),
                    vari_id,
                    lit: if vari.lit == "-1" {
                        "::std::u32::MAX"
                    } else {
                        vari.lit.as_str()
                    }
                    .to_string(),
                    }})
                .collect();

            writeln!(out, "#[repr(transparent)]").unwrap();
            writeln!(out, "#[derive(Debug, Eq, PartialEq, Copy, Clone)]").unwrap();
            writeln!(out, "pub struct Raw{}(pub u32);\n", enum_id).unwrap();

            writeln!(out).unwrap();

            for vari in variants.iter() {
                writeln!(
                    out,
                    "pub const {0}: Raw{1} = Raw{1}(sys::{2}); // {3}",
                    vari.const_id, enum_id, vari.sys_id, vari.lit
                )
                .unwrap();
            }

            writeln!(out).unwrap();

            writeln!(out, "#[repr(u32)]").unwrap();
            writeln!(out, "#[derive(Debug, Eq, PartialEq, Copy, Clone)]").unwrap();
            writeln!(out, "pub enum {} {{", enum_id).unwrap();
            for vari in variants.iter() {
                writeln!(out, "    {} = sys::{}, // {}", vari.vari_id, vari.sys_id, vari.lit).unwrap();
            }
            writeln!(out, "}}").unwrap();

            writeln!(out).unwrap();

            writeln!(out, "impl {} {{", enum_id).unwrap();
            writeln!(out, "    #[inline]").unwrap();
            writeln!(out, "    fn from_raw(val: Raw{}) -> Option<Self> {{", enum_id).unwrap();
            writeln!(out, "         match val {{").unwrap();
            for vari in variants.iter() {
                writeln!(
                    out,
                    "             {0} => Some({1}::{2}),",
                    vari.const_id, enum_id, vari.vari_id
                )
                .unwrap();
            }
            writeln!(out, "             _ => None,").unwrap();
            writeln!(out, "         }}").unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();

            writeln!(out).unwrap();

            writeln!(out, "impl From<Raw{0}> for {0} {{", enum_id).unwrap();
            writeln!(out, "    fn from(val: Raw{}) -> Self {{", enum_id).unwrap();
            writeln!(out, "        {}::from_raw(val).unwrap_or_else(|| {{", enum_id).unwrap();
            writeln!(
                out,
                "            panic!(\"Invalid value {{}} for {}.\");",
                enum_id
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
