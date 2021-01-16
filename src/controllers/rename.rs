use std::fs;
use colored::*;
#[path = "../types.rs"] mod types;
#[path = "./utils.rs"] mod utils;
use chrono::prelude::DateTime;
use chrono::Utc;

// rename types --
// 0 -> prepend
// 1 -> append
// 2 -> prepend-date
// 3 => append-date

pub fn control(
    paths: fs::ReadDir, 
    extension_to_check: String, 
    search_content: String, 
    text: String,
    rename_type: i32
) {
    for path in paths {
        let rename_text = text.to_owned();
        let file_name: String = path.unwrap()
                        .path()
                        .display()
                        .to_string()
                        .replace("./", "");
        let new_file = file_name.to_owned();

        let extension = utils::get_extension(file_name);
        let mut should_rename = true;

        if !search_content.is_empty() &&
            !new_file.contains(&search_content) {
                should_rename = false;
        }

        if !extension_to_check.is_empty() &&
            extension != extension_to_check {
                should_rename = false;
        }

        if should_rename {
            let mut changed_file_name: String = String::from("");
            let old_file_name = new_file.to_owned();

            if rename_type == 0 {
                changed_file_name = rename_text + new_file.as_str();
            } else if rename_type == 1 {
                if !extension.is_empty() {
                    let file_name_without_extension = utils::get_file_name_without_extension(new_file);
                    changed_file_name = file_name_without_extension + text.as_str() + "." + &extension;
                } else {
                    changed_file_name = new_file + rename_text.as_str();
                }
            } else if rename_type == 2 {
                match fs::metadata(new_file) {
                    Ok(meta) => {
                        if let Ok(time) = meta.modified() {
                            let datetime: DateTime<Utc> = time.into();
                            let timestamp_str = datetime.format("%Y-%m-%d").to_string();
                            changed_file_name = timestamp_str+"_"+ old_file_name.as_str();

                            println!("{:?}", changed_file_name);
                        } else {
                            println!("{}", "Not supported on this platform".red().bold());
                        }
                    },
                    Err(_) => ()
                }
            }

            changed_file_name = "./".to_string() + changed_file_name.as_str();
            let old_file_name = "./".to_string()+old_file_name.as_str();
            let display_name = old_file_name.to_owned();

            match fs::rename(old_file_name, changed_file_name) {
                Ok(_) => utils::log_success("RENAMED", display_name.as_str()),
                Err(e) => utils::log_error(e)
            }
        }
    }
}