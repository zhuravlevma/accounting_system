use accounting_system::providers::belarus::{FileCriminalCase, RaiseEnterprise, KGB};
use accounting_system::providers::russia::{Miller, StealALotOfMoney, StealSomeMoney};
use accounting_system::providers::{
    belarus_factory, russia_factory, Product, ProductCreator, Responsible,
};

fn main() {
    let gasprom = russia_factory();
    let product = gasprom.create_product();
    let price = product.get_price();
    let responsible = gasprom.create_responsible("Ivan");
    let info = responsible.get_info();
    println!("price {}", price);
    println!("info from responsible: {}", info);
    let mut miller = Miller::default();
    miller.add_command(Box::new(StealSomeMoney));
    miller.add_command(Box::new(StealALotOfMoney));
    miller.execute_all();

    let belarus_factory = belarus_factory();
    let product = belarus_factory.create_product();
    let responsible = belarus_factory.create_responsible("Boris");
    let info = responsible.get_info();
    let price = product.get_price();
    println!("price {}", price);
    println!("info from responsible: {}", info);
    let mut kgb = KGB::default();
    kgb.add_command(Box::new(FileCriminalCase));
    kgb.add_command(Box::new(RaiseEnterprise));
    kgb.execute_all();
}
