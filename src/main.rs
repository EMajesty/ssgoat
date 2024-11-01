use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use colored::Colorize;

// use clap::Parser;

use comrak::{markdown_to_html, Options};
use glob::glob;

// /// A simple static site generator
// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     /// Input path
//     #[arg(short, long)]
//     input: String,
//
//     /// Output path
//     #[arg(short, long)]
//     output: String,
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    let in_path;
    let out_path;
    let in_folder_structure = "/**/*.md";

    match args.len() {
        3 => {
            in_path = &args[1];
            out_path = &args[2];
        }
        _ => {
            println!("{}", "Give input and output paths as arguments".red());
            return;
        }
    }

    let search_path = format!("{}{}", in_path, in_folder_structure);
    let mut file_list: Vec<PathBuf> = Vec::new();

    for entry in glob(&search_path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if !is_hidden(&path) {
                    file_list.push(path);
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("{}", "> Converting files".blue());

    let sidebar = create_sidebar(&file_list);

    for file in &file_list {
        println!("{}", file.display());

        let mut html = convert_file(&file);

        let header = create_header(&html, &in_path);
        let footer = create_footer();
        html = format!("{}{}{}{}", &header, &sidebar, &html, &footer);
        write_file(html, file, &out_path);
    }

    println!("{}", "> Collecting resources".blue());
    collect_resources(&in_path, &out_path);

    println!("{}", "> Done".green());
}

fn convert_file(path: &Path) -> String {
    let markdown = fs::read_to_string(path).expect("Failed to read file");
    let mut html = markdown_to_html(&markdown, &Options::default());
    html.insert_str(0, "<div class=\"main-content\">");
    html.push_str("</div>");
    return html;
}

fn create_header(html: &String, in_path: &str) -> String {
    let mut title = ' '.into();

    match find_title(html) {
        Some(content) => title = content,
        None => println!("No h1 tag found in file {}", &in_path),
    }

    let mut header_html = String::from("<!DOCTYPE html><html><head><title>");
    header_html.push_str(&title);
    header_html.push_str("</title>");
    header_html.push_str("<link rel=\"stylesheet\" href=\"");
    header_html.push_str("style.css\">");
    header_html.push_str("</head><body>");
    header_html.push_str("<div class=\"header\">");
    // header_html.push_str(&format!("<h1>{}</h1>", &title));
    header_html.push_str("</div>");
    header_html.push_str("<div class=\"container\">");
    header_html
}

fn create_sidebar(file_list: &Vec<PathBuf>) -> String {
    let mut sidebar_html = String::from("<div class=\"sidebar\"><ul>");

    for file in file_list {
        let file_name = file.with_extension("html");
        let file_name = file_name.file_name().unwrap().to_str().unwrap();
        sidebar_html.push_str(&format!(
            "<li><a href=\"{}\">{}</a></li>",
            file_name,
            file_name.strip_suffix(".html").unwrap()
        ));
    }

    sidebar_html.push_str("</ul></div>");
    sidebar_html
}

fn create_footer() -> String {
    let mut footer_html = String::from("</div>");
    footer_html.push_str("</body></html>");
    footer_html
}

fn write_file(html: String, path: &Path, out_path: &str) {
    let file_name = path.with_extension("html");
    let file_name = file_name.file_name().unwrap().to_str().unwrap();
    let write_path = format!("{}/{}", out_path, file_name);

    let mut file = File::create(write_path).expect("Failed to create file");
    file.write_all(html.as_bytes())
        .expect("Failed to write to file");
}

fn collect_resources(in_path: &str, out_path: &str) {
    let write_path = format!("{}/", out_path);
    for extension in &[
        "jpg", "png", "gif", "bmp", "svg", "css", "ttf", "otf", "woff", "woff2",
    ] {
        let search_path = format!("{}{}{}", in_path, "/**/*.", extension);
        for entry in glob(&search_path).expect("Failed to read glob pattern") {
            match entry {
                Ok(file) => {
                    if !is_hidden(&file) {
                        println!("{}", file.display());
                        let file_name = file.file_name().unwrap();
                        let destination_path =
                            format!("{}{}", write_path, file_name.to_str().unwrap());
                        fs::copy(file, destination_path).unwrap();
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}

fn is_hidden(path: &PathBuf) -> bool {
    path.iter().any(|component| {
        component
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    })
}

fn find_title(html: &str) -> Option<String> {
    let start = html.find("<h1")?;
    let end = html[start..].find("</h1>")?;
    let title_start = html[start..].find('>')?;
    Some(html[start + title_start + 1..start + end].trim().to_string())
}
