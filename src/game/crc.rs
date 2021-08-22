const CRC_TABLE_SIZE: i32 = 256;

fn initialize_crc_table(crc_table: &mut Vec<i32>) {
    for index in 0..CRC_TABLE_SIZE {
        let mut crc = index << 24;
        for _ in 0..8 {
            if crc < 0 {
                crc = crc << 1;
            } else {
                crc = (crc << 1) ^ 0x04c11db7;
            }
        }

        let index_le = CRC_TABLE_SIZE - index - 1;
        crc_table[index_le as usize] = crc;
    }
}

#[allow(overflowing_literals)]
pub fn get_game_name_crc(name: &String) -> String {
    let mut crc_table = vec![0; CRC_TABLE_SIZE as usize];
    initialize_crc_table(&mut crc_table);

    let mut name_bytes = String::from(name).into_bytes();
    name_bytes.push(0x00); // USBExtreme format expect null byte at the end
    let mut crc = 0;

    for byte in name_bytes {
        let index = byte as i32 ^ ((crc >> 24) & 0xff);
        crc = crc_table[index as usize] ^ ((crc << 8) & 0xffffff00); // overflow on purpose
    }

    format!("{:X}", crc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_allowed_lowercase() {
        let name = String::from("f");
        let name_crc = get_game_name_crc(&name);
        assert_eq!(name_crc, String::from("8433E5CC"));
    }

    #[test]
    fn test_shortest_allowed_uppercase() {
        let name = String::from("F");
        let name_crc = get_game_name_crc(&name);
        assert_eq!(name_crc, String::from("A490D3EA"));
    }

    #[test]
    fn test_longest_allowed_lowercase() {
        let name = String::from("fooooooooooooooooooooooooooooooo");
        let name_crc = get_game_name_crc(&name);
        assert_eq!(name_crc, String::from("84BA9D95"));
    }

    #[test]
    fn test_longest_allowed_uppercase() {
        // really shouting now
        let name = String::from("FOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO");
        let name_crc = get_game_name_crc(&name);
        assert_eq!(name_crc, String::from("8CAF0142"));
    }
}
