use converter::ImageToBrailleConverter;

mod braille;
mod converter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let file_path = match &args[..] {
        [_, file_path, ..] => file_path,
        _ => {
            eprintln!("Usage: {} <file_path> [width]", args[0]);
            std::process::exit(1);
        }
    };

    let width = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(60);

    let converter = ImageToBrailleConverter::from_file(file_path)?.resize(width);

    let mut stdout = std::io::stdout().lock();
    converter.convert(&mut stdout)?;

    Ok(())
}
