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
    println!("{}", "Welcome, to filx! your file organization tool.\n Neatly organize your files with just with a single command\n".bold().underline());
    println!("{}", "Available commands -- ".green().bold().italic());
    println!("{} -- {}", "1. filx run".cyan().bold(), "organize files based on their extension.".cyan());
    println!("{} -- {}", "2. filx ext -t <extension>".cyan().bold(), 
    "organize files based on specific extension (all extensions if no type provided).".cyan());
}
