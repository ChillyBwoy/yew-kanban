use yew::prelude::{Component, ComponentLink, Html, Renderable, ShouldRender};

use crate::models::user::{Avatar, User};

pub enum Msg {}

pub struct Model {
  users: Vec<User>,
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model {
      users: vec![
        User::new("John".to_string(), "Doe".to_string(), Avatar::Empty),
        User::new(
          "Jane".to_string(),
          "Smith".to_string(),
          Avatar::Emoji("🐶".to_string()),
        ),
      ],
    }
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    true
  }
}

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    html! {
      <div class="container",>
        <h2>{ "Users" }</h2>
        { view_list(&self.users) }
      </div>
    }
  }
}

fn view_item((idx, user): (usize, &User)) -> Html<Model> {
  html! {
    <li>
      <span>{ idx + 1 }</span>
      <span>{ &user.full_name() }</span>
      <span>{ &user.avatar }</span>
    </li>
  }
}

fn view_list(users: &Vec<User>) -> Html<Model> {
  html! {
    <ul>
      { for users.iter().enumerate().map(view_item) }
    </ul>
  }
}
