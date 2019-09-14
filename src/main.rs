mod item;
mod database;

fn main() {
    let mut restaurant = database::Restaurant::new();
    let item = item::Item::new("Test item");

    println!("Table 1: {:?}", restaurant.items_from_table(1));
    restaurant.add_items(1, vec![item]);
    println!("Table 1: {:?}", restaurant.items_from_table(1));
    restaurant.add_items(2, vec![item::Item::new("Rice"), item::Item::new("Chicken")]);
    println!("Table 2: {:?}", restaurant.items_from_table(2));

    let typo = item::Item::new("Brocolli");
    let typo2 = typo.clone();
    let table = 3;

    restaurant.add_items(table, vec![item::Item::new("Rice"), item::Item::new("Chicken"), typo]);
    println!("Table {}: {:?}", table, restaurant.items_from_table(table));
    restaurant.remove_item(table, typo2);
    println!("Table {}: {:?}", table, restaurant.items_from_table(table));
    restaurant.add_items(table, vec![item::Item::new("Tempura"), item::Item::new("Mochi")]);
    println!("Table {}: {:?}", table, restaurant.items_from_table(table));
}

