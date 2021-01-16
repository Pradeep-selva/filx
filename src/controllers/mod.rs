use std::fs;
use structopt::StructOpt;
use colored::*;
mod all_ext;
#[path = "../types.rs"] mod types;

pub fn all_controller(paths: fs::ReadDir) {
    all_ext::control(paths, "".to_string());
}

pub fn ext_controller(paths: fs::ReadDir) {
    let args = types::Args::from_args();

    match args.extension_type {
        None => println!("{}\n{}", 
                "Enter a valid extension type with the -t (or) --type flag.".red()
                .bold(), "Run -h (or) --help for more info.".green().bold()),
        Some(ext_type) => all_ext::control(paths, ext_type.to_string())
    }
}