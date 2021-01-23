// Creating Custom Types for Validation
#[derive(Debug)]
pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
      if value < 1 || value > 100 {
          panic!("Guess value must be between 1 and 100, got {}.", value);
      }

      Guess { value }
  }

  pub fn value(&self) -> i32 {
      self.value
  }
}

#[derive(Debug)]
pub struct Guess2 {
  value: Option<i32>,
}

impl Guess2 {
  pub fn new(value: i32) -> Guess2 {
      if value < 1 || value > 100 {
        return Guess2 { 
          value: None
        }
      }

      Guess2 { 
        value: Some(value)
      }
  }

  pub fn value(&self) -> Option<i32> {
      self.value
  }
}

#[derive(Debug)]
pub struct Guess3 {
  value: i32,
}

impl Guess3 {
  pub fn new(value: i32) -> Result<Guess3, String> {
      if value < 1 || value > 100 {
          let error_message = format!("Guess value must be between 1 and 100, got {}.", value);
          return Err(error_message);
      }

      Ok(Guess3 { value })
  }

  pub fn value(&self) -> i32 {
      self.value
  }
}