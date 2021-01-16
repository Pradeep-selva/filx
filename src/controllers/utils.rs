use colored::*;

#[allow(dead_code)]
pub fn get_extension(path: String) -> String {
    let path_name: Vec<&str> = path.split('.').collect::<Vec<&str>>();

    let extension: &str = path_name.last().clone().unwrap();
    if path_name.len() > 1 && path_name[0] != "" {
        return extension.to_string();
    } else {
        return "".to_string();
    }
}

#[allow(dead_code)]
pub fn get_file_name_without_extension(path: String) -> String {
   let path_name: Vec<&str> = path.split('.').collect::<Vec<&str>>();

    let file_name: &str = path_name[0];
    if path_name.len() > 1 && path_name[0] != "" {
        return file_name.to_string();
    } else {
        return "".to_string();
    } 
}

#[allow(dead_code)]
pub fn get_prefix_or_suffix_or_contain(file_name: String, search_term: &str) -> i32 {

    // -1 -> not found
    // 0 -> prefix
    // 1 -> suffix
    // 2 -> contains

    let split_name: Vec<&str> = file_name.split(&search_term.to_lowercase())
                                    .collect::<Vec<&str>>();

    if split_name.len() < 2 {
        return -1;
    } else {
        if split_name[0].is_empty() {
            return 0;
        } else if split_name[1].is_empty() {
            return 1;
        } else {
            return 2;
        }
    }
}

#[allow(dead_code)]
pub fn log_success(title: &str, message: &str) {
    println!("{}: {}", title.bold(), message)
}

#[allow(dead_code)]
pub fn log_error(error: std::io::Error) {
    println!("{}: {}", "ERROR".red().bold(), error.to_string().red())
}