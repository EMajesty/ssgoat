use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use comrak::{markdown_to_html, Options};
use glob::glob;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut in_path = ".";
    let mut out_path = ".";

    match args.len() {
        1 => (),
        2 => in_path = &args[1],
        3 => {
            in_path = &args[1];
            out_path = &args[2];
        },
        _ => {
            panic!("Too many arguments")
        },
    }

    println!("Input path is {}", in_path);
    println!("Output path is {}", out_path);

    let in_folder_structure = "/**/*.md";
    let search_path = format!("{}{}", in_path, in_folder_structure);

    for entry in glob(&search_path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                process_file(&path, &in_path, &out_path);
            },
            Err(e) => println!("{:?}", e),
        }
    }

    println!("Done");
}

fn process_file(path: &Path, in_path: &str, out_path: &str) {
    dbg!(&path);
    dbg!(&out_path);

    let markdown = fs::read_to_string(path).expect("Failed to read file");
    let html = markdown_to_html(&markdown, &Options::default());
    let write_path = path.strip_prefix(in_path)
        .unwrap()
        .with_extension("html");
    let write_path = format!("{}/{}", out_path, write_path.display());

    dbg!(&write_path);

    if let Some(parent) = Path::new(&write_path).parent() {
        fs::create_dir_all(parent).expect("Failed to create directory");
    }

    let mut file = File::create(&write_path).expect("Failed to create file");
    file.write_all(html.as_bytes()).expect("Failed to write to file");
}
