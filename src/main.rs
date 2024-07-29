use colored::*;
use std::env;
use std::fs;

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
        let mut icon = String::from("    ").white();

        let file = file.unwrap();

        match file.file_name().into_string().unwrap().split(".").last() {
            Some("rs") => icon = String::from("  󱘗  ").white(),
            Some("md") => icon = String::from("    ").white(),
            Some("sh") => icon = String::from("    ").white(),
            Some("json") => icon = String::from("    ").white(),
            Some("yaml") => icon = String::from("    ").white(),
            Some("toml") => icon = String::from("    ").white(),
            Some("yml") => icon = String::from("    ").white(),
            Some("lock") => icon = String::from("    ").white(),
            Some("py") => icon = String::from("  󰌠  ").white(),
            Some("pyc") => icon = String::from("  󰌠  ").white(),
            Some("html") => icon = String::from("    ").white(),
            Some("css") => icon = String::from("    ").white(),
            Some("js") => icon = String::from("    ").white(),
            Some("ts") => icon = String::from("    ").white(),
            Some("jsx") => icon = String::from("    ").white(),
            Some("tsx") => icon = String::from("    ").white(),
            Some("java") => icon = String::from("    ").white(),
            Some("c") => icon = String::from("    ").white(),
            Some("cpp") => icon = String::from("    ").white(),
            Some("h") => icon = String::from("    ").white(),
            Some("hpp") => icon = String::from("    ").white(),
            Some("go") => icon = String::from("  󰟓  ").white(),
            Some("rb") => icon = String::from("    ").white(),
            Some("php") => icon = String::from("    ").white(),
            Some("lua") => icon = String::from("    ").white(),
            Some("sql") => icon = String::from("    ").white(),
            Some("swift") => icon = String::from("    ").white(),
            Some("psd") => icon = String::from("    ").white(),
            Some("ai") => icon = String::from("    ").white(),
            Some("pdf") => icon = String::from("  󰈦  ").white(),
            Some("png") => icon = String::from("    ").white(),
            Some("jpg") => icon = String::from("    ").white(),
            Some("jpeg") => icon = String::from("    ").white(),
            Some("gif") => icon = String::from("    ").white(),
            Some("svg") => icon = String::from("    ").white(),
            Some("webp") => icon = String::from("    ").white(),
            Some("mp3") => icon = String::from("  󰝚  ").white(),
            Some("wav") => icon = String::from("  󰝚  ").white(),
            Some("flac") => icon = String::from("  󰝚  ").white(),
            Some("ogg") => icon = String::from("  󰝚  ").white(),
            Some("m4a") => icon = String::from("  󰝚  ").white(),
            Some("opus") => icon = String::from("  󰝚  ").white(),

            Some(&_) => (),
            None => (),
        }

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
