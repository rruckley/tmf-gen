
use clap::Parser;
use log::{info,error,debug};
use std::{cell::Ref, fs::{self, File}, io::{Read, Write}, process::Output};
use std::path::Path;
use convert_case::{Case,Casing};
use typify::{TypeSpace, TypeSpaceSettings};

use openapiv3::{OpenAPI, ReferenceOr};

#[derive(Parser,Debug)]
struct Args {
    #[arg(long, help = "OAS File to load")]
    file: String,

    #[arg(long, help = "Output folder")]
    output: String,

    #[arg(long, help = "TMF Number")]
    tmf: String,
}

const SCHEMA_PRELUDE : &str = "\"$schema\": \"https://json-schema.org/draft/2020-12/schema\"";

fn main() -> Result<(),std::io::Error> {
    env_logger::init();

    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    info!("Starting {pkg} : v{ver}");

    let args = Args::parse();

    let file_name = args.file;
    let out_folder = args.output;
    let tmf = args.tmf;

    info!("Using input: {}",file_name);

    let mut file = File::open(file_name)?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    // dbg!(&contents);
    let openapi: OpenAPI = serde_json::from_str(contents.as_str()).expect("Could not deserialise input");

    let components = match openapi.components {
        Some(c) => c,
        None => {
            error!("Could not find components");
            return Ok(());
        }
    };
    let root_folder = openapi.info.title;
    info!("OpenAPI Title: {}",&root_folder);
    let _result = fs::create_dir(format!("{}/{}",out_folder,tmf));

    let schemas = components.schemas;

    for (name,object) in schemas {
        info!("Found schema: {}",name);
        let mod_name = match name.split_once("_") {
            Some(s) => {
                String::from(s.0)
            },
            None => name.clone(),
        }.to_case(Case::Snake);
        let filename = format!("{}.rs",name.to_case(Case::Snake));
        debug!("File Name: {}",filename);

        let mod_path = format!("{}/{}/{}",out_folder,tmf,mod_name);
        debug!("Mod path: {}",mod_path);
        let _result = fs::create_dir(&mod_path);

        match object {
            ReferenceOr::Reference { reference } => {
                info!("Reference {}.{}",name,reference);
            },
            ReferenceOr::Item(i) => {
                info!("Schema {}.schema",name);
                // Since we are creating a file here, we need to add it to
                let file_path = format!("{}/{}",mod_path,filename);

                let mut file = match File::create_new(&file_path) {
                    Ok(f) => f,
                    Err(e) => File::open(&file_path).unwrap()
                };

                let file_contents = format!("//! {}\npub struct {} {{ }}\n",&name,&name);

                let _result = file.write(file_contents.as_bytes())?;
            }
        }
    }                                                                                                                                                                                                                                  

    Ok(())
}
