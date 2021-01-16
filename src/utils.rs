use std::env;
use colored::*;

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