use super::item;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Restaurant {
    empty: Vec<item::Item>,
    tables: HashMap<u8, Vec<item::Item>>,
}

#[derive(Debug)]
pub enum Action{
    Inserted,
    Deleted,
    Updated,
    Data(Vec<item::Item>)
}

impl Restaurant {

    pub fn new() -> Restaurant {
        Restaurant {
            tables: HashMap::new(),
            empty: Vec::new(),
        }
    }

    pub fn items_from_table(&self, table: u8) -> Result<Action, String> {
        self.map(table, |items| Action::Data(items.to_vec()))
    }

    pub fn update_item(&mut self, table: u8, item_id: Uuid, item: item::Item ) -> Result<Action, String> {
        self.map(table, |old_items| -> Vec<item::Item> {
            old_items
                .iter()
                .map(|x| (if x._id == item_id { &item } else { x }).clone() )
                .collect()
        }).and_then(|items| self.replaceWhole(table, items, Action::Updated) )
    }

    pub fn add_items(&mut self, table: u8, items: Vec<item::Item>) -> Result<Action, String> {
        let result = self.map(table, |old_items| [old_items, &items[..]].concat());
        match result {
            Err(_) => self.replaceWhole(table, items, Action::Inserted),
            _ => Ok(Action::Inserted)
        }
    }

    pub fn remove_item(&mut self, table: u8, item_id: Uuid) -> Result<Action, String> {
        self.map(table, |items| -> Vec<item::Item> {
            items.iter()
                .filter(|x| x._id == item_id)
                .map(|x| x.clone())
                .collect()
        }).and_then(|items| self.replaceWhole(table, items, Action::Deleted))
    }

    fn map<T>(&self, table: u8, mut op: impl FnMut(&Vec<item::Item>) -> T) -> Result<T, String> {
        match self.tables.get(&table) {
            Some(items) => Ok(op(items)),
            None => Err("Table is empty".to_string()),
        }
    }

    fn replaceWhole(&self, table: u8, items: Vec<item::Item>, action: Action) -> Result<Action, String>{
        self.tables.insert(table, items);
        Ok(action)
    }
}

#[cfg(test)]
mod tests {
    use super::item::Item;
    use super::*;

    #[test]
    fn items_from_table_empty() {
        let res = Restaurant::new();
        let expected: Vec<item::Item> = Vec::new();
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn items_from_table_non_empty() {
        let mut res = Restaurant::new();
        res.tables.insert(
            1,
            vec![Item {
                name: "Test 1".to_string(),
                cook_time: 15,
            }],
        );
        let expected = vec![Item {
            name: "Test 1".to_string(),
            cook_time: 15,
        }];
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn add_items_to_new_table() {
        let mut res = Restaurant::new();
        res.add_items(
            1,
            vec![Item {
                name: "Test 1".to_string(),
                cook_time: 15,
            }],
        );
        let expected = vec![Item {
            name: "Test 1".to_string(),
            cook_time: 15,
        }];
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn add_items_to_empty_table() {
        let mut res = Restaurant::new();
        res.tables.insert(1, Vec::new());
        res.add_items(
            1,
            vec![Item {
                name: "Test 1".to_string(),
                cook_time: 15,
            }],
        );
        let expected = vec![Item {
            name: "Test 1".to_string(),
            cook_time: 15,
        }];
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn add_items_to_non_empty_table() {
        let mut res = Restaurant::new();
        res.tables.insert(
            1,
            vec![Item {
                name: "Test 1".to_string(),
                cook_time: 10,
            }],
        );
        res.add_items(
            1,
            vec![Item {
                name: "Test 2".to_string(),
                cook_time: 15,
            }],
        );
        let expected = vec![
            Item {
                name: "Test 1".to_string(),
                cook_time: 10,
            },
            Item {
                name: "Test 2".to_string(),
                cook_time: 15,
            },
        ];
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn remove_items_from_new_table() {
        let mut res = Restaurant::new();
        res.remove_item(
            1,
            Item {
                name: "Test 1".to_string(),
                cook_time: 15,
            },
        );
        let expected: Vec<Item> = Vec::new();
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn remove_items_from_empty_table() {
        let mut res = Restaurant::new();
        res.tables.insert(1, Vec::new());
        res.remove_item(
            1,
            Item {
                name: "Test 1".to_string(),
                cook_time: 15,
            },
        );
        let expected: Vec<Item> = Vec::new();
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn remove_items_from_non_empty_table_jto_empty() {
        let mut res = Restaurant::new();
        res.tables.insert(
            1,
            vec![Item {
                name: "Test 1".to_string(),
                cook_time: 10,
            }],
        );
        res.remove_item(
            1,
            Item {
                name: "Test 1".to_string(),
                cook_time: 10,
            },
        );
        let expected: Vec<Item> = Vec::new();
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn remove_items_from_non_empty_table_begining() {
        let mut res = Restaurant::new();
        res.tables.insert(
            1,
            vec![
                Item {
                    name: "Test 1".to_string(),
                    cook_time: 10,
                },
                Item {
                    name: "Test 2".to_string(),
                    cook_time: 15,
                },
                Item {
                    name: "Test 3".to_string(),
                    cook_time: 5,
                },
            ],
        );
        res.remove_item(
            1,
            Item {
                name: "Test 1".to_string(),
                cook_time: 10,
            },
        );
        let expected = vec![
            Item {
                name: "Test 2".to_string(),
                cook_time: 15,
            },
            Item {
                name: "Test 3".to_string(),
                cook_time: 5,
            },
        ];
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn remove_items_from_non_empty_table_middle() {
        let mut res = Restaurant::new();
        res.tables.insert(
            1,
            vec![
                Item {
                    name: "Test 1".to_string(),
                    cook_time: 10,
                },
                Item {
                    name: "Test 2".to_string(),
                    cook_time: 15,
                },
                Item {
                    name: "Test 3".to_string(),
                    cook_time: 5,
                },
            ],
        );
        res.remove_item(
            1,
            Item {
                name: "Test 2".to_string(),
                cook_time: 15,
            },
        );
        let expected = vec![
            Item {
                name: "Test 1".to_string(),
                cook_time: 10,
            },
            Item {
                name: "Test 3".to_string(),
                cook_time: 5,
            },
        ];
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn remove_items_from_non_empty_table_ending() {
        let mut res = Restaurant::new();
        res.tables.insert(
            1,
            vec![
                Item {
                    name: "Test 1".to_string(),
                    cook_time: 10,
                },
                Item {
                    name: "Test 2".to_string(),
                    cook_time: 15,
                },
                Item {
                    name: "Test 3".to_string(),
                    cook_time: 5,
                },
            ],
        );
        res.remove_item(
            1,
            Item {
                name: "Test 3".to_string(),
                cook_time: 5,
            },
        );
        let expected = vec![
            Item {
                name: "Test 1".to_string(),
                cook_time: 10,
            },
            Item {
                name: "Test 2".to_string(),
                cook_time: 15,
            },
        ];
        assert_eq!(res.items_from_table(1), &expected);
    }
}
