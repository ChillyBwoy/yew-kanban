#[macro_use]
extern crate yew;

use yew::prelude::App;

mod components;
mod models;

use crate::components::board::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
