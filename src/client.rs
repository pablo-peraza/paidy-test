use super::{database::Restaurant, item::Item};
use rand::Rng;
use std::time::Duration;

pub enum ClientActions {
    WriteOrder,
    RemoveItem,
    TableStatus,
}

pub struct Client {
    pub id: u8,
}

impl Client {
    pub fn new(id: u8) -> Self {
        Client { id }
    }

    pub fn wait_some_time(&self) -> Duration {
        let wait_time = rand::thread_rng().gen_range(1000, 5001);
        println!(
            "I am tired!; waiting {} ms before doing something else",
            wait_time
        );
        Duration::from_millis(wait_time)
    }

    pub fn do_job(&self, mut restaurant: std::sync::MutexGuard<Restaurant>) {
        let table = rand::thread_rng().gen_range(1, 16);
        println!(
            "Waitress #{} doing her job! I am serving table #{}",
            self.id, table
        );
        match self.random_action() {
            ClientActions::TableStatus => {
                println!(
                    "Table {} have the following items: {:?}",
                    table,
                    restaurant.items_from_table(table)
                );
            }
            ClientActions::WriteOrder => {
                println!(
                    "Table {} is asking for new items. They currently have: {:?}",
                    table,
                    restaurant.items_from_table(table)
                );
                let items = restaurant.add_items(
                    table,
                    vec![
                        Item::new("Pork Ramen"),
                        Item::new("Chicken Curry"),
                        Item::new("Coffee"),
                    ],
                );
                println!("Adding the following items {:?} to table #{}", items, table);
                println!(
                    "Done serving table #{}. They currently have: {:?}",
                    table,
                    restaurant.items_from_table(table)
                );
            }
            ClientActions::RemoveItem => {
                println!(
                    "Oops! Made a mistake with table #{}. Need to remove a chicken curry from {:?}",
                    table,
                    restaurant.items_from_table(table)
                );
                let it = Item::new("Chicken Curry");
                let items = restaurant.remove_item(table, it.clone());
                println!("Table #{} items now are {:?}", table, items);
            }
        };
        println!(
            "Waitress #{} done!!!______________________________________________",
            self.id
        );
    }

    fn random_action(&self) -> ClientActions {
        match rand::thread_rng().gen_range(1, 4) {
            1 => ClientActions::WriteOrder,
            2 => ClientActions::RemoveItem,
            _ => ClientActions::TableStatus,
        }
    }
}
