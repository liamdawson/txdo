
/*
psuedo format rules:

'x '                          - completed
'([A-Z]) '                    - priority
'[0-9]{4}-[0-9]{2}-[0-9]{2} ' - completion date
'[0-9]{4}-[0-9]{2}-[0-9]{2} ' - creation date (mandatory if completion date set)
'[^\n]+'                      - description
*/

pub struct TodoItem<'a> {
    raw: &'a str,
    pub completed: bool,
    pub description: &'a str,
}

impl<'a> TodoItem<'a> {
    pub fn parse(src: &'a str) -> Self {
        TodoItem {
            raw: src,
            completed: false,
            description: src,
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
}
