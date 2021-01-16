mod all;
use std::fs;

pub fn all_controller(paths: fs::ReadDir) {
    all::control(paths);
}