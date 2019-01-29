use std::convert::From;
use yew::prelude::{Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::virtual_dom::VNode;

use crate::models::status::Status;
use crate::models::task::Task;

pub enum Msg {
    IncreaseStatus(usize),
    DecreaseStatus(usize),
    UpdateNewTaskName(String),
    UpdateNewTaskAssignee(yew::html::ChangeData),
    UpdateNewTaskMandays(String),
    NewTask,
}

pub struct Model {
    tasks: Vec<Task>,
    new_task: Task,
}

impl Model {
    fn find_task_by(&mut self, idx: usize) -> Option<&mut Task> {
        self.tasks.get_mut(idx)
    }

    fn clear_form(&mut self) {
        self.new_task = Task::create_empty();
    }

    fn add_new_task(&mut self, name: String, assignee: String, estimate: u32) {
        self.tasks.push(Task {
            name,
            assignee,
            estimate,
            status: Status::ToDo,
        });
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            tasks: vec![],
            new_task: Task::create_empty(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateNewTaskName(val) => {
                self.new_task.name = val;
            }

            Msg::UpdateNewTaskAssignee(val) => {
                if let yew::html::ChangeData::Select(v) = &val {
                    self.new_task.assignee = v.raw_value();
                }
            }

            Msg::UpdateNewTaskMandays(val) => {
                if let Ok(v) = u32::from_str_radix(&val, 10) {
                    self.new_task.estimate = v;
                }
            }

            Msg::NewTask => {
                self.add_new_task(
                    self.new_task.name.clone(),
                    self.new_task.assignee.clone(),
                    self.new_task.estimate,
                );
                self.clear_form();
            }

            Msg::IncreaseStatus(idx) => match self.find_task_by(idx) {
                None => (),
                Some(task) => {
                    task.status = task.status.right();
                }
            },

            Msg::DecreaseStatus(idx) => match self.find_task_by(idx) {
                None => (),
                Some(task) => {
                    task.status = task.status.left();
                }
            },
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let Model { tasks, .. } = &self;

        html! {
            <div class="container",>
                { view_header(&self) }
                <div class="columns",>
                    { view_column(Status::ToDo, tasks) }
                    { view_column(Status::InProgress, tasks) }
                    { view_column(Status::Review, tasks) }
                    { view_column(Status::Testing, tasks) }
                    { view_column(Status::Ready, tasks) }
                    { view_column(Status::Done, tasks) }
                </div>
            </div>
        }
    }
}

fn view_column(status: Status, tasks: &Vec<Task>) -> Html<Model> {
    html! {
        <div class=format!("column is-2 status-{}", status.to_string()),>
            <div class="tags has-addons",>
                <span class="tag",>{ status.name() }</span>
                <span class="tag is-dark",>{ tasks.iter().filter(|e| e.status == status).count() }</span>
            </div>
            { for tasks.iter().enumerate().filter(|(_, e)| e.status == status).map(view_task) }
        </div>
    }
}

fn view_task((idx, task): (usize, &Task)) -> Html<Model> {
    let button_left: VNode<Model> = match task.can_left() {
        true => html! {
            <button class="button is-small is-white", onclick=|_| Msg::DecreaseStatus(idx),>{ "◀︎" }</button>
        },
        false => html! {
            <></>
        },
    };

    let button_right: VNode<Model> = match task.can_right() {
        true => html! {
            <button class="button is-small is-white", onclick=|_| Msg::IncreaseStatus(idx),>{ "▶︎︎" }</button>
        },
        false => html! {
            <></>
        },
    };

    html! {
        <div class="card",>
            <div class="card-content",>
                { &task.name }
            </div>
            <footer class="card-footer",>
                <div class="card-footer-item",>
                    { &task.assignee }
                </div>
                <div class="card-footer-item",>
                    { format!("{}h", &task.estimate) }
                </div>
            </footer>
            <footer class="card-footer",>
                <span class="card-footer-item",>
                    {button_left}
                </span>
                <span class="card-footer-item",>
                    {button_right}
                </span>
            </footer>
        </div>
    }
}

fn view_header(model: &Model) -> Html<Model> {
    html! {
        <div class="columns",>
            <div class="column is-half",>
                <input class="input", type="text", value=&model.new_task.name, oninput=|e| Msg::UpdateNewTaskName(e.value), />
            </div>

            <div class="column",>
                {view_assignee_select(model)}
            </div>

            <div class="column",>
                <input class="input", type="text", value=&model.new_task.estimate, oninput=|e| Msg::UpdateNewTaskMandays(e.value), />
            </div>

            <div class="column",>
                <button class="button is-fullwidth", onclick=|_| Msg::NewTask,>{ "+" }</button>
            </div>
        </div>
    }
}

fn view_assignee_select(model: &Model) -> Html<Model> {
    html! {
        <div class="select is-fullwidth",>
            <select value=&model.new_task.assignee, onchange=|e| Msg::UpdateNewTaskAssignee(e),>
                <option value="🐱",>{ "🐱" }</option>
                <option value="🐶",>{ "🐶" }</option>
                <option value="🐹",>{ "🐹" }</option>
            </select>
        </div>
    }
}
