use std::{error::Error, process, sync::Arc};

use clap::Parser;
use tokio::{runtime::Runtime, signal, sync::Mutex};

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

    let ui = Arc::new(Mutex::new(ui::ui::UI::new()));

    // setup
    let mut ui_lock = ui.lock().await;
    ui_lock.setup()?;
    drop(ui_lock);

    // start the rendering task
    ui::ui::start_rendering(ui.clone());

    // wait for ctrl c
    signal::ctrl_c().await?;

    // clean up program
    let ui_lock = ui.lock().await;
    ui_lock.cleanup()?;
    drop(ui_lock);

    // better way to do this??
    process::exit(0);
}
