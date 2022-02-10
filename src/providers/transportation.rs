use crate::providers::{BelarusMilk, BelarusMilkFactory, Gasprom, Product, ProductCreator, Responsible, RussianGas};
use crate::providers::belarus::{FileCriminalCase, KGB};
use crate::providers::russia::{Miller, StealALotOfMoney};

pub trait Transportation {
    type Creator: ProductCreator;
    type Product: Product;
    fn transport(&self, factory: Self::Creator) -> Self::Product;
}

pub struct BelarusTransportation;
impl Transportation for BelarusTransportation {
    type Creator = BelarusMilkFactory;
    type Product = BelarusMilk;

    fn transport(&self, factory: Self::Creator) -> Self::Product {
        let responsible = factory.create_responsible("Ivan");
        let mut kgb = KGB::default();
        kgb.add_command(Box::new(FileCriminalCase));
        println!("{}", responsible.get_info());
        kgb.execute_all();
        factory.create_product()
    }
}

pub struct RussianTransportation;
impl Transportation for RussianTransportation {
    type Creator = Gasprom;
    type Product = RussianGas;

    fn transport(&self, factory: Self::Creator) -> Self::Product {
        let mut miller = Miller::default();
        miller.add_command(Box::new(StealALotOfMoney));
        miller.add_command(Box::new(StealALotOfMoney));
        miller.add_command(Box::new(StealALotOfMoney));
        miller.execute_all();
        factory.create_product()
    }
}

pub trait DynTransportation {
    fn transport(&self, product: Box<dyn Product>) -> Box<dyn Product>;
}

pub struct DynRussianTransportation;
impl DynTransportation for DynRussianTransportation {
    fn transport(&self, product: Box<dyn Product>) -> Box<dyn Product> {
        let mut miller = Miller::default();
        miller.add_command(Box::new(StealALotOfMoney));
        miller.add_command(Box::new(StealALotOfMoney));
        miller.add_command(Box::new(StealALotOfMoney));
        miller.execute_all();
        product
    }
}

pub struct DynBelarusTransportation;
impl DynTransportation for DynBelarusTransportation {
    fn transport(&self, product: Box<dyn Product>) -> Box<dyn Product> {
        let mut kgb = KGB::default();
        kgb.add_command(Box::new(FileCriminalCase));
        kgb.execute_all();
        product
    }
}