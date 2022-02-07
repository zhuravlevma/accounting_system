use accounting_system::providers::{russia_factory, Product, ProductCreator, Provider};
fn main() {
    let gasprom = russia_factory();
    let product = gasprom.create_product();
    let price = product.get_price();
    let provider = gasprom.create_provider();
    let info = provider.get_info();
    println!("price {}", price);
    println!("info provider: {}", info);
}
