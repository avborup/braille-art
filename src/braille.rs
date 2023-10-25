/// Convert a 4-row, 2-column chunk of pixels to a Braille character where each
/// true entry is mapped to a dot and each false entry is mapped to empty space.
pub fn chunk_to_braille(chunk: [[bool; 2]; 4]) -> char {
    let ordering = [
        chunk[0][0],
        chunk[1][0],
        chunk[2][0],
        chunk[0][1],
        chunk[1][1],
        chunk[2][1],
        chunk[3][0],
        chunk[3][1],
    ];

    let codepoint_offset: u32 = ordering
        .iter()
        .enumerate()
        .map(|(i, &pixel)| (pixel as u32) << i)
        .sum();

    let codepoint = 0x2800 + codepoint_offset;

    std::char::from_u32(codepoint).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_dots_are_correct_location() {
        let mut cases = vec![];
        for i in 0..=7 {
            let mut chunk = [[false; 2]; 4];
            chunk[i / 2][i % 2] = true;
            cases.push(chunk_to_braille(chunk));
        }

        assert_eq!(cases, ['⠁', '⠈', '⠂', '⠐', '⠄', '⠠', '⡀', '⢀']);
    }

    #[test]
    fn single_dots_inverted_are_correct_location() {
        let mut cases = vec![];
        for i in 0..=7 {
            let mut chunk = [[true; 2]; 4];
            chunk[i / 2][i % 2] = false;
            cases.push(chunk_to_braille(chunk));
        }

        assert_eq!(cases, ['⣾', '⣷', '⣽', '⣯', '⣻', '⣟', '⢿', '⡿']);
    }

    #[test]
    fn some_random_chunks() {
        assert_eq!(
            chunk_to_braille([[true, true], [false, true], [false, false], [true, true]]),
            '⣙'
        );
        assert_eq!(
            chunk_to_braille([[false, true], [true, false], [false, true], [true, false]]),
            '⡪'
        );
        assert_eq!(
            chunk_to_braille([[false, true], [false, true], [true, false], [true, false]]),
            '⡜'
        );
    }
}
