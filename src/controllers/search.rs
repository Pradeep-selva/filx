use std::fs;
use structopt::StructOpt;
use colored::*;
#[path = "../types.rs"] mod types;
#[path = "./utils.rs"] mod utils;

// search type --
// -1 -> none
// 0 -> prefix
// 1 -> suffix
// 2 -> contains

pub fn control(
    paths: fs::ReadDir, 
    extension_to_check: String, 
    search_content: String, 
    search_type: i32
) {
    let mut extensions:Vec<String> = Vec::new();

    let mut should_persist = false;
    let mut should_backup = false;
    let args = types::Args::from_args();

    match args.variant {
        Some(variant) => {
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

        let file_to_check = new_file.to_owned();
        let file_name_without_extension = utils::get_file_name_without_extension(file_to_check);
        let fname_if_contains = file_name_without_extension.to_owned();
        let mut is_present = false;

        if search_type == utils::get_prefix_or_suffix_or_contain(
            file_name_without_extension, 
            search_content.as_str()
        ) {
            is_present = true;
        } else if search_type == 2 && utils::get_prefix_or_suffix_or_contain(
            fname_if_contains, 
            search_content.as_str()
        ) != -1 {
            is_present = true;
        } 

        should_copy = should_copy && is_present;

        if !extension_to_check.is_empty() {
            should_copy = should_copy && extension == extension_to_check;
        }

        let should_append = !extensions.contains(&extension) && should_copy;
        
        if should_append {
            let extension = extension.to_owned();
        
            extensions.push(extension);

            match fs::create_dir("./".to_string()+search_content.as_str()) {
                Ok(_) => utils::log_success("CREATED", search_content.as_str()),
                Err(_) => ()
            };
        }

        if should_copy {
            let file_to_copy = "./".to_string()+&new_file;
            let file_to_delete = file_to_copy.to_owned();
            let dir_to_paste = "./".to_string()+search_content.as_str()+"/"+&new_file;

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