pub fn parse_to_string(game_buff: &[u8], start: usize, end: usize) -> String {
    let mut buffer = vec![0; end - start];
    buffer.copy_from_slice(&game_buff[start..end]);

    // Strip buffer from any null bytes
    let buffer = buffer.into_iter()
        .filter(|byte| *byte != 0x00)
        .collect::<Vec<u8>>();
        
    String::from_utf8(buffer).unwrap()
}

pub fn compose_from_str(string: &String, size: usize) -> Vec<u8> {
    let mut buff = String::from(string).into_bytes();
    let padding_len = size - &buff.len();
    let padding = vec![0x00; padding_len];
    buff.extend_from_slice(&padding);
    
    buff
}