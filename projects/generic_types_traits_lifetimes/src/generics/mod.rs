pub trait Summary {
  fn summarize(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

pub struct Point<T, U> {
  pub x: T,
  pub y: U,
}

impl<T, U> Point<T, U> {
  pub fn x(&self) -> &T {
    &self.x
  }
  pub fn y(&self) -> &U {
    &self.y
  }
}

impl Point<f32, f32> {
  pub fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

impl<T, U> Point<T, U> {
  pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

impl std::fmt::Display for Point<f32, f32> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "x: {}, y: {}", self.x(), self.y())
  }
}

impl std::fmt::Display for Tweet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.summarize())
  }
}
