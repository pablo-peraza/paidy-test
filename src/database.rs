use super::item;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Restaurant {
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
            tables: HashMap::new()
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
        }).and_then(|items| self.replace_whole(table, items, Action::Updated) )
    }

    pub fn add_items(&mut self, table: u8, items: Vec<item::Item>) -> Result<Action, String> {
        let result = self.map(table, |old_items| [old_items, &items[..]].concat());
        match result {
            Err(_) => self.replace_whole(table, items, Action::Inserted),
            _ => Ok(Action::Inserted)
        }
    }

    pub fn remove_item(&mut self, table: u8, item_id: Uuid) -> Result<Action, String> {
        self.map(table, |items| -> Vec<item::Item> {
            items.iter()
                .filter(|x| x._id == item_id)
                .map(|x| x.clone())
                .collect()
        }).and_then(|items| self.replace_whole(table, items, Action::Deleted))
    }

    fn map<T>(&self, table: u8, mut op: impl FnMut(&Vec<item::Item>) -> T) -> Result<T, String> {
        match self.tables.get(&table) {
            Some(items) => Ok(op(items)),
            None => Err("Table is empty".to_string()),
        }
    }

    fn replace_whole(&mut self, table: u8, items: Vec<item::Item>, action: Action) -> Result<Action, String>{
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
        let expected = Some("Table is empty".to_string());
        assert_eq!(expected, res.items_from_table(1).err());
    }

    #[test]
    fn items_from_table_non_empty() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        res.tables.insert(1, vec![item.clone()]);
        let expected = vec![item];
        match res.items_from_table(1) {
            Ok(Action::Data(items)) => assert_eq!(items, expected),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn add_items_to_new_table() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        match res.add_items(1, vec![item.clone()]) {
            Ok(Action::Inserted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        let expected = vec![item];
        match res.items_from_table(1) {
            Ok(Action::Data(items)) => assert_eq!(items, expected),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn add_items_to_empty_table() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        res.tables.insert(1, Vec::new());
        match res.add_items(1, vec![item.clone()]) {
            Ok(Action::Inserted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        let expected = vec![item];
        match res.items_from_table(1) {
            Ok(Action::Data(items)) => assert_eq!(items, expected),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn add_items_to_non_empty_table() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        let item2 = Item::new("Test 2");
        res.tables.insert(1,vec![item.clone()]);
        match res.add_items(1, vec![item.clone()]) {
            Ok(Action::Inserted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        let expected = vec![item, item2];
        match res.items_from_table(1) {
            Ok(Action::Data(items)) => assert_eq!(items, expected),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn remove_items_from_new_table() {
        let mut res = Restaurant::new();
        res.remove_item(1, Item::new("Test 1")._id);
        match res.items_from_table(1) {
            Err(s) => assert!(true),
            Ok(_) => assert!(false)
        }

    }

    #[test]
    fn remove_items_from_empty_table() {
        let mut res = Restaurant::new();
        res.tables.insert(1, Vec::new());
        res.remove_item(1, Item::new("Test 1")._id);
        match res.items_from_table(1) {
            Err(s) => assert!(true),
            Ok(_) => assert!(false)
        }
    }

    #[test]
    fn remove_items_from_non_empty_table_jto_empty() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        res.tables.insert(1,vec![item.clone()]);
        match res.remove_item(1, item._id) {
            Ok(Action::Deleted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(s) => assert!(true),
            Ok(_) => assert!(false)
        }
    }

    #[test]
    fn remove_items_from_non_empty_table_begining() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        let item2 = Item::new("Test 2");
        let item3 = Item::new("Test 3");
        res.tables.insert(1,vec![item.clone(), item2.clone(), item3.clone()]);
        let expected = vec![item2, item3];
        match res.remove_item(1, item._id) {
            Ok(Action::Deleted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(s) => assert!(false),
            Ok(Action::Data(items)) => assert_eq!(expected, items),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn remove_items_from_non_empty_table_middle() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        let item2 = Item::new("Test 2");
        let item3 = Item::new("Test 3");
        res.tables.insert(1,vec![item.clone(), item2.clone(), item3.clone()]);
        let expected = vec![item, item3];
        match res.remove_item(1, item2._id) {
            Ok(Action::Deleted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(s) => assert!(false),
            Ok(Action::Data(items)) => assert_eq!(expected, items),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn remove_items_from_non_empty_table_ending() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        let item2 = Item::new("Test 2");
        let item3 = Item::new("Test 3");
        res.tables.insert(1,vec![item.clone(), item2.clone(), item3.clone()]);
        let expected = vec![item, item2];
        match res.remove_item(1, item3._id) {
            Ok(Action::Deleted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(s) => assert!(false),
            Ok(Action::Data(items)) => assert_eq!(expected, items),
            _a => panic!("This isn't right")
        }
    }
}
