use std::fs;
mod all;

pub fn all_controller(paths: fs::ReadDir) {
    all::control(paths);
}