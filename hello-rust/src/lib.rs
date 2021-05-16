mod front_of_house;
mod pig_latin;
pub mod agenda;

pub use front_of_house::hosting;
pub use pig_latin::convert_words;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    // use crate::front_of_house::hosting::add_to_waitlist;
    // add_to_waitlist();
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn other() {
        let a = 5;
        let b = 4;
        assert!(a < b);
    }
}