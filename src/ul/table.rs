pub fn make_hline(col_sizes: &Vec<usize>) -> String {
    let mut buff = vec![0x2b];

    for size in col_sizes {
        // including one byte of padding on each side
        buff.extend_from_slice(&vec![0x2d; size + 2]);
        buff.push(0x2b);
    }

    String::from_utf8(buff).unwrap()
}

pub fn make_row(cols: &Vec<&str>, col_sizes: &Vec<usize>) -> String {
    // opening vline
    let mut buff = vec![0x7c];

    for (col, size) in cols.iter().zip(col_sizes.iter()) {
        // one byte of left padding (whitespace)
        buff.push(0x20);

        let padding_size = size - col.len();
        let padding = vec![0x20; padding_size];

        buff.extend_from_slice(&String::from(*col).into_bytes());
        buff.extend_from_slice(&padding);

        // one byte of right padding (whitespace) and closing vline
        buff.push(0x20);
        buff.push(0x7c);
    }

    String::from_utf8(buff).unwrap()
}
