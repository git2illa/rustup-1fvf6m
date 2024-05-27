
mod front_of_house;
pub use crate::front_of_house::hosting;

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
pub mod hosting;



