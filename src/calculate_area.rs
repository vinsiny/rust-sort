pub trait Area {
  fn area(&self) -> f64;
}

pub struct Circle {
  pub radius: f64,
}

impl Area for Circle {
  fn area(&self) -> f64 {
      std::f64::consts::PI * self.radius * self.radius
  }
}

pub struct Triangle {
  pub base: f64,
  pub height: f64,
}

impl Area for Triangle {
  fn area(&self) -> f64 {
      0.5 * self.base * self.height
  }
}

pub struct Square {
  pub side: f64,
}

impl Area for Square {
  fn area(&self) -> f64 {
      self.side * self.side
  }
}

pub fn print_area<T: Area>(shape: T) {
  println!("The area is: {}", shape.area());
}
