use std::fs::File;
#[path = "./utils.rs"] mod utils;

// dev command to seed for testing.

fn add_to_char(c: char, n: u32) -> char {
    std::char::from_u32(c as u32 + n).unwrap_or(c)
}

pub fn control() {
    let base = "ab";
    let ch: char = 'c';

    for i in 0..40 {
        let new_ch = add_to_char(ch, i).to_string();
        match File::create(base.to_string()+&new_ch+".cpp") {
            Ok(_) => utils::log_success("CREATED", "New File..."),
            Err(e) => utils::log_error(e)
        }
    }
}