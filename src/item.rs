use rand::Rng;
use std::cmp::PartialEq;
use uuid::Uuid;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub cook_time: u32,
    pub _id: Uuid
}

impl Item {
    pub fn new(name: &str) -> Item {
        Item {
            name: name.to_owned(),
            cook_time: rand::thread_rng().gen_range(5, 16),
            _id: Uuid::new_v4()
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self._id == other._id
    }
}

impl Clone for Item {
    fn clone(&self) -> Self {
        Item {
            name: self.name.to_string(),
            cook_time: self.cook_time,
            _id: self._id
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_creation() {
        let result = Item::new("Test");
        let expected = Item {
            name: "Test".to_owned(),
            cook_time: result.cook_time,
            _id: result._id
        };
        assert_eq!(result, expected);
    }
}
