use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();

    let mut extensions: Vec<String> = Vec::new();

    for path in paths {
        let file_name: String = path.unwrap().path().display().to_string().replace("./", "");

        println!("{:?}", file_name);
    }

    println!("{:?}", extensions);
}
