use std::{error::Error, process, sync::Arc};

use clap::Parser;
use tokio::{runtime::Runtime, signal};

pub mod ui;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(
        short,
        long,
        value_parser,
        default_value = "~/.config/spaceranger/config"
    )]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _args = Args::parse();

    let ui = Arc::new(ui::ui::UI::new());
    ui.setup();
    ui::ui::start_rendering(ui.clone());

    // wait for ctrl c
    signal::ctrl_c().await?;
    ui.cleanup();
    process::exit(0);
}
