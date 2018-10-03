
/*
psuedo format rules:

'x '                          - completed
'([A-Z]) '                    - priority
'[0-9]{4}-[0-9]{2}-[0-9]{2} ' - completion date
'[0-9]{4}-[0-9]{2}-[0-9]{2} ' - creation date (mandatory if completion date set)
'[^\n]+'                      - description
*/

fn is_completed(input: String) -> (bool, String) {
    let prefix = input.chars().take(2).collect::<String>();

    if prefix == "x " {
        (true, input.chars().skip(2).collect())
    } else {
        (false, input)
    }
}

fn has_priority(input: String) -> (Option<char>, String) {
    let prefix = input.chars().take(4).collect::<String>();

    if prefix.starts_with("(") && prefix.ends_with(") ") {
        let priority_char = prefix.chars().nth(1).unwrap();

        if priority_char.is_ascii_uppercase() {
            return (Some(priority_char), input.chars().skip(4).collect::<String>());
        }
    }

    (None, input)
}

pub struct TodoItem {
    pub completed: bool,
    pub priority: Option<char>,
    pub description: String,
}

impl TodoItem {
    pub fn parse(src: &str) -> Self {
        let buffer = String::from(src);
        let (completed, buffer) = is_completed(buffer);
        let (priority, buffer) = has_priority(buffer);

        TodoItem {
            completed,
            priority,
            description: buffer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_very_basic_todo_description() {
        const SAMPLE: &str = "water cat";
        const EXPECTED_DESCRIPTION: &str = SAMPLE;

        assert_eq!(EXPECTED_DESCRIPTION, TodoItem::parse(SAMPLE).description);
    }

    #[test]
    fn parses_basic_todo_as_unfinished() {
        // https://www.youtube.com/watch?v=XqzdHTpKJb
        const SAMPLE: &str = "hang sloths up to dry";

        assert!(!TodoItem::parse(SAMPLE).completed);
    }

    #[test]
    fn parses_basic_todo_as_finished() {
        const SAMPLE: &str = "x start writing txdo";

        assert!(TodoItem::parse(SAMPLE).completed);
    }

    #[test]
    fn parses_finished_todo_description() {
        const SAMPLE: &str = "x start writing txdo";
        const EXPECTED_DESCRIPTION: &str = "start writing txdo";

        assert_eq!(TodoItem::parse(SAMPLE).description, EXPECTED_DESCRIPTION);
    }

    #[test]
    fn ignores_x_with_preceding_space_for_completed() {
        const SAMPLE: &str = " x marks the spot";

        assert!(!TodoItem::parse(SAMPLE).completed);
    }

    #[test]
    fn ignores_x_without_trailing_space_for_finished() {
        const SAMPLE: &str = "xylophone practice";

        assert!(!TodoItem::parse(SAMPLE).completed);
    }

    #[test]
    fn extracts_priority() {
        const SAMPLE: &str = "(A) write test";
        const EXPECTED_PRIORITY: char = 'A';

        assert_eq!(TodoItem::parse(SAMPLE).priority, Some(EXPECTED_PRIORITY));
    }

    #[test]
    fn ignores_priorities_with_preceding_space() {
        const SAMPLE: &str = " (C) bogus copyright text here";

        assert_eq!(TodoItem::parse(SAMPLE).priority, None);
    }

    #[test]
    fn ignores_priorities_without_trailing_space() {
        const SAMPLE: &str = "(O)_(o) <- confirm if best asciimoji";

        assert_eq!(TodoItem::parse(SAMPLE).priority, None);
    }
}
