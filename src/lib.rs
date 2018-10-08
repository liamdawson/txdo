mod matchers;

use std::str;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TodoItem<'a> {
    pub completed: bool,
    pub priority: Option<u8>,
    pub completed_at: Option<&'a [u8]>,
    pub created_at: Option<&'a [u8]>,
    pub description: &'a str,
}

impl<'a> TodoItem<'a> {
    pub fn parse(src: &'a [u8]) -> Self {
        let buffer = src;
        let (completed, buffer) = matchers::parse_completed(buffer);
        let (priority, buffer) = matchers::parse_priority(buffer);
        let (date1, buffer) = matchers::parse_date(buffer);
        let (date2, buffer) = match completed {
            true => matchers::parse_date(buffer),
            false => (None, buffer),
        };

        let completed_at = match completed {
            true => date1,
            false => None,
        };

        let created_at = match completed {
            true => date2,
            false => date1,
        };

        let description = str::from_utf8(buffer).unwrap();

        TodoItem {
            completed,
            priority,
            completed_at,
            created_at,
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parse_sample {
        ( $val:expr ) => {
            TodoItem::parse($val.as_bytes())
        };
    }

    // start by testing descriptions, as they're a quick indicator that prefix
    // parsing is broken

    #[test]
    fn correctly_handles_two_dates_when_not_completed() {
        const DESCRIPTION: &str = "2018-12-31 water cat";
        const SAMPLE: &str = "2018-01-01 2018-12-31 water cat";
        let expected_date: &[u8] = "2018-01-01".as_bytes();

        assert_eq!(parse_sample!(SAMPLE).created_at, Some(expected_date));
        assert_eq!(parse_sample!(SAMPLE).description, DESCRIPTION);
    }

    #[test]
    fn correctly_handles_two_dates_when_completed() {
        const DESCRIPTION: &str = "water cat";
        const SAMPLE: &str = "x 2018-01-01 2018-12-31 water cat";
        let expected_completed_date: &[u8] = "2018-01-01".as_bytes();
        let expected_created_date: &[u8] = "2018-12-31".as_bytes();

        assert_eq!(
            parse_sample!(SAMPLE).created_at,
            Some(expected_created_date)
        );
        assert_eq!(
            parse_sample!(SAMPLE).completed_at,
            Some(expected_completed_date)
        );
        assert_eq!(parse_sample!(SAMPLE).description, DESCRIPTION);
    }
}
