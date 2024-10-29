use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use comrak::{markdown_to_html, Options};
use glob::glob;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut in_path = ".";
    let mut out_path = ".";
    let in_folder_structure = "/**/*.md";

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

    let search_path = format!("{}{}", in_path, in_folder_structure);
    let mut file_list: Vec<PathBuf> = Vec::new();

    for entry in glob(&search_path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                file_list.push(path);
            },
            Err(e) => println!("{:?}", e),
        }
    }

    let mut html;

    let sidebar = create_sidebar(&file_list, &in_path);
    let footer = create_footer();

    for file in &file_list {
        let header = create_header(file.to_str().unwrap(), in_path);
        html = convert_file(&file);
        html = format!("{}{}{}{}", header, sidebar, html, footer);
        write_file(html, file, &in_path, &out_path);
    }

    dbg!(&file_list);
    println!("Done");
}

fn convert_file(path: &Path) -> String {
    let markdown = fs::read_to_string(path).expect("Failed to read file");
    let html = markdown_to_html(&markdown, &Options::default());
    return html;
}

fn create_header(mut title: &str, in_path: &str) -> String {
    let depth = &title.chars().filter(|&x| x == '/').count();

    dbg!(&title);
    dbg!(&depth);
    title = title.strip_prefix(in_path).unwrap()
        .strip_prefix("/").unwrap()
        .strip_suffix(".md").unwrap();
    let mut header_html = String::from("<!DOCTYPE html><html><head><title>");
    header_html.push_str(title);
    header_html.push_str("</title>");
    header_html.push_str("<link rel=\"stylesheet\" href=");
    
    for _ in 0..depth - 2 {
        header_html.push_str("../");
    }

    header_html.push_str("\"resources/css/style.css\">");
    header_html.push_str("</head><body>");
    header_html
}

fn create_sidebar(file_list: &Vec<PathBuf>, in_path: &str) -> String {
    let mut sidebar_html = String::from("<div class\"sidebar\"><ul>");

    for file in file_list {
        let file_name = file.file_name().unwrap()
            .to_str().unwrap()
            .strip_suffix(".md").unwrap();
        let clean_path = file.to_str().unwrap()
            .strip_prefix(in_path).unwrap()
            .strip_prefix("/").unwrap()
            .strip_suffix(".md").unwrap();
        sidebar_html.push_str(&format!("<li><a href=\"{}.html\">{}</a></li>",
                &clean_path, &file_name));
        dbg!(file_name);
        dbg!(clean_path);
    }

    sidebar_html.push_str("</ul></div>");
    sidebar_html
}

fn create_footer() -> String {
    let footer_html = String::from("</body></html>");
    footer_html
}

fn write_file(html: String, path: &Path, in_path: &str, out_path: &str) {
    let write_path = path.strip_prefix(in_path)
        .unwrap()
        .with_extension("html");
    let write_path = format!("{}/{}", out_path, write_path.display());

    if let Some(parent) = Path::new(&write_path).parent() {
        fs::create_dir_all(parent).expect("Failed to create directory");
    }

    let mut file = File::create(&write_path).expect("Failed to create file");
    file.write_all(html.as_bytes()).expect("Failed to write to file");
}
