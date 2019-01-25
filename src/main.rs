#[macro_use]
extern crate yew;

use std::cmp::PartialEq;
use std::convert::From;
use std::fmt;
use yew::prelude::*;
use yew::virtual_dom::VNode;

enum Msg {
    IncreaseStatus(usize),
    DecreaseStatus(usize),
    UpdateNewTaskName(String),
    UpdateNewTaskAssignee(yew::html::ChangeData),
    UpdateNewTaskMandays(String),
    NewTask,
}

enum Status {
    ToDo,
    InProgress,
    Review,
    Testing,
    Ready,
    Done,
}

impl Status {
    fn left(&self) -> Self {
        match self {
            Status::Done => Status::Ready,
            Status::Ready => Status::Testing,
            Status::Testing => Status::Review,
            Status::Review => Status::InProgress,
            Status::InProgress => Status::ToDo,
            Status::ToDo => Status::ToDo,
        }
    }

    fn right(&self) -> Self {
        match self {
            Status::ToDo => Status::InProgress,
            Status::InProgress => Status::Review,
            Status::Review => Status::Testing,
            Status::Testing => Status::Ready,
            Status::Ready => Status::Done,
            Status::Done => Status::Done,
        }
    }

    fn name(&self) -> &str {
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

struct Task {
    name: String,
    assignee: String,
    estimate: u32,
    status: Status,
}

impl Task {
    fn can_left(&self) -> bool {
        self.status != Status::ToDo
    }

    fn can_right(&self) -> bool {
        self.status != Status::Done
    }
}

struct State {
    tasks: Vec<Task>,
    new_task_name: String,
    new_task_assignee: String,
    new_task_estimate: u32,
}

impl State {
    fn find_task_by(&mut self, idx: usize) -> Option<&mut Task> {
        self.tasks.get_mut(idx)
    }

    fn add_new_task(&mut self, name: String, assignee: String, estimate: u32) {
        self.new_task_assignee = "".to_string();
        self.new_task_estimate = 0;
        self.new_task_name = "".to_string();

        self.tasks.push(Task {
            name,
            assignee,
            estimate,
            status: Status::ToDo,
        });
    }
}

struct Model {
    state: State,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            state: State {
                tasks: vec![
                    Task {
                        name: "Task 1".to_string(),
                        assignee: "🐱".to_string(),
                        estimate: 3,
                        status: Status::InProgress,
                    },
                    Task {
                        name: "Task 2".to_string(),
                        assignee: "🐶".to_string(),
                        estimate: 2,
                        status: Status::ToDo,
                    },
                    Task {
                        name: "Task 3".to_string(),
                        assignee: "🐱".to_string(),
                        estimate: 1,
                        status: Status::ToDo,
                    },
                    Task {
                        name: "Task 4".to_string(),
                        assignee: "🐹".to_string(),
                        estimate: 3,
                        status: Status::Done,
                    },
                ],
                new_task_name: "".to_string(),
                new_task_assignee: "".to_string(),
                new_task_estimate: 0,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateNewTaskName(val) => {
                self.state.new_task_name = val;
            }

            Msg::UpdateNewTaskAssignee(val) => {
                if let yew::html::ChangeData::Select(v) = &val {
                    self.state.new_task_assignee = v.raw_value();
                }
            }

            Msg::UpdateNewTaskMandays(val) => {
                if let Ok(v) = u32::from_str_radix(&val, 10) {
                    self.state.new_task_estimate = v;
                }
            }

            Msg::NewTask => self.state.add_new_task(
                self.state.new_task_name.clone(),
                self.state.new_task_assignee.clone(),
                self.state.new_task_estimate,
            ),

            Msg::IncreaseStatus(idx) => match self.state.find_task_by(idx) {
                None => (),
                Some(task) => {
                    task.status = task.status.right();
                }
            },

            Msg::DecreaseStatus(idx) => match self.state.find_task_by(idx) {
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
        html! {
            <section class="section", id="board",>
                <div class="container header",>
                    { view_header(&self.state) }
                </div>
                <div class="container",>
                    <div class="columns",>
                        { view_column(Status::ToDo, &self.state.tasks) }
                        { view_column(Status::InProgress, &self.state.tasks) }
                        { view_column(Status::Review, &self.state.tasks) }
                        { view_column(Status::Testing, &self.state.tasks) }
                        { view_column(Status::Ready, &self.state.tasks) }
                        { view_column(Status::Done, &self.state.tasks) }
                    </div>
                </div>
            </section>
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

fn view_header(state: &State) -> Html<Model> {
    html! {
        <div class="columns",>
            <div class="column is-half",>
                <input class="input", type="text", value=&state.new_task_name, oninput=|e| Msg::UpdateNewTaskName(e.value), />
            </div>

            <div class="column",>
                {view_assignee_select(state)}
            </div>

            <div class="column",>
                <input class="input", type="text", value=&state.new_task_estimate, oninput=|e| Msg::UpdateNewTaskMandays(e.value), />
            </div>

            <div class="column",>
                <button class="button is-fullwidth", onclick=|_| Msg::NewTask,>{ "+" }</button>
            </div>
        </div>
    }
}

fn view_assignee_select(state: &State) -> Html<Model> {
    html! {
        <div class="select is-fullwidth",>
            <select value=&state.new_task_assignee, onchange=|e| Msg::UpdateNewTaskAssignee(e),>
                <option value="🐱",>{ "🐱" }</option>
                <option value="🐶",>{ "🐶" }</option>
                <option value="🐹",>{ "🐹" }</option>
            </select>
        </div>
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
