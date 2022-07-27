use codegen::persist::Persist;



use serde::{Deserialize, Serialize};

#[derive(Persist, Serialize, Deserialize, Debug)]
pub struct MyPersistStruct {
    counter: u32,
}

impl MyPersistStruct {
    pub fn new() -> Self {
        MyPersistStruct { counter: 0 }
    }
}


fn main() {
    let mut my_struct = MyPersistStruct::new();
    my_struct.counter = 1;

    my_struct.save();
    println!("{:?}", my_struct.load());
}


