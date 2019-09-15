use rand::Rng;
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub cook_time: u32
}

impl Item{

    pub fn new(name: &str) -> Item {
        Item {
            name: name.to_owned(),
            cook_time: rand::thread_rng().gen_range(5, 16)
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.cook_time == other.cook_time
    }
}

impl Clone for Item {
    fn clone(&self) -> Self {
        Item {
            name: self.name.to_string(),
            cook_time: self.cook_time
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_creation() {
        let result = Item::new("Test");
        let expected = Item{name: "Test".to_owned(), cook_time: result.cook_time};
        assert_eq!(result, expected);
    }
}
