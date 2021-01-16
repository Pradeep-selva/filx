use std::fs;
use structopt::StructOpt;
use colored::*;
mod rearrange;
mod search;
mod rename;
#[path = "../types.rs"] mod types;

pub fn all_controller(paths: fs::ReadDir) {
    rearrange::control(paths, "".to_string());
}

pub fn ext_controller(paths: fs::ReadDir) {
    let args = types::Args::from_args();

    match args.extension_type {
        None => println!("{}\n{}", 
                "Enter a valid extension type with the -t (or) --type flag.".red()
                .bold(), "Run -h (or) --help for more info.".green().bold()),
        Some(ext_type) => rearrange::control(paths, ext_type.to_string())
    }
}

pub fn search_controller(paths: fs::ReadDir, search_type: i32) {
    let args = types::Args::from_args();

    match args.search_content {
        None => println!("{}\n{}", 
                "Enter a valid search content with the --search-content flag.".red()
                .bold(), "Run -h (or) --help for more info.".green().bold()),
        Some(search_content) => {
            match args.extension_type {
                None => search::control(paths, "".to_string(), search_content, search_type),
                Some(ext_type) => search::control(paths, ext_type, search_content, search_type)
            }
        }
    }
}

pub fn rename_controller(paths: fs::ReadDir, rename_type: i32) {
    let args = types::Args::from_args();
    let mut search_content: String = String::from("");
    let mut extension_to_check: String = String::from("");

    match args.search_content {
        Some(search) => search_content = search,
        None => ()
    }

    match args.extension_type {
        Some(ext_type) => extension_to_check = ext_type,
        None => ()
    }

    match args.text {
        Some(text) => rename::control(paths, extension_to_check, search_content, text, rename_type),
        None => println!("{}\n{}", 
                "Enter a valid text to manipulate name with --text flag.".red()
                .bold(), "Run -h (or) --help for more info.".green().bold()) 
    }
}