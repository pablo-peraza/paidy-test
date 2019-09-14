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
    restaurant.add_items(3, vec![item::Item::new("Rice"), item::Item::new("Chicken"), typo]);
    println!("Table 3: {:?}", restaurant.items_from_table(3));
    restaurant.remove_item(3, typo2);
    println!("Table 3: {:?}", restaurant.items_from_table(3));
    restaurant.add_items(3, vec![item::Item::new("Tempura"), item::Item::new("Mochi")]);
    println!("Table 3: {:?}", restaurant.items_from_table(3));
}

