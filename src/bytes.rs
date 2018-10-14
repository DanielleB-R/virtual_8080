pub fn assemble_word(h: u8, l: u8) -> u16 {
    u16::from(h) << 8 | u16::from(l)
}

pub fn high_order_byte(n: u16) -> u8 {
    (n >> 8) as u8
}

pub fn low_order_byte(n: u16) -> u8 {
    (n & 0xff) as u8
}

pub fn rotate_right(n: u8) -> u8 {
    ((n & 0x01) << 7) | (n >> 1)
}

pub fn rotate_left(n: u8) -> u8 {
    ((n & 0x80) >> 7) | (n << 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assemble_word_test() {
        assert_eq!(assemble_word(0, 0), 0);
        assert_eq!(assemble_word(0, 0x23), 0x23);
        assert_eq!(assemble_word(0xa8, 0), 0xa800);
        assert_eq!(assemble_word(0x75, 0x3f), 0x753f);
    }

    #[test]
    fn high_order_byte_test() {
        assert_eq!(high_order_byte(0), 0);
        assert_eq!(high_order_byte(0x23), 0);
        assert_eq!(high_order_byte(0xa800), 0xa8);
        assert_eq!(high_order_byte(0x753f), 0x75);
    }

    #[test]
    fn low_order_byte_test() {
        assert_eq!(low_order_byte(0), 0);
        assert_eq!(low_order_byte(0x23), 0x23);
        assert_eq!(low_order_byte(0xa800), 0);
        assert_eq!(low_order_byte(0x753f), 0x3f);
    }

    #[test]
    fn rotate_right_test() {
        assert_eq!(rotate_right(0), 0);
        assert_eq!(rotate_right(0x01), 0x80);
        assert_eq!(rotate_right(0x80), 0x40);
        assert_eq!(rotate_right(0x44), 0x22);
        assert_eq!(rotate_right(0xff), 0xff);
    }

    #[test]
    fn rotate_left_test() {
        assert_eq!(rotate_left(0), 0);
        assert_eq!(rotate_left(0x01), 0x02);
        assert_eq!(rotate_left(0x80), 0x01);
        assert_eq!(rotate_left(0x44), 0x88);
        assert_eq!(rotate_left(0xff), 0xff);
    }
}
