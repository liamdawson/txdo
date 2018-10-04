pub fn parse_completed(buffer: &[u8]) -> (bool, &[u8]) {
    if buffer.len() > 1
        && buffer[0] == ('x' as u8)
        && buffer[1] == (' ' as u8) {
        (true, &buffer[2..])
    } else {
        (false, &buffer)
    }
}

pub fn parse_priority(buffer: &[u8]) -> (Option<u8>, &[u8]) {
    if buffer.len() > 3
        && buffer[0] == ('(' as u8)
        && buffer[2] == (')' as u8)
        && buffer[3] == (' ' as u8)
        && buffer[1].is_ascii_uppercase() {
        (Some(buffer[1]), &buffer[4..])
    } else {
        (None, &buffer)
    }
}

pub fn parse_date(buffer: &[u8]) -> (Option<&[u8]>, &[u8]) {
    // require full ISO8601 date format:
    // YYYY-MM-DD (10 bytes)
    const DATE_LENGTH: usize = 10;

    fn valid_character(char: u8, pos: usize) -> bool {
        if pos == 4 || pos == 7 {
            char == ('-' as u8)
        } else {
            char.is_ascii_digit()
        }
    }

    if buffer.len() > DATE_LENGTH {
        // TODO: better validation (e.g. reject 9999-99-99)

        if (0..DATE_LENGTH).all(|ind| valid_character(buffer[ind], ind))
            && buffer[DATE_LENGTH] == (' ' as u8) {
            return (Some(&buffer[0..DATE_LENGTH]), &buffer[DATE_LENGTH + 1..]);
        }
    }

    (None, buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! input {
        ($val:expr) => { $val.as_bytes() }
    }

    pub fn completed_cases() {
        assert_eq!(parse_completed(input!("x ")).0, true);

        assert_eq!(parse_completed(input!("x")).0, false);
        assert_eq!(parse_completed(input!("xylophone")).0, false);
        assert_eq!(parse_completed(input!(" x")).0, false);
        assert_eq!(parse_completed(input!("X ")).0, false);
    }
}
