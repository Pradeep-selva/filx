mod utils;
mod types;
use std::fs;
use colored::*;
use structopt::StructOpt;

fn main() {
    let paths = fs::read_dir("./").unwrap();
    let is_arg = utils::is_args_empty();

    if is_arg {
        let cli = types::Cli::from_args();

        let mut extensions: Vec<String> = Vec::new();

        match cli.command.as_str() {
            "all" => {
                for path in paths {
                    let file_name: String = path.unwrap()
                                    .path()
                                    .display()
                                    .to_string()
                                    .replace("./", "");
                    let new_file = file_name.to_owned();

                    let extension = utils::get_extension(file_name);
                    
                    let should_copy = !extension.is_empty();
                    let should_append = !extensions.contains(&extension) && should_copy;
                    
                    if should_append {
                        let extension = extension.to_owned();
                    
                        extensions.push(extension);
                    
                        let dir_name = extensions.last().clone().unwrap();
                        match fs::create_dir("./".to_string()+dir_name) {
                            Ok(_) => println!("{}: {}","CREATED".bold(), dir_name),
                            Err(e) => println!("{}: {:?}","ERROR".bold().red(), e)
                        };

                    }

                    if should_copy {
                        let file_to_copy = "./".to_string()+&new_file;
                        let file_to_delete = file_to_copy.to_owned();
                        let dir_to_paste = "./".to_string()+&extension+"/"+&new_file; 

                        match fs::copy(file_to_copy, dir_to_paste) {
                            Ok(_) => println!("{}: {}","COPIED".bold(), new_file),
                            Err(e) => println!("{}: {:?}", "ERROR".bold().red(), e)
                        }

                        match fs::remove_file(file_to_delete) {
                            Ok(_) => println!("{}: {}","REMOVED".bold(), new_file),
                            Err(e) => println!("{}: {:?}","ERROR".bold().red(), e)
                        }
                    }
                }
            },
            "ext" => {
                match cli.extension_type {
                    None => println!("{}", 
                            "Enter a valid extension type with the -t (or) -type flag.".red()
                            .bold()),
                    Some(ext_type) => println!("{}", ext_type)
                }
            }
            _ => {
                println!("{}", "-- Unrecognized command -- \n".red().bold());
                utils::display_help();
            }
        }
    } else {
        utils::display_help();
    }
}
