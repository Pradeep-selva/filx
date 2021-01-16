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
        let args = types::Args::from_args();
        let command: String;

        if args.help {
            utils::display_help();
            return;
        }

        match args.command {
            Some(c) => command = c,
            None => command = String::from(""),
        }

        match command.as_str() {
            "all" => controllers::all_controller(paths),
            "ext" => controllers::ext_controller(paths),
            "prefix" => controllers::prefix_controller(paths),
            _ => {
                println!("{}", "-- Unrecognized command -- \n".red().bold());
                utils::display_help();
            }
        }
    } else {
        utils::display_help();
    }
}
