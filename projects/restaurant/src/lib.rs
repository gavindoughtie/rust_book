#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house;

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
        pub appetizer: Appetizer,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
                appetizer: Appetizer::Salad
            }
        }
    }

    impl std::fmt::Display for Breakfast {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}, {}", self.toast, self.seasonal_fruit)           
        }
    }
}

pub use crate::front_of_house::{hosting, serving};

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let app1 = back_of_house::Appetizer::Soup;
    let app2 = back_of_house::Appetizer::Salad;

    meal.appetizer = app1;
    meal.appetizer = app2; // Replaces app1

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    self::front_of_house::hosting::add_to_waitlist();

    // Imported path
    hosting::add_to_waitlist();

    let food = serving::take_order("rye");
    serving::serve_order(meal);
    serving::serve_order(food);
    serving::take_payment();
}
