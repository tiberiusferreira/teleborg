extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate log;

pub use self::bot::bot::Bot;

mod value_extension;
pub mod bot;
pub mod objects;
pub mod error;






