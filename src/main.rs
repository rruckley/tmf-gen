
use clap::{Parser,Subcommand};
use log::{info,debug};
use std::{fs::File, io::{Read, Write}, process::Output};
use convert_case::{Case,Casing};
use typify::{TypeSpace, TypeSpaceSettings};

use openapiv3::OpenAPI;

#[derive(Parser,Debug)]
struct Args {
    #[arg(long, help = "OAS File to load")]
    file: String,

    #[arg(long, help = "Output folder")]
    output: String,
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

    let mut file = File::open(file_name)?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let openapi : OpenAPI = serde_json::from_str(contents.as_str()).expect("Could not parse YAML");

    let components = openapi.components.expect("No components found!");
    let schemas = components.schemas;

    let mod_title = openapi.info.title;
    let mod_version = openapi.info.version;
    info!("Processing module: {} v{}",mod_title,mod_version);
    let out_schema = format!("{{\n\t{}",SCHEMA_PRELUDE);
    for (name,schema) in schemas.iter() {
        match schema.as_item() {
            Some(s) => {
                let json = serde_json::to_string(s)?;
                out_schema.push_str(format!("\n\"{}\": {}\n"),name,json).as_str();
                // let snake_name = name.to_case(Case::Snake);
                // let snake_json = format!("{}.json",snake_name);
                // let output_file = format!("{}/{}/{}/{}",out_folder,mod_title,mod_version,snake_json);
                // info!("Creating schema: {}",output_file);
                // let mut file = File::create(output_file)?;
                // let out = format!("{{\n{},\n\"{}\": {}\n}}\n",SCHEMA_PRELUDE,name,json);
                // file.write_all(out.as_bytes())?;      

                // let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
                // let root_schema = serde_json::from_str::<schemars::schema::RootSchema>(&json).unwrap();
                // type_space.add_root_schema(root_schema).unwrap();

                // let contents = format!(
                //          "{}\n{}",
                //          "use serde::{Deserialize, Serialize};",
                //          prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap())
                // );

            }
            None => {}
        }

        // Now create rust class
        // // 
        
    }

    Ok(())
}
