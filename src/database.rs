use std::collections::HashMap;
use super::item;

pub struct Restaurant{
    empty: Vec<item::Item>,
    tables: HashMap<u8, Vec<item::Item>>
}

impl Restaurant{
    pub fn new () -> Restaurant {
        Restaurant {
            tables: HashMap::new(),
            empty: Vec::new()
        }
    }

    pub fn items_from_table(&self, table: u8) -> &Vec<item::Item> {
        match self.tables.get(&table) {
            Some(items) => items,
            None => &self.empty
        }
    }

    pub fn add_items(&mut self, table: u8, items: Vec<item::Item>) -> &Vec<item::Item> {
        if self.tables.contains_key(&table) {
            let concatenated = [&self.items_from_table(table)[..], &items[..]].concat();
            self.tables.insert(table, concatenated);
            self.items_from_table(table)
        }
        else {
            self.tables.insert(table, items);
            self.items_from_table(table)
        }
    }

    pub fn remove_item(&mut self, table: u8, item: item::Item) -> &Vec<item::Item> {
        if let Some(items) = self.tables.get_mut(&table) {
            items.retain(|it| *it != item);
        }
        self.items_from_table(table)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::item::Item;

    #[test]
    fn items_from_table_empty() {
        let res = Restaurant::new();
        let expected: Vec<item::Item> = Vec::new();
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn items_from_table_non_empty() {
        let mut res = Restaurant::new();
        res.tables.insert(1, vec![Item{name: "Test 1".to_string(), cook_time: 15}]);
        let expected = vec![Item{name: "Test 1".to_string(), cook_time: 15}];
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn add_items_to_new_table(){
        let mut res = Restaurant::new();
        res.add_items(1, vec![Item{name: "Test 1".to_string(), cook_time: 15}]);
        let expected = vec![Item{name: "Test 1".to_string(), cook_time: 15}];
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn add_items_to_empty_table(){
        let mut res = Restaurant::new();
        res.tables.insert(1, Vec::new());
        res.add_items(1, vec![Item{name: "Test 1".to_string(), cook_time: 15}]);
        let expected = vec![Item{name: "Test 1".to_string(), cook_time: 15}];
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn add_items_to_non_empty_table(){
        let mut res = Restaurant::new();
        res.tables.insert(1, vec![Item{name: "Test 1".to_string(), cook_time: 10}]);
        res.add_items(1, vec![Item{name: "Test 2".to_string(), cook_time: 15}]);
        let expected = vec![Item{name: "Test 1".to_string(), cook_time: 10}, Item{name: "Test 2".to_string(), cook_time: 15}];
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn remove_items_from_new_table(){
        let mut res = Restaurant::new();
        res.remove_item(1, Item{name: "Test 1".to_string(), cook_time: 15});
        let expected: Vec<Item> = Vec::new();
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn remove_items_from_empty_table(){
        let mut res = Restaurant::new();
        res.tables.insert(1, Vec::new());
        res.remove_item(1, Item{name: "Test 1".to_string(), cook_time: 15});
        let expected: Vec<Item> = Vec::new();
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn remove_items_from_non_empty_table_jto_empty(){
        let mut res = Restaurant::new();
        res.tables.insert(1, vec![
            Item{name: "Test 1".to_string(), cook_time: 10}
        ]);
        res.remove_item(1, Item{name: "Test 1".to_string(), cook_time: 10});
        let expected: Vec<Item> = Vec::new();
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn remove_items_from_non_empty_table_begining(){
        let mut res = Restaurant::new();
        res.tables.insert(1, vec![
            Item{name: "Test 1".to_string(), cook_time: 10},
            Item{name: "Test 2".to_string(), cook_time: 15},
            Item{name: "Test 3".to_string(), cook_time: 5},
        ]);
        res.remove_item(1, Item{name: "Test 1".to_string(), cook_time: 10});
        let expected = vec![
            Item{name: "Test 2".to_string(), cook_time: 15},
            Item{name: "Test 3".to_string(), cook_time: 5}
        ];
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn remove_items_from_non_empty_table_middle(){
        let mut res = Restaurant::new();
        res.tables.insert(1, vec![
            Item{name: "Test 1".to_string(), cook_time: 10},
            Item{name: "Test 2".to_string(), cook_time: 15},
            Item{name: "Test 3".to_string(), cook_time: 5},
        ]);
        res.remove_item(1, Item{name: "Test 2".to_string(), cook_time: 15});
        let expected = vec![
            Item{name: "Test 1".to_string(), cook_time: 10},
            Item{name: "Test 3".to_string(), cook_time: 5}
        ];
        assert_eq!(res.items_from_table(1), &expected);
    }

    #[test]
    fn remove_items_from_non_empty_table_ending(){
        let mut res = Restaurant::new();
        res.tables.insert(1, vec![
            Item{name: "Test 1".to_string(), cook_time: 10},
            Item{name: "Test 2".to_string(), cook_time: 15},
            Item{name: "Test 3".to_string(), cook_time: 5},
        ]);
        res.remove_item(1, Item{name: "Test 3".to_string(), cook_time: 5});
        let expected = vec![
            Item{name: "Test 1".to_string(), cook_time: 10},
            Item{name: "Test 2".to_string(), cook_time: 15}
        ];
        assert_eq!(res.items_from_table(1), &expected);
    }
}
