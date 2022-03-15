pub fn make_hline(col_sizes: &[usize]) -> String {
    let mut buff = vec![0x2b];

    for size in col_sizes {
        // including one byte of padding on each side
        buff.extend_from_slice(&vec![0x2d; size + 2]);
        buff.push(0x2b);
    }

    String::from_utf8(buff).unwrap()
}

pub fn make_row(cols: Vec<String>, col_sizes: &[usize]) -> String {
    // opening vline
    let mut buff = vec![0x7c];

    for (col, size) in cols.iter().zip(col_sizes.iter()) {
        // one byte of left padding (whitespace)
        buff.push(0x20);

        let padding_size = size - col.len();
        let padding = vec![0x20; padding_size];

        buff.extend_from_slice(&col.clone().into_bytes());
        buff.extend_from_slice(&padding);

        // one byte of right padding (whitespace) and closing vline
        buff.push(0x20);
        buff.push(0x7c);
    }

    String::from_utf8(buff).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_row() {
        let cols = vec![String::from("foo"), String::from("bar")];
        let sizes = [3, 4];
        let row = make_row(cols, &sizes);
        assert_eq!(row, String::from("| foo | bar  |"));
    }

    #[test]
    fn test_make_hline() {
        let sizes = [3, 4, 5];
        let hline = make_hline(&sizes);
        assert_eq!(hline, String::from("+-----+------+-------+"));
    }
}
