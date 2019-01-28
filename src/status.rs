use std::cmp::PartialEq;
use std::fmt;

pub enum Status {
  ToDo,
  InProgress,
  Review,
  Testing,
  Ready,
  Done,
}

impl PartialEq for Status {
  fn eq(&self, other: &Status) -> bool {
    self.to_string() == other.to_string()
  }
}

impl From<&str> for Status {
  fn from(s: &str) -> Self {
    match s {
      "todo" => Status::ToDo,
      "in_progress" => Status::InProgress,
      "review" => Status::Review,
      "testing" => Status::Testing,
      "ready" => Status::Ready,
      "done" => Status::Done,
      _ => Status::ToDo,
    }
  }
}

impl fmt::Display for Status {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Status::ToDo => write!(f, "todo"),
      Status::InProgress => write!(f, "in_progress"),
      Status::Review => write!(f, "review"),
      Status::Testing => write!(f, "testing"),
      Status::Ready => write!(f, "ready"),
      Status::Done => write!(f, "done"),
    }
  }
}

impl Status {
  pub fn left(&self) -> Self {
    match self {
      Status::Done => Status::Ready,
      Status::Ready => Status::Testing,
      Status::Testing => Status::Review,
      Status::Review => Status::InProgress,
      Status::InProgress => Status::ToDo,
      Status::ToDo => Status::ToDo,
    }
  }

  pub fn right(&self) -> Self {
    match self {
      Status::ToDo => Status::InProgress,
      Status::InProgress => Status::Review,
      Status::Review => Status::Testing,
      Status::Testing => Status::Ready,
      Status::Ready => Status::Done,
      Status::Done => Status::Done,
    }
  }

  pub fn name(&self) -> &str {
    match self {
      Status::ToDo => "To be done",
      Status::InProgress => "In progress",
      Status::Review => "Review",
      Status::Testing => "Testing",
      Status::Ready => "Ready",
      Status::Done => "Done",
    }
  }
}
