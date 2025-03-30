const CHARS: [char; 38] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '-', '.',
];

pub(crate) fn encode(bytes: &[u8]) -> String {
    bytes
        .chunks(3)
        .map(|bytes| match bytes.len() {
            1 => do_encode(bytes[0] as u32, 2),
            2 => do_encode(((bytes[1] as u32) << 8) | (bytes[0] as u32), 4),
            3 => do_encode(
                ((bytes[2] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[0] as u32),
                5,
            ),
            _ => unreachable!(),
        })
        .fold(String::new(), |acc, s| acc + &s)
}

fn do_encode(bytes: u32, len: usize) -> String {
    let mut res = String::new();
    let mut bytes = bytes;

    for _ in 0..len {
        let index = bytes % 38;
        res.push(CHARS[index as usize]);

        bytes /= 38;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base38_encode() {
        assert_eq!(encode(&vec![0xde, 0xad, 0xbe, 0xef]), "C.R.5B6");
        assert_eq!(encode(&vec![]), "");
        assert_eq!(encode(&vec![0x00]), "00");
        assert_eq!(encode(&vec![0xff, 0xff, 0xff, 0xff]), "PLS18R6");
    }
}
