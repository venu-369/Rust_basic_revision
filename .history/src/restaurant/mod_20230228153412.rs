mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }
    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("wheat"),
                cheese: String::from("mozzarella"),
                topping: String::from(topping),
            }
        }
    }
}
