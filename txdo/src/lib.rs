
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
    pub description: &'a str,
}

impl<'a> TodoItem<'a> {
    pub fn parse(src: &'a str) -> Self {
        TodoItem {
            raw: src,
            description: src,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_parses_very_basic_todo() {
        const SAMPLE: &'static str = "water cat";
        const EXPECTED_DESCRIPTION: &'static str = SAMPLE;

        assert_eq!(EXPECTED_DESCRIPTION, TodoItem::parse(SAMPLE).description);
    }
}
