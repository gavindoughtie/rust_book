pub use super::super::back_of_house::{Breakfast};

pub fn take_order(toast: &str) -> Breakfast {
  return Breakfast::summer(toast);
}

pub fn serve_order(_order: Breakfast) {

}

pub fn take_payment() {}
