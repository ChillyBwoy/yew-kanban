use yew::prelude::*;

use crate::router;

use crate::components::board::Model as BoardModel;
use crate::components::user_list::Model as UserListModel;

pub enum Child {
  Board,
  UserList,
  PathNotFound(String),
}

pub struct Model {
  child: Child,
  router: Box<Bridge<router::Router<()>>>,
}

pub enum Msg {
  NavigateTo(Child),
  HandleRoute(router::Route<()>),
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
    let callback = link.send_back(|route: router::Route<()>| Msg::HandleRoute(route));
    let mut router = router::Router::bridge(callback);

    // TODO Not sure if this is technically correct. This should be sent _after_ the component has been created.
    // I think the `Component` trait should have a hook called `on_mount()`
    // that is called after the component has been attached to the vdom.
    // It seems like this only works because the JS engine decides to activate the
    // router worker logic after the mounting has finished.
    router.send(router::Request::GetCurrentRoute);

    Model {
      child: Child::Board, // This should be quickly overwritten by the actual route.
      router,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::NavigateTo(child) => {
        let path_segments = match child {
          Child::Board => vec!["board".into()],
          Child::UserList => vec!["users".into()],
          Child::PathNotFound(_) => vec!["path_not_found".into()],
        };

        let route = router::Route {
          path_segments,
          query: None,
          fragment: None,
          state: (),
        };

        self.router.send(router::Request::ChangeRoute(route));
        false
      }
      Msg::HandleRoute(route) => {
        info!("Routing: {}", route.to_route_string());
        // Instead of each component selecting which parts of the path are important to it,
        // it is also possible to match on the `route.to_route_string().as_str()` once
        // and create enum variants representing the different children and pass them as props.
        self.child = if let Some(first_segment) = route.path_segments.get(0) {
          match first_segment.as_str() {
            "board" => Child::Board,
            "users" => Child::UserList,
            other => Child::PathNotFound(other.into()),
          }
        } else {
          Child::PathNotFound("path_not_found".into())
        };

        true
      }
    }
  }
}

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    html! {
      <div>
        {view_nav()}
        <section class="section",>
          {self.child.view()}
        </section>
      </div>
    }
  }
}

impl Renderable<Model> for Child {
  fn view(&self) -> Html<Model> {
    match *self {
      Child::Board => html! {
          <>
              <BoardModel: />
          </>
      },
      Child::UserList => html! {
          <>
              <UserListModel: />
          </>
      },
      Child::PathNotFound(ref path) => html! {
          <>
              {format!("Not found: '{}'", path)}
          </>
      },
    }
  }
}

fn view_nav() -> Html<Model> {
  html! {
    <nav class="navbar", role="navigation", aria-label="main navigation",>
      <div class="navbar-menu",>
        <div class="navbar-start",>
          <a class="navbar-item", onclick=|_| Msg::NavigateTo(Child::Board),>
            { "board" }
          </a>
          <a class="navbar-item", onclick=|_| Msg::NavigateTo(Child::UserList),>
            { "users" }
          </a>
        </div>
      </div>
    </nav>
  }
}
