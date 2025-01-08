pub mod domain {
    pub trait Identifier: Clone + PartialEq + Send + Sync {}

    pub trait Entity : Eq + PartialEq {
        type Identifier: Identifier;

        fn get_id(&self) -> &Self::Identifier;
    }
}

pub fn ping() {
    println!("Hello from dcli-ddd!");
}
