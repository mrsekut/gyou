use app::App;
use clap::Parser;
use std::io::{self};
mod app;
mod directory;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(default_value = "src")]
    root: String,

    #[clap(
        long = "ext",
        short = 'e',
        help = "File extension filter (comma separated), e.g., ts,tsx"
    )]
    ext: Option<String>,
}

// Usage:
//     cargo run -- [root directory to search] [--ext "extension"]
// Example: cargo run -- src --ext "rs,txt"
fn main() -> io::Result<()> {
    let args = Args::parse();

    let exts = match &args.ext {
        Some(e) => e.split(',').map(|s| s.trim().to_string()).collect(),
        None => vec![], // all files
    };

    let mut terminal = ratatui::init();
    let app_result = App::new(&args.root, &exts).run(&mut terminal);

    ratatui::restore();
    app_result
}
