pub fn parse_to_string(game_buff: &[u8], start: usize, size: usize) -> String {
    let mut buffer = vec![0; size];
    buffer.copy_from_slice(&game_buff[start..start + size]);

    // Strip buffer from any null bytes
    let buffer = buffer
        .into_iter()
        .filter(|byte| *byte != 0x00)
        .collect::<Vec<u8>>();

    String::from_utf8(buffer).unwrap()
}

pub fn compose_from_str(string: &str, size: usize) -> Vec<u8> {
    let mut buff = String::from(string).into_bytes();
    let padding_len = size - buff.len();
    let padding = vec![0x00; padding_len];
    buff.extend_from_slice(&padding);

    buff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_string() {
        let buffer = vec![0x66, 0x6f, 0x6f, 0x00, 0x00];
        let parsed = parse_to_string(&buffer, 0, 5);
        assert_eq!(parsed, String::from("foo"));
    }

    #[test]
    fn test_compose_from_string() {
        let bytes = compose_from_str("foo", 5);
        let expected = vec![0x66, 0x6f, 0x6f, 0x00, 0x00];
        let matching = expected.iter().zip(bytes.iter()).all(|(x, y)| x == y);
        assert!(matching);
    }
}
