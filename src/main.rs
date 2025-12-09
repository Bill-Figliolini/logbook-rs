use anyhow::{Result, bail};

use std::{fs::File, io::Write};

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();

    if args.is_empty() {
        bail!("Usage: logbook <Message>")
    }
    let path = "logbook.txt";
    let mut logbook = File::options().create(true).append(true).open(path)?;
    writeln!(logbook, "{}", args.join(" "))?;
    Ok(())
}
