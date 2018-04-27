extern crate reqwest;

#[macro_use] extern crate serde_derive;
extern crate serde_json;

#[macro_use] extern crate log;
#[macro_use] extern crate failure;
pub use self::bot::bot::{Bot};
pub use self::bot::update_cleaner::*;
pub use self::objects::*;
pub use self::bot::bot::TelegramInterface;

mod value_extension;
pub mod bot;
pub mod objects;
pub mod error;






