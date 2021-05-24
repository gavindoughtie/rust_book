pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  // if the type were, say, Vec<Box<dyn Clone>>,
  // then this would not be object safe, since
  // dyn (Trait) effectively erases the type of the
  // object and Clone uses Self, which has to know
  // the type at compile time. Maybe.
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
      for component in self.components.iter() {
          component.draw();
      }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
      // code to actually draw a button
      println!("Button: {}", self.label);
  }
}

pub struct List {
  pub width: u32,
  pub height: u32,
  pub items: Vec<String>,
}

impl Draw for List {
  fn draw(&self) {
      // code to actually draw a button
      println!("List: {:?}", self.items);
  }
}

pub struct AveragedCollection {
  list: Vec<i32>,
  average: f64,
}

impl AveragedCollection {
  pub fn new() -> AveragedCollection {
    AveragedCollection{
      list: vec![],
      average: 0.0
    }
  }
  pub fn add(&mut self, value: i32) {
      self.list.push(value);
      self.update_average();
  }

  pub fn remove(&mut self) -> Option<i32> {
      let result = self.list.pop();
      match result {
          Some(value) => {
              self.update_average();
              Some(value)
          }
          None => None,
      }
  }

  pub fn average(&self) -> f64 {
      self.average
  }

  fn update_average(&mut self) {
      let total: i32 = self.list.iter().sum();
      self.average = total as f64 / self.list.len() as f64;
  }
}
