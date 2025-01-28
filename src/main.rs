
use clap::Parser;
use log::{info,error,debug};
use std::{cell::Ref, fs::{self, File}, io::{Read, Write}, process::Output};
use std::path::Path;
use convert_case::{Case,Casing};
use typify::{TypeSpace, TypeSpaceSettings};
use codegen::Scope;

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

    let file = File::open(file_name)?;

    let mut contents = String::new();

    let mut tmf_scope = Scope::new();

    let tmf_mod = tmf_scope.new_module("customer_bill")
        .import("crate", "HasId")
        .import("crate","HasName")
        .import("crate","HasDescription");

    let tmf_struct = tmf_scope.new_struct("PolicyManagement");
    tmf_struct
        .derive("Clone")
        .derive("Default")
        .derive("Debug")
        .doc("Bill Cycle Module")
        .field("id", "Option<String>")
        .field("href","Option<String>")
        .field("description","Option<String>")
        .vis("pub");

    tmf_mod.push_struct(&tmf_struct);

    let tmf_output = tmf_scope.to_string();

    println!("{}",tmf_output);

    Ok(())
}
