use std::fs;
use structopt::StructOpt;
use colored::*;
#[path = "../types.rs"] mod types;
#[path = "./utils.rs"] mod utils;

pub fn control(paths: fs::ReadDir, extension_to_check: String) {
    let mut extensions:Vec<String> = Vec::new();

    let mut should_persist = false;
    let mut should_backup = false;
    let args = types::Args::from_args();

    match args.variant {
        Some(variant) => {
            println!("{}", variant);
            match variant.as_str() {
                "default" => (),
                "persist" => should_persist = true,
                "backup" => should_backup = true,
                _ => {
                    println!("{} {} \n run --help for more info.", 
                        "Unrecognized variant -- use ".red().bold(), 
                        "default, persist, backup.".bold());
                    return;
                }
            }
        },
        None => ()
    }

    if should_backup {
        match fs::create_dir("./filx_backups") {
            Ok(_) => println!("{}", "Created backups dir".green().bold()),
            Err(e) => utils::log_error(e)
        }
    }

    for path in paths {
        let file_name: String = path.unwrap()
                        .path()
                        .display()
                        .to_string()
                        .replace("./", "");
        let new_file = file_name.to_owned();

        let extension = utils::get_extension(file_name);
        
        let mut should_copy = !extension.is_empty();

        if !extension_to_check.is_empty() {
            should_copy = should_copy && extension == extension_to_check;
        }

        let should_append = !extensions.contains(&extension) && should_copy;
        
        if should_append {
            let extension = extension.to_owned();
        
            extensions.push(extension);

            let dir_name = extensions.last().clone().unwrap();
            match fs::create_dir("./".to_string()+dir_name) {
                Ok(_) => utils::log_success("CREATED", dir_name),
                Err(_) => () 
            };
        }

        if should_copy {
            let file_to_copy = "./".to_string()+&new_file;
            let file_to_delete = file_to_copy.to_owned();
            let dir_to_paste = "./".to_string()+&extension+"/"+&new_file;

            let file_to_backup = file_to_copy.to_owned();
            let dir_to_backup = "./filx_backups".to_string()+"/"+&new_file;

            match fs::copy(file_to_copy, dir_to_paste) {
                Ok(_) => utils::log_success("COPIED", new_file.as_str()),
                Err(e) => utils::log_error(e)
            }

            if should_backup {
               match fs::copy(file_to_backup, dir_to_backup) {
                    Ok(_) => utils::log_success("BACKUP", new_file.as_str()),
                    Err(e) => utils::log_error(e)
                } 
            }

            if !should_persist {
                match fs::remove_file(file_to_delete) {
                    Ok(_) => utils::log_success("REMOVED", new_file.as_str()),
                    Err(e) => utils::log_error(e)
                }
            }
        }
    }
}