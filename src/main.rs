use app::App;
use clap::Parser;
use std::io::{self};
mod app;
mod directory;
mod ui;
use directory::list_dir_items;
use ratatui::widgets::ListState;

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
    let root = args.root;
    let base = root.clone();
    let mut current_dir = base.clone();
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    let ext_filter: Vec<String> = if let Some(exts) = args.ext {
        exts.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        vec![] // all files
    };

    let ext_filter_ref: Vec<&str> = ext_filter.iter().map(|s| s.as_str()).collect();
    let items = list_dir_items(&current_dir, &ext_filter_ref)?;

    let mut terminal = ratatui::init();
    let app_result = App::new(root.clone(), base, items).run(&mut terminal);

    //     draw_ui(&mut terminal, &current_dir, &mut list_state, &items)?;
    //     if let Some(next_dir) = handle_event(&current_dir, &base, &mut list_state, &items) {
    //         if next_dir == "__exit__" {
    //             break;
    //         }
    //         current_dir = next_dir;
    //         list_state.select(Some(0));
    //     }
    // }

    ratatui::restore();
    app_result
}
