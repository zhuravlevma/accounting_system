use accounting_system::providers::{russia_factory, Product, ProductCreator, Responsible};
fn main() {
    let gasprom = russia_factory();
    let product = gasprom.create_product();
    let price = product.get_price();
    let responsible = gasprom.create_responsible();
    let info = responsible.get_info();
    println!("price {}", price);
    println!("info from responsible: {}", info);
}
