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

    println!("{} {}", "USAGE --".green().bold().italic(), "filx [OPTIONS] <command> <flags>".bold());
    
    println!("{}", "\nCOMMANDS --".green().bold().italic());
    println!("{} -- {}", "1. filx run".cyan().bold(), "organize files based on their extension.".cyan());
    println!("{} -- {}", "2. filx ext -t <extension>".cyan().bold(), 
    "organize files based on specific extension (all extensions if no type provided).".cyan());
    
    println!("{}", "\nOPTIONS -- ".green().bold().italic());
    println!("{} -- {}", "1. -var <variant> (or) --variant <variant>".cyan().bold(), 
        "specify a variant for your command".cyan());
    println!("\t{}", "available variants".bold().underline());
    println!("\t\t{} -- {}", "1. default".cyan().bold(), 
        "Organize files into their respective folders as specified".cyan());
    println!("\t\t{} -- {}", "1. persist".cyan().bold(), 
        "keep their initial copies intact after organizing".cyan());
    println!("\t\t{} -- {}", "1. backup".cyan().bold(), 
    "take a backup of all organized files in separate folder".cyan());
    
    println!("{}", "\nFLAGS -- ".green().bold().italic());
    println!("{} -- {}", "1. -V (or) --version".cyan().bold(), "See your filx version".cyan());
    println!("{} -- {}", "2. -h (or) --help".cyan().bold(), "Get a help message".cyan());
}
