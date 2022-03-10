// This module shows how to organize a module into multiple folders/files.
mod front_of_house;

// This module shows how to organize a module within a single block.
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_resaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::seat_at_table();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // toast is changable.
    meal.toast = String::from("Wheat");
    println!("toast in breakfast: {}", meal.toast);

    // will panic.
    // meal.seasonal_fruit = String::from("blueberries")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}