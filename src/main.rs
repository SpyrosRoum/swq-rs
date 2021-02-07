mod gif;
mod quote;

use clipboard_ext::{clipboard::ClipboardProvider, x11_fork::ClipboardContext};

use anyhow::{anyhow, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opt {
    /// Retrieves Star Wars Quotes
    Quote { keywords: String },
    /// Retrieves Star Wars GIFs
    Gif { keywords: String },
}

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();

    let r = match opt {
        Opt::Quote { keywords } => quote::get_quote(keywords.as_str()),
        Opt::Gif { keywords } => gif::get_gif(keywords.as_str()),
    }?;

    println!("{}\n", r);

    let mut ctx: ClipboardContext =
        ClipboardProvider::new().map_err(|_| anyhow!("Failed to open clipboard"))?;
    ctx.set_contents(r)
        .map_err(|_| anyhow!("Failed to set clipboard contents"))?;

    println!("Added to your clipboard!");

    Ok(())
}
