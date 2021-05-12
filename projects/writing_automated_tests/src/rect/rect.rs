
#[derive(Debug)]
pub struct Rectangle {
  pub width: u32,
  pub height: u32,
}

impl Rectangle {
  pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
  pub fn get_height(&self) -> u32 {
    return self.height;
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn larger_can_hold_smaller() {
    let larger = super::Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = super::Rectangle {
      width: 5,
      height: 1,
    };

    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn smaller_cannot_hold_larger() {
    let larger = super::Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = super::Rectangle {
      width: 5,
      height: 1,
    };

    assert!(!smaller.can_hold(&larger));
  }
}
