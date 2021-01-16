use std::fs;
use structopt::StructOpt;
use colored::*;
#[path = "../types.rs"] mod types;
#[path = "./utils.rs"] mod utils;

pub fn control(paths: fs::ReadDir) {
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
            Err(e) => println!("{}: {:?}", "ERROR".bold().red(), e)
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

            let file_to_backup = file_to_copy.to_owned();
            let dir_to_backup = "./filx_backups".to_string()+"/"+&new_file;

            match fs::copy(file_to_copy, dir_to_paste) {
                Ok(_) => println!("{}: {}","COPIED".bold(), new_file),
                Err(e) => println!("{}: {:?}", "ERROR".bold().red(), e)
            }

            if should_backup {
               match fs::copy(file_to_backup, dir_to_backup) {
                    Ok(_) => println!("{}: {}","BACKUP".bold(), new_file),
                    Err(e) => println!("{}: {:?}", "ERROR".bold().red(), e)
                } 
            }

            if !should_persist {
                match fs::remove_file(file_to_delete) {
                    Ok(_) => println!("{}: {}","REMOVED".bold(), new_file),
                    Err(e) => println!("{}: {:?}","ERROR".bold().red(), e)
                }
            }
        }
    }
}