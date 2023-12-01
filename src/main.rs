///associate greetings module with this crate
mod greetings;
mod how_you_hold_data_for_operations;
///Optionally load each member of greetings
/*use greetings::default_greeting;
use greetings::spanish;
use greetings::french;*/
///Alternatively, use * to load them all
//use greetings::*;
///Alternatively, load them in one line

extern crate hello_world2_lib;

use greetings::{english, spanish, french};
fn main() {
// let mut greeting: &str = "Hello there";
// greeting = "Hello there again";
// println!("Hello, world!");
// println!("{}", english::default_greeting());
// println!("{}", spanish::default_greeting());
// println!("{}", french::default_greeting());
how_you_hold_data_for_operations::derived::user_defined::run2();
}