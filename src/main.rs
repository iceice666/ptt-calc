mod cli;
mod score;
mod tui;
use anyhow::Result as anyResult;
use std::{collections::VecDeque, env};

fn main() -> anyResult<()> {
    let mut args: VecDeque<String> = env::args().collect();
    let _ = args.remove(0);

    // determine which mode to run
    match args.get(0) {
        None => {}
        Some(mode) => {
            if mode == "list" || mode == "b30" {
                let _ = crate::cli::main(args);
            } else if mode == "tui" {
                let _ = crate::tui::main();
            }
        }
    };

    Ok(())
}
