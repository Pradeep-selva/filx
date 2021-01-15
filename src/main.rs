mod utils;
mod types;
use std::fs;
use structopt::StructOpt;

fn main() {
    let paths = fs::read_dir("./").unwrap();
    let is_arg = utils::is_args_empty();

    if is_arg {
        let cli = types::Cli::from_args();

        println!("{:?}", cli);

        let mut extensions: Vec<String> = Vec::new();

        for path in paths {
            let file_name: String = path.unwrap().path().display().to_string().replace("./", "");

            let extension = utils::get_extension(file_name);
            
            let should_append = !extensions.contains(&extension) && !extension.is_empty();
            
            if should_append {
                let extension = extension.to_owned();
            
                extensions.push(extension);
            
                // let dir_name = extensions.last().clone().unwrap();
                // match fs::create_dir("./".to_string()+dir_name) {
                //     Ok(_) => println!("CREATED: {}", dir_name),
                //     Err(e) => println!("ERROR: {:?}", e)
                // };
            }
        }

        println!("{:?}", extensions);
    } else {
        println!("Enter args")
    }
}
