use std::fs;
#[path = "../types.rs"] mod types;
#[path = "./utils.rs"] mod utils;

// rename types --
// 0 -> prepend
// 1 -> append
// 2 -> date

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
                let file_name_without_extension = utils::get_file_name_without_extension(new_file);
                changed_file_name = file_name_without_extension + text.as_str() + "." + &extension;
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