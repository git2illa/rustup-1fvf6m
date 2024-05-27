pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 pub mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitinglist(){}
        pub fn seat_at_table(){}
     }

     pub mod serving{
         fn take_order(){}
         fn serve_order(){}
         fn take_payment(){}
     }
 }

use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
    // use 
    hosting::add_to_waitinglist();
    hosting::add_to_waitinglist();
    hosting::add_to_waitinglist();



    // absoulute path
    crate::front_of_house::hosting::add_to_waitinglist();

    // relative path
    front_of_house::hosting::add_to_waitinglist();
}

fn serve_order(){}

mod back_of_house{
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }

    fn cook_order(){}
}
