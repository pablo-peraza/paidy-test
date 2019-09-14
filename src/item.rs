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
            cook_time: rand::thread_rng().gen_range(5, 15)
        }
    }

    pub fn cook_time_seconds(&self) -> u32 {
            min_to_sec(self.cook_time)
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

fn min_to_sec(min: u32) -> u32 {
    min * 60
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_to_sec_0_min() {
        assert_eq!(min_to_sec(0), 0);
    }

    #[test]
    fn min_to_sec_1_min() {
        assert_eq!(min_to_sec(1), 60);
    }

    #[test]
    fn min_to_sec_5_min() {
        assert_eq!(min_to_sec(5), 300);
    }

    #[test]
    fn item_creation() {
        let result = Item::new("Test");
        let expected = Item{name: "Test".to_owned(), cook_time: result.cook_time};
        assert_eq!(result, expected);
    }
}
