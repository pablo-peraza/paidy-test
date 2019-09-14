mod item;
mod database;

fn main() {
    let item = item::Item::new("Test item");

    println!("Item: {}", item.name);

    let restaurant = database::Restaurant::new();
}

