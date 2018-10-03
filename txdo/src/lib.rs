
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

pub struct TodoItem {
    pub completed: bool,
    pub description: String,
}

impl TodoItem {
    pub fn parse(src: &str) -> Self {
        let buffer = String::from(src);
        let (completed, buffer) = is_completed(buffer);

        TodoItem {
            completed,
            description: buffer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_parses_very_basic_todo_description() {
        const SAMPLE: &str = "water cat";
        const EXPECTED_DESCRIPTION: &str = SAMPLE;

        assert_eq!(EXPECTED_DESCRIPTION, TodoItem::parse(SAMPLE).description);
    }

    #[test]
    fn correctly_parses_basic_todo_as_unfinished() {
        // https://www.youtube.com/watch?v=XqzdHTpKJb
        const SAMPLE: &str = "hang sloths up to dry";

        assert!(!TodoItem::parse(SAMPLE).completed);
    }

    #[test]
    fn correctly_parses_basic_todo_as_finished() {
        const SAMPLE: &str = "x start writing txdo";

        assert!(TodoItem::parse(SAMPLE).completed);
    }

    #[test]
    fn correctly_parses_finished_todo_description() {
        const SAMPLE: &str = "x start writing txdo";
        const EXPECTED_DESCRIPTION: &str = "start writing txdo";

        assert_eq!(TodoItem::parse(SAMPLE).description, EXPECTED_DESCRIPTION);
    }

    #[test]
    fn correctly_ignores_x_following_space_for_finished() {
        const SAMPLE: &str = " x marks the spot";

        assert!(!TodoItem::parse(SAMPLE).completed);
    }

    #[test]
    fn correctly_ignores_x_without_trailing_space_for_finished() {
        const SAMPLE: &str = "xylophone practice";

        assert!(!TodoItem::parse(SAMPLE).completed);
    }
}
