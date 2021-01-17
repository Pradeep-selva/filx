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
    println!("{} -- {}", "2. filx ext --type <extension>".cyan().bold(), 
    "organize files based on specific extension (all extensions if no type provided).".cyan());
    println!("{} -- {}", "3. filx prefix --content <string>".cyan().bold(), 
    "organize files based on their file name prefix.".cyan());
    println!("{} -- {}", "4. filx suffix --content <string>".cyan().bold(), 
    "organize files based on their file name suffix.".cyan());
    println!("{} -- {}", "5. filx contains --content <string>".cyan().bold(), 
    "organize files based on text contained in their filename.".cyan());
    println!("{} -- {}", "6. filx prepend --text <string>".cyan().bold(), 
    "prepend a text to all files.".cyan());
    println!("{} -- {}", "7. filx append --text <string>".cyan().bold(), 
    "append a text to all files.".cyan());
    println!("{} -- {}", "8. filx prepend-date".cyan().bold(), 
    "prepend the last modified date of all files to themselves.".cyan());
    println!("{} -- {}", "9. filx append-date".cyan().bold(), 
    "append the last modified date of all files to themselves".cyan());
    
    println!("{}", "\nOPTIONS -- ".green().bold().italic());
    println!("{} -- {}", "1. --variant <variant>".cyan().bold(), 
        "specify a variant for your command".cyan());
    println!("\t{}", "available variants".bold().underline());
    println!("\t\t{} -- {}", "1. default".cyan().bold(), 
        "Organize files into their respective folders as specified".cyan());
    println!("\t\t{} -- {}", "2. persist".cyan().bold(), 
        "keep their initial copies intact after organizing".cyan());
    println!("\t\t{} -- {}", "2. backup".cyan().bold(), 
    "take a backup of all organized files in separate folder".cyan());

    println!("{} -- {}", "2. --type <file_type>".cyan().bold(), 
        "specify the type of the files to organize".cyan());
    println!("\t{}: {}", "NOTE".bold().underline(), 
    "type can be chained to ext, prefix, suffix and contains".cyan());

    println!("{} -- {}", "3. --content <string>".cyan().bold(), 
        "enter any text to search in filename and organize.".cyan());
    println!("\t{}: {}", "NOTE".bold().underline(), 
    "type can be chained to prefix, suffix, contains, prepend and append".cyan());

    println!("{} -- {}", "4. --text <string>".cyan().bold(), 
        "enter any text to append/prepend.".cyan());
    println!("\t{}: {}", "NOTE".bold().underline(), 
    "type can be chained to append, prepend".cyan());
    
    println!("{}", "\nFLAGS -- ".green().bold().italic());
    println!("{} -- {}", "1. -V (or) --version".cyan().bold(), "See your filx version".cyan());
    println!("{} -- {}", "2. -h (or) --help".cyan().bold(), "Get a help message".cyan());
}
