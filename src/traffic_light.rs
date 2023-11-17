trait TrafficLight {
  fn duration(&self) -> u32;
}

enum Signal {
  Red,
  Yellow,
  Green,
}

impl TrafficLight for Signal {
  fn duration(&self) -> u32 {
      match self {
          Signal::Red => 20,
          Signal::Yellow => 10,
          Signal::Green => 60,
      }
  }
}

pub fn light_duration() {
  let red_light = Signal::Red;
  let yellow_light = Signal::Yellow;
  let green_light = Signal::Green;

  println!("Red light duration: {} seconds", red_light.duration());
  println!("Yellow light duration: {} seconds", yellow_light.duration());
  println!("Green light duration: {} seconds", green_light.duration());
}
