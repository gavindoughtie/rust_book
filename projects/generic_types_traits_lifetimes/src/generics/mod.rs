use std::cmp::PartialOrd;
use std::fmt::{Display, Result};

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

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}

impl<T: Display, U: Display> Point<T, U> {
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

// impl Ord for Point<f32, f32> {
//   fn cmp(&self, other: &Point<f32, f32>) -> Ordering {
//     if self.x > other.x {
//       return Ordering::Greater;
//     } else if self.x < other.x {
//       return Ordering::Less;
//     }
//     Ordering::Equal
//   }
// }

impl<T, U> Point<T, U> {
    pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// impl<T :Display + PartialOrd, U :Display + PartialOrd> PartialOrd for Point<T, U> {
//   fn partial_cmp(&self, other: &Point<T, U>) -> Option<Ordering> {
//     self.x.partial_cmp(&other.x)
//   }
// }

// impl<T :Display + Ord, U :Display + Ord> Ord for Point<T, U> {
//   fn cmp(&self, other: &Point<T, U>) {
//     return self.x.cmp(other.x);
//   }
//   // fn le(&self, other: &Point<T, U>) -> bool {
//   //   self.x.le(&other.x)
//   // }
//   // fn gt(&self, other: &Point<T, U>) -> bool {
//   //   self.x.gt(&other.x)
//   // }
// }

// impl<T :Display + PartialEq, U :Display + PartialEq> PartialEq for Point<T, U> {
//   fn eq(&self, other: &Point<T, U>) -> bool {
//     self.x == other.x
//   }
// }

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{}", self.summarize())
    }
}

impl Display for Point<f32, f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "x: {}, y: {}", self.x(), self.y())
    }
}

impl Display for Point<String, String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "x: {}, y: {}", self.x(), self.y())
    }
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
