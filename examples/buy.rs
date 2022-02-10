use accounting_system::providers::{BelarusMilkFactory, Gasprom, ProductCreator};
use accounting_system::providers::transportation::{BelarusTransportation, DynBelarusTransportation, DynRussianTransportation, RussianTransportation};
use accounting_system::{Unit, Warehouse};

fn main() {
    let mut warehouse1 = Warehouse::default();
    warehouse1.buy_gas(RussianTransportation, Gasprom);
    warehouse1.buy_milk(BelarusTransportation, BelarusMilkFactory);
    let belarus_milk = BelarusMilkFactory.create_product();
    let russian_gas = Gasprom.create_product();
    warehouse1.buy_product_from_stock(Box::new(DynRussianTransportation), Box::new(russian_gas));
    warehouse1.buy_product_from_stock(Box::new(DynBelarusTransportation), Box::new(belarus_milk));
    println!("{}", warehouse1.calculate_price());
}