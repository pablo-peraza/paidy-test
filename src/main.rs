mod item;
mod database;

fn main() {
    let item = item::Item::new("Test item");

    println!("Item: {}", item.name);
    println!("Item's cook time: {} minutes | {} seconds", item.cook_time, item.cook_time_seconds());

    let restaurant = database::Restaurant::new();
}

