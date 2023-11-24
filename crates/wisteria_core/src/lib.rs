pub mod prelude {
    pub use crate::{TestStruct, TestTrait};
}
pub fn hello_core() {
    println!("Hello From Wisteria Core Crate");
}

pub struct TestStruct;

pub trait TestTrait {
    fn hello_test(&self);
}

impl TestTrait for TestStruct {
    fn hello_test(&self) {
        println!("TestStruct");
    }
}
