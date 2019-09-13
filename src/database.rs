use std::collections::HashMap;
use super::item;


pub struct Registry(u8, Vec<item::Item>);

pub struct Restaurant{
    pub tables: HashMap<u8, Vec<item::Item>>
}

impl Restaurant{
    pub fn new () -> Restaurant {
        Restaurant {
            tables: HashMap::new()
        }
    }

    /* fn items_from_table(&self, table: u8) -> Vec<item::Item> {
        match self.tables.get(&table) {
            Some(items) => items,
            None => Vec::default()
        }
    } */

    pub fn add(&mut self, registry: Registry) {
        self.tables.insert(registry.0, registry.1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_add_1() {
        let mut res = Restaurant::new();
        res.add(Registry(1, Vec::new()));
        let expected: Vec<item::Item> = Vec::new();
        assert_eq!(res.tables.get(&1).unwrap(), &expected);
    }

    /* #[test]
    fn tables_query_empty() {
        let res = Restaurant::new();
        assert_eq!(res.items_from_table(0).len(), 0);
    } */
}
