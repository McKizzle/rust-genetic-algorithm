pub use self::item::Item;

mod item {
    use std::fmt;

    pub struct Item {
        pub weight: i32,
        pub value: i32,
    }
    
    impl fmt::Display for Item {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Item [{}w, {}v]", &self.weight, &self.value)
        }
    }
}

