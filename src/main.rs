use std::error::Error;

use clap::Parser;
use tokio::runtime::Runtime;

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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // start tokio runtime
    let rt = Runtime::new()?;
    let _guard = rt.enter();

    let should_render = ui::init_renderer();

    Ok(())
}
