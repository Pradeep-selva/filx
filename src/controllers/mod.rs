use std::fs;
use structopt::StructOpt;
use colored::*;
mod rearrange;
mod search;
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

pub fn prefix_controller(paths: fs::ReadDir) {
    let args = types::Args::from_args();

    match args.search_content {
        None => println!("{}\n{}", 
                "Enter a valid search content with the -sc (or) --search-content flag.".red()
                .bold(), "Run -h (or) --help for more info.".green().bold()),
        Some(search_content) => search::control(paths, "".to_string(), search_content, 0)
    }
}