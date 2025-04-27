mod cli;
pub mod error;
mod input;

pub(crate) mod replacer;
mod unescape;

use clap::Parser;
use std::process;

use sd::sd;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

fn try_main() -> Result<(), sd::error::Error> {
    let options = cli::Options::parse();

    sd(
        options.preview,
        options.literal_mode,
        options.replacements,
        options.flags,
        options.find,
        options.replace_with,
        options.files,
    )
}
