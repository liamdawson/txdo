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
        let (date2, buffer) = matchers::parse_date(buffer);

        let completed_at = match completed {
            true => date1,
            false => None
        };

        let created_at = match completed {
            true => date2,
            false => date1
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
        ( $val:expr ) => { TodoItem::parse($val.as_bytes()) }
    }

    // start by testing descriptions, as they're a quick indicator that prefix
    // parsing is broken

    #[test]
    fn extracts_description_from_basic_item() {
        const SAMPLE: &str = "water cat";

        assert_eq!(parse_sample!(SAMPLE).description, SAMPLE);
    }

    #[test]
    fn extracts_description_from_finished_item() {
        const DESCRIPTION: &str = "hang sloths up to dry";
        const SAMPLE: &str = "x hang sloths up to dry";

        assert_eq!(parse_sample!(SAMPLE).description, DESCRIPTION);
    }

    #[test]
    fn extracts_description_from_item_with_priority() {
        const DESCRIPTION: &str = "reticulate splines";
        const SAMPLE: &str = "(A) reticulate splines";

        assert_eq!(parse_sample!(SAMPLE).description, DESCRIPTION);
    }

    #[test]
    fn extracts_description_from_item_with_creation_date() {
        const DESCRIPTION: &str = "make new year's resolutions";
        const SAMPLE: &str = "2019-01-01 make new year's resolutions";

        assert_eq!(parse_sample!(SAMPLE).description, DESCRIPTION);
    }

    #[test]
    fn extracts_description_from_item_with_completion_date() {
        const DESCRIPTION: &str = "write a novel";
        const SAMPLE: &str = "x 2018-11-30 write a novel";

        assert_eq!(parse_sample!(SAMPLE).description, DESCRIPTION);
    }

    #[test]
    fn extracts_description_from_item_with_completion_and_creation_date() {
        const DESCRIPTION: &str = "reach 25 years old";
        const SAMPLE: &str = "x 2025-01-01 2000-01-01 reach 25 years old";

        assert_eq!(parse_sample!(SAMPLE).description, DESCRIPTION);
    }
}
