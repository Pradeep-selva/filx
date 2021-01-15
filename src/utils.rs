use std::env;
use colored::*;

pub fn get_extension(path: String) -> String {
    let path_name: Vec<&str> = path.split('.').collect::<Vec<&str>>();

    let extension: &str = path_name.last().clone().unwrap();
    if path_name.len() > 1 && path_name[0] != "" {
        return extension.to_string();
    } else {
        return "".to_string();
    }
}

pub fn is_args_empty() -> bool {
    if env::args().len() > 1 {
        return true;
    } else {
        return false;
    }
}

pub fn display_help() {
    println!("{}", "Available commands -- ".green().bold().italic());
    println!("{} -- {}", "1. filx run".cyan().bold(), "organize files based on their extension.".cyan());
    println!("{} -- {}", "2. filx ext -t <extension>".cyan().bold(), 
    "organize files based on specific extension (all extensions if no type provided).".cyan());
}