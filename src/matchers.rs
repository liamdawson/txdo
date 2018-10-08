pub fn parse_completed(buffer: &[u8]) -> (bool, &[u8]) {
    if buffer.len() > 1 && buffer[0] == ('x' as u8) && buffer[1] == (' ' as u8) {
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
        && buffer[1].is_ascii_uppercase()
    {
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
            && buffer[DATE_LENGTH] == (' ' as u8)
        {
            return (Some(&buffer[0..DATE_LENGTH]), &buffer[DATE_LENGTH + 1..]);
        }
    }

    (None, buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str;

    macro_rules! input {
        ($val:expr) => {
            $val.as_bytes()
        };
    }

    fn coerce_to_string(bytes: &[u8]) -> &str {
        str::from_utf8(bytes).unwrap()
    }

    #[test]
    pub fn completed_cases() {
        assert_eq!(parse_completed(input!("x ")).0, true);

        // must have a space following
        assert_eq!(parse_completed(input!("x")).0, false);
        assert_eq!(parse_completed(input!("xylophone")).0, false);
        // can't be preceded by a space
        assert_eq!(parse_completed(input!(" x")).0, false);
        // can't be uppercase
        assert_eq!(parse_completed(input!("X ")).0, false);
    }

    #[test]
    pub fn priority_cases() {
        assert_eq!(parse_priority(input!("(A) ")).0, Some('A' as u8));
        assert_eq!(parse_priority(input!("(X) ")).0, Some('X' as u8));

        // can't be lowercase
        assert_eq!(parse_priority(input!("(c) ")).0, None);
        // can't be preceded by a space
        assert_eq!(parse_priority(input!(" (c)")).0, None);
        assert_eq!(parse_priority(input!(" (O).(o)")).0, None);
    }

    #[test]
    pub fn date_cases() {
        // TODO: fix and test non-date formats
        //       e.g. 2018-99-99

        assert_eq!(
            parse_date(input!("2018-01-01 ")).0.map(coerce_to_string),
            Some("2018-01-01")
        );

        // this should pass
        // assert_eq!(parse_date(input!("2018-99-99 ")).0.map(coerce_to_string), None);

        // dates should be followed by spaces
        assert_eq!(
            parse_date(input!("2018-01-01")).0.map(coerce_to_string),
            None
        );

        // dates must use four/two/two-digit components
        assert_eq!(
            parse_date(input!("18-01-01 ")).0.map(coerce_to_string),
            None
        );
        assert_eq!(
            parse_date(input!("2018-1-01 ")).0.map(coerce_to_string),
            None
        );
        assert_eq!(
            parse_date(input!("2018-01-1 ")).0.map(coerce_to_string),
            None
        );

        // dates must use hyphens as separators
        assert_eq!(
            parse_date(input!("2018_01_01 ")).0.map(coerce_to_string),
            None
        );
    }
}
