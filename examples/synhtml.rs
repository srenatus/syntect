//! Prints highlighted HTML for a file to stdout.
//! Basically just wraps a body around `highlighted_html_for_file`
extern crate syntect;
extern crate getopts;

use getopts::Options;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Color, ThemeSet};
use syntect::html::highlighted_html_for_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = Options::new();
    opts.optopt("s", "extra-syntaxes", "SYNTAX_FOLDER", "Additional folder to search for .sublime-syntax files in.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let mut ss = SyntaxSet::load_defaults_nonewlines();
    if let Some(folder) = matches.opt_str("extra-syntaxes") {
        let mut builder = ss.into_builder();
        builder.add_from_folder(folder, false).unwrap();
        ss = builder.build();
    }

    if matches.free.len() == 0 {
        println!("Please pass in a file to highlight");
        return;
    }
    let file = &matches.free[0];

    let ts = ThemeSet::load_defaults();

    let style = "
        pre {
            font-size:13px;
            font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
        }";
    println!("<head><title>{}</title><style>{}</style></head>", file, style);
    let theme = &ts.themes["base16-ocean.dark"];
    let c = theme.settings.background.unwrap_or(Color::WHITE);
    println!("<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n", c.r, c.g, c.b);
    let html = highlighted_html_for_file(file, &ss, theme).unwrap();
    println!("{}", html);
    println!("</body>");
}
