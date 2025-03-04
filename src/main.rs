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

// 実行方法:
//     cargo run -- [検索するルートディレクトリ] [--file-ext "拡張子,拡張子"]
// 例: cargo run -- src --file-ext "rs,txt"
// TODO: clean
fn main() -> io::Result<()> {
    let args = Args::parse();
    let ext_filter: Vec<String> = if let Some(exts) = args.ext {
        exts.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        vec![] // all files
    };

    let mut terminal = ratatui::init();
    let app_result = App::new(&args.root, &ext_filter).run(&mut terminal);

    ratatui::restore();
    app_result
}
