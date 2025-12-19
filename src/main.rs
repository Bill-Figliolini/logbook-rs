use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();

    let path = "logbook.txt";
    if args.is_empty() {
        let contents = logbook_rs::read(path)?;
        match contents {
            Some(contents) => print!("{}", contents),
            None => println!("Empty Logbook"),
        }
    } else {
        let text = args.join(" ");
        logbook_rs::append(path, text)?;
    }
    Ok(())
}
