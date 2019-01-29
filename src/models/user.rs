use std::fmt;

pub enum Avatar {
  Empty,
  Emoji(String),
}

impl fmt::Display for Avatar {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Avatar::Empty => write!(f, ""),
      Avatar::Emoji(emj) => write!(f, "{}", emj),
    }
  }
}

pub struct User {
  pub first_name: String,
  pub last_name: String,
  pub avatar: Avatar,
}

impl User {
  pub fn new(fname: String, lname: String, avatar: Avatar) -> Self {
    User {
      first_name: fname,
      last_name: lname,
      avatar: avatar,
    }
  }

  pub fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }
}
