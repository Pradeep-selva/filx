mod utils;
mod types;
mod controllers;
use std::fs;
use colored::*;
use structopt::StructOpt;

fn main() {
    let paths = fs::read_dir("./").unwrap();
    let is_arg = utils::is_args_empty();

    if is_arg {
        let cli = types::Cli::from_args();

        match cli.command.as_str() {
            "all" => controllers::all_controller(paths),
            "ext" => {
                match cli.extension_type {
                    None => println!("{}", 
                            "Enter a valid extension type with the -t (or) -type flag.".red()
                            .bold()),
                    Some(ext_type) => println!("{}", ext_type)
                }
            }
            _ => {
                println!("{}", "-- Unrecognized command -- \n".red().bold());
                utils::display_help();
            }
        }
    } else {
        utils::display_help();
    }
}
