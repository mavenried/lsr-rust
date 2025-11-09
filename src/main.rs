use colored::*;
use std::env;
use std::fs;

mod get_icon;

use get_icon::get_icon;
fn main() {
    let terminal_size = termsize::get().unwrap();
    let current_dir = env::current_dir().unwrap();
    let args: Vec<String> = env::args().collect();
    println!("{}", "-=".repeat(terminal_size.cols as usize / 2));
    print_dir_contents(&current_dir, 0, &args);
    println!("{}", "-=".repeat(terminal_size.cols as usize / 2));
}

fn print_dir_contents(dir: &std::path::Path, indent: u32, args: &Vec<String>) {
    for file in fs::read_dir(dir).unwrap() {
        let file = file.unwrap();

        let mut icon = get_icon(file.file_name().into_string().unwrap().split(".").last());

        let out = file.file_name().into_string().unwrap();
        let mut out = out.split("/").last().unwrap().to_string();

        let indent_str = " ▏".repeat(indent as usize);

        if file.file_type().unwrap().is_dir() {
            icon = String::from("   ").bold().yellow();
            out = format!("{}{}{}", indent_str.black(), icon.bold(), out.bold());
        } else {
            out = format!("{}{}{}", indent_str.black(), icon, out);
        }

        if !out.starts_with(".") || args.contains(&String::from("-a")) {
            println!("{}", out);

            if file.file_type().unwrap().is_dir() {
                let next_indent = indent + 1;
                print_dir_contents(&file.path(), next_indent, args);
            }
        }
    }
}
