#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate yew;
extern crate stdweb;

use yew::prelude::App;

mod components;
mod models;
mod router;
mod routing;

use crate::components::app::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
