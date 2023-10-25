mod braille;

fn main() {
    let chunk = [[true, true], [false, true], [false, false], [true, true]];

    println!("{}", braille::chunk_to_braille(chunk));
}
