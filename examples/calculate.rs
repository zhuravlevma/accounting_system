use accounting_system::writer::{SecureWriter, Terminal, Write};
use accounting_system::Warehouse;
use accounting_system::{Product, Shop, Unit};

fn main() {
    let cheese = Product::new(123.8, 10.0);
    let fish = Product::new(127.3, 40.0);
    let wine = Product::new(567.0, 70.0);

    let mut warehouse1 = Warehouse::default();
    warehouse1.add(Box::new(cheese));
    warehouse1.add(Box::new(fish));
    warehouse1.add(Box::new(wine));

    let potato = Product::new(124.8, 25.0);
    let vegetable = Product::new(200.0, 12.0);

    let mut warehouse2 = Warehouse::default();
    warehouse2.add(Box::new(potato));
    warehouse2.add(Box::new(vegetable));

    let sum_warehouse1 = warehouse1.calculate_price();
    println!("Warehouse1 {}", sum_warehouse1);
    let sum_warehouse2 = warehouse2.calculate_price();
    println!("Warehouse2 {}", sum_warehouse2);

    let mut shop = Shop::default();
    shop.add(Box::new(warehouse1));
    shop.add(Box::new(warehouse2));
    let sum_shop = shop.calculate_price();
    println!("Shop {}", sum_shop);

    let writer = Terminal {};
    writer.write(sum_shop.to_string());
    let secure_wrapper = SecureWriter {
        writer: Box::new(writer),
    };
    secure_wrapper.write(sum_shop.to_string());
}
