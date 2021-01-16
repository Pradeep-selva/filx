use colored::*;

pub fn get_extension(path: String) -> String {
    let path_name: Vec<&str> = path.split('.').collect::<Vec<&str>>();

    let extension: &str = path_name.last().clone().unwrap();
    if path_name.len() > 1 && path_name[0] != "" {
        return extension.to_string();
    } else {
        return "".to_string();
    }
}

pub fn log_success(title: &str, message: &str) {
    println!("{}: {}", title.bold(), message)
}

pub fn log_error(error: std::io::Error) {
    println!("{}: {}", "ERROR".red().bold(), error.to_string().red())
}