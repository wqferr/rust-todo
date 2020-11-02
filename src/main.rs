mod item;

use item::Item;

fn main() {
    let mut test_item = Item::new("title", Some("description"));
    println!("{}", test_item.to_string());

    test_item.set_done(true);
    println!("{}", test_item.to_string());
}
