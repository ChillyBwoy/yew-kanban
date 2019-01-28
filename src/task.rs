use crate::status::Status;

pub struct Task {
  pub name: String,
  pub assignee: String,
  pub estimate: u32,
  pub status: Status,
}

impl Task {
  pub fn create_empty() -> Self {
    Task {
      name: "".to_string(),
      assignee: "".to_string(),
      estimate: 0,
      status: Status::ToDo,
    }
  }

  pub fn can_left(&self) -> bool {
    self.status != Status::ToDo
  }

  pub fn can_right(&self) -> bool {
    self.status != Status::Done
  }
}
