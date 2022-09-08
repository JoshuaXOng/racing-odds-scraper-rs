use crate::pages::page::{AsPage};

pub trait BetfairPage: AsPage {
  fn do_something_bf(&self) {
    println!("Called do_something_bf!");
  }
}