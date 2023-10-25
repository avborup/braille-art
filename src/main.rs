mod braille;
mod converter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = converter::ImageToBrailleConverter::from_file("goose.jpg")?.resize(60);

    let mut stdout = std::io::stdout().lock();
    converter.convert(&mut stdout)?;

    Ok(())
}
