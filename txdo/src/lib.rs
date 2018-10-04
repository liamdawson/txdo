
/*
psuedo format rules:

'x '                          - completed
'([A-Z]) '                    - priority
'[0-9]{4}-[0-9]{2}-[0-9]{2} ' - completion date
'[0-9]{4}-[0-9]{2}-[0-9]{2} ' - creation date (mandatory if completion date set)
'[^\n]+'                      - description
*/

pub struct TodoItem {
    pub completed: bool,
    pub priority: Option<char>,
    pub completed_at: Option<String>,
    pub created_at: Option<String>,
    pub description: String,
}

impl TodoItem {
    pub fn parse(src: &str) -> Self {
        TodoItem {
            completed: false,
            priority: None,
            completed_at: None,
            created_at: None,
            description: src.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_description_from_basic_item() {
        const SAMPLE: &str = "water cat";

        assert_eq!(TodoItem::parse(SAMPLE).description, SAMPLE);
    }
}
