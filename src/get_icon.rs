use colored::{ColoredString, Colorize};

pub fn get_icon(ext: Option<&str>) -> ColoredString {
    let icon;
    match ext {
        Some("rs") => icon = String::from("  󱘗  ").white(),
        Some("md") => icon = String::from("    ").white(),
        Some("sh") => icon = String::from("    ").white(),
        Some("bash") => icon = String::from("    ").white(),
        Some("zsh") => icon = String::from("    ").white(),
        Some("fish") => icon = String::from("    ").white(),
        Some("json") => icon = String::from("    ").white(),
        Some("yaml") | Some("yml") | Some("toml") | Some("lock") => {
            icon = String::from("    ").white()
        }
        Some("xml") => icon = String::from("  󰗀  ").white(),
        Some("ini") => icon = String::from("  󰏗  ").white(),
        Some("conf") => icon = String::from("  󰏗  ").white(),

        // Programming Languages
        Some("py") | Some("pyc") | Some("pyo") => icon = String::from("  󰌠  ").white(),
        Some("html") => icon = String::from("    ").white(),
        Some("css") => icon = String::from("    ").white(),
        Some("scss") => icon = String::from("    ").white(),
        Some("less") => icon = String::from("    ").white(),
        Some("js") => icon = String::from("    ").white(),
        Some("mjs") => icon = String::from("    ").white(),
        Some("cjs") => icon = String::from("    ").white(),
        Some("ts") => icon = String::from("    ").white(),
        Some("jsx") | Some("tsx") => icon = String::from("    ").white(),
        Some("vue") => icon = String::from("  󰡄  ").white(),
        Some("svelte") => icon = String::from("    ").white(),
        Some("astro") => icon = String::from("    ").white(),
        Some("java") => icon = String::from("    ").white(),
        Some("c") => icon = String::from("    ").white(),
        Some("cpp") | Some("cc") | Some("cxx") => icon = String::from("    ").white(),
        Some("h") | Some("hpp") | Some("hh") => icon = String::from("  󰙱  ").white(),
        Some("go") => icon = String::from("  󰟓  ").white(),
        Some("rb") => icon = String::from("    ").white(),
        Some("php") => icon = String::from("    ").white(),
        Some("lua") => icon = String::from("    ").white(),
        Some("pl") => icon = String::from("    ").white(),
        Some("r") => icon = String::from("  󰟔  ").white(),
        Some("dart") => icon = String::from("    ").white(),
        Some("kt") | Some("kts") => icon = String::from("    ").white(),
        Some("swift") => icon = String::from("    ").white(),
        Some("zig") => icon = String::from("    ").white(),
        Some("nim") => icon = String::from("    ").white(),
        Some("scala") => icon = String::from("    ").white(),
        Some("erl") | Some("hrl") => icon = String::from("    ").white(),
        Some("ex") | Some("exs") => icon = String::from("    ").white(),
        Some("hs") => icon = String::from("    ").white(),
        Some("ml") | Some("mli") => icon = String::from("    ").white(),
        Some("clj") | Some("cljs") | Some("edn") => icon = String::from("    ").white(),
        Some("elm") => icon = String::from("    ").white(),
        Some("cr") => icon = String::from("    ").white(),
        Some("v") => icon = String::from("    ").white(),
        Some("vala") => icon = String::from("    ").white(),
        Some("fs") | Some("fsi") => icon = String::from("    ").white(),

        // Data / Config
        Some("csv") => icon = String::from("  󰈛  ").white(),
        Some("tsv") => icon = String::from("  󰈛  ").white(),
        Some("sql") => icon = String::from("    ").white(),
        Some("db") | Some("sqlite") | Some("sqlite3") => icon = String::from("    ").white(),

        // Markup / Docs
        Some("txt") => icon = String::from("    ").white(),
        Some("tex") => icon = String::from("  󰙩  ").white(),
        Some("pdf") => icon = String::from("  󰈦  ").white(),
        Some("epub") => icon = String::from("  󰈙  ").white(),
        Some("doc") | Some("docx") => icon = String::from("  󰈬  ").white(),
        Some("odt") => icon = String::from("  󰈬  ").white(),
        Some("rtf") => icon = String::from("  󰈬  ").white(),
        Some("xls") | Some("xlsx") | Some("ods") => icon = String::from("  󰈛  ").white(),
        Some("ppt") | Some("pptx") | Some("odp") => icon = String::from("  󰈧  ").white(),

        // Images
        Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("bmp") | Some("svg")
        | Some("webp") | Some("ico") | Some("tiff") => icon = String::from("    ").white(),
        Some("psd") => icon = String::from("    ").white(),
        Some("ai") => icon = String::from("    ").white(),

        // Audio / Video
        Some("mp3") | Some("wav") | Some("flac") | Some("ogg") | Some("m4a") | Some("opus") => {
            icon = String::from("  󰝚  ").white()
        }
        Some("mp4") | Some("mkv") | Some("avi") | Some("mov") | Some("webm") | Some("flv")
        | Some("wmv") => icon = String::from("  󰕧  ").white(),
        Some("srt") | Some("vtt") => icon = String::from("  󰨖  ").white(),

        // Archives
        Some("zip") | Some("tar") | Some("gz") | Some("bz2") | Some("xz") | Some("7z")
        | Some("rar") => icon = String::from("  󰀼  ").white(),

        // System / DevOps
        Some("makefile") | Some("mk") => icon = String::from("  󰞋  ").white(),
        Some("dockerfile") => icon = String::from("  󰡨  ").white(),
        Some("dockerignore") => icon = String::from("  󰡨  ").white(),
        Some("env") | Some(".env") => icon = String::from("  󰌪  ").white(),
        Some("service") => icon = String::from("  󰓅  ").white(),
        Some("systemd") => icon = String::from("  󰓅  ").white(),
        Some("nix") => icon = String::from("    ").white(),
        Some("pkgbuild") => icon = String::from("    ").white(),
        Some("log") => icon = String::from("  󰌱  ").white(),
        Some("bak") | Some("old") => icon = String::from("  󰁯  ").white(),

        // Web / Misc
        Some("svgz") => icon = String::from("    ").white(),
        Some("woff") | Some("woff2") | Some("ttf") | Some("otf") => {
            icon = String::from("  󰛖  ").white()
        }
        Some("torrent") => icon = String::from("  󰛴  ").white(),
        Some("desktop") => icon = String::from("  󰣇  ").white(),
        Some("appimage") => icon = String::from("    ").white(),
        Some("iso") => icon = String::from("  󰋊  ").white(),
        Some("bin") | Some("exe") => icon = String::from("    ").white(),
        Some("obj") | Some("o") => icon = String::from("    ").white(),
        Some("wasm") => icon = String::from("    ").white(),

        // Version Control
        Some("gitignore") => icon = String::from("  󰊢  ").white(),
        Some("gitmodules") => icon = String::from("  󰊢  ").white(),
        Some("gitattributes") => icon = String::from("  󰊢  ").white(),

        _ => icon = String::from("    ").white(), // default file icon
    }
    return icon;
}
