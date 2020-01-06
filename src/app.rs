use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::events::IKeyboardEvent;
use yew::format::Json;
use yew::services::storage::{Area, StorageService};
use yew::{html, Component, ComponentLink, Href, Html, Renderable, ShouldRender};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

// Components
mod components;

use self::components::controls::Controls;

const KEY: &'static str = "yew.micromassive.self";

pub struct App {
    storage: StorageService,
    state: State,
    current_room: u8,
    current_click: u64,
    rng: StdRng,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    filter: Filter,
    value: String,
    edit_value: String,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

pub enum Msg {
    // Micromassive!
    StepForward(),
    // TodosMVC
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Nope,
}

fn room_exits(id: u8) -> Option<[u8; 3]> {
  match id {
    1 => Some([2, 5, 8]),
    2 => Some([1, 3, 10]),
    3 => Some([2, 4, 12]),
    4 => Some([3, 5, 14]),
    5 => Some([1, 4, 6]),
    6 => Some([5, 7, 15]),
    7 => Some([6, 8, 17]),
    8 => Some([1, 7, 11]),
    9 => Some([10, 12, 19]),
    10 => Some([2, 9, 11]),
    11 => Some([8, 10, 20]),
    12 => Some([3, 9, 13]),
    13 => Some([12, 14, 18]),
    14 => Some([4, 13, 15]),
    15 => Some([6, 14, 16]),
    16 => Some([15, 17, 18]),
    17 => Some([7, 16, 20]),
    18 => Some([13, 16, 19]),
    19 => Some([9, 18, 20]),
    20 => Some([11, 17, 19]),
    _ => None
  }
}

fn integer_from_float(float: f64, max: u8) -> u8 {
  return (float * (max as f64)) as u8;
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local);
        let entries = {
            if let Json(Ok(restored_entries)) = storage.restore(KEY) {
                restored_entries
            } else {
                Vec::new()
            }
        };
        let state = State {
            entries,
            filter: Filter::All,
            value: "".into(),
            edit_value: "".into(),
        };
        let current_room = 1;
        let current_click = 0;

        // Replace this with a user-provided seed phrase; or generate a new one
        let seed: [u8; 32] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,];
        let rng: StdRng = SeedableRng::from_seed(seed);

        App { storage, state, current_room, current_click, rng }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Micromassive
            Msg::StepForward() => {

              info!("Stepping Forward!");

              info!("{}", integer_from_float(self.rng.gen::<f64>(), 9));

              info!("Step Complete");
              self.current_click = self.current_click + 1;
            }
            // TodosMVC
            Msg::Add => {
                let entry = Entry {
                    description: self.state.value.clone(),
                    completed: false,
                    editing: false,
                };
                self.state.entries.push(entry);
                self.state.value = "".to_string();
            }
            Msg::Edit(idx) => {
                let edit_value = self.state.edit_value.clone();
                self.state.complete_edit(idx, edit_value);
                self.state.edit_value = "".to_string();
            }
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::UpdateEdit(val) => {
                println!("Input: {}", val);
                self.state.edit_value = val;
            }
            Msg::Remove(idx) => {
                self.state.remove(idx);
            }
            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }
            Msg::ToggleEdit(idx) => {
                self.state.edit_value = self.state.entries[idx].description.clone();
                self.state.toggle_edit(idx);
            }
            Msg::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
            }
            Msg::Toggle(idx) => {
                self.state.toggle(idx);
            }
            Msg::ClearCompleted => {
                self.state.clear_completed();
            }
            Msg::Nope => {}
        }
        self.storage.store(KEY, Json(&self.state.entries));
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        info!("rendered!");
        html! {
            <div class="micromassive-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "Micromassive" }</h1>
                        { self.view_input() }
                    </header>
                    <section class="main">
                        <input class="toggle-all" type="checkbox" checked=self.state.is_all_completed() onclick=|_| Msg::ToggleAll />
                        <ul class="todo-list">
                            { for self.state.entries.iter().filter(|e| self.state.filter.fit(e)).enumerate().map(view_entry) }
                        </ul>
                    </section>
                    <footer class="footer">
                        <span class="todo-count">
                            <strong>{ self.state.total() }</strong>
                            { " item(s) left" }
                        </span>
                        <ul class="filters">
                            { for Filter::iter().map(|flt| self.view_filter(flt)) }
                        </ul>
                        <button class="clear-completed" onclick=|_| Msg::ClearCompleted>
                            { format!("Clear completed ({})", self.state.total_completed()) }
                        </button>
                    </footer>
                </section>
                <Controls:
                  exits=room_exits(self.current_room).unwrap(), current_click=self.current_click,
                  onsignal=|msg| msg,
                />
                <footer class="info">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/thomasmost/" target="_blank">{ "Thomas Moore" }</a></p>
                </footer>
            </div>
        }
    }
}

impl App {
  
    fn view_filter(&self, filter: Filter) -> Html<App> {
        let flt = filter.clone();
        html! {
            <li>
                <a class=if self.state.filter == flt { "selected" } else { "not-selected" }
                   href=&flt
                   onclick=|_| Msg::SetFilter(flt.clone())>
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self) -> Html<App> {
        html! {
            // You can use standard Rust comments. One line:
            // <li></li>
            <input class="new-todo"
                   placeholder="What needs to be done?"
                   value=&self.state.value
                   oninput=|e| Msg::Update(e.value)
                   onkeypress=|e| {
                       if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
                   } />
            /* Or multiline:
            <ul>
                <li></li>
            </ul>
            */
        }
    }
}

fn view_entry((idx, entry): (usize, &Entry)) -> Html<App> {
    let mut class = "todo".to_string();
    if entry.editing {
        class.push_str(" editing");
    }
    if entry.completed {
        class.push_str(" completed");
    }
    html! {
        <li class=class>
            <div class="view">
                <input class="toggle" type="checkbox" checked=entry.completed onclick=|_| Msg::Toggle(idx) />
                <label ondoubleclick=|_| Msg::ToggleEdit(idx)>{ &entry.description }</label>
                <button class="destroy" onclick=|_| Msg::Remove(idx) />
            </div>
            { view_entry_edit_input((idx, &entry)) }
        </li>
    }
}

fn view_entry_edit_input((idx, entry): (usize, &Entry)) -> Html<App> {
    if entry.editing {
        html! {
            <input class="edit"
                   type="text"
                   value=&entry.description
                   oninput=|e| Msg::UpdateEdit(e.value)
                   onblur=|_| Msg::Edit(idx)
                   onkeypress=|e| {
                      if e.key() == "Enter" { Msg::Edit(idx) } else { Msg::Nope }
                   } />
        }
    } else {
        html! { <input type="hidden" /> }
    }
}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl<'a> Into<Href> for &'a Filter {
    fn into(self) -> Href {
        match *self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Completed => "#/completed".into(),
        }
    }
}

impl Filter {
    fn fit(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }
}

impl State {
    fn total(&self) -> usize {
        self.entries.len()
    }

    fn total_completed(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| Filter::Completed.fit(e))
            .count()
    }

    fn is_all_completed(&self) -> bool {
        let mut filtered_iter = self
            .entries
            .iter()
            .filter(|e| self.filter.fit(e))
            .peekable();

        if filtered_iter.peek().is_none() {
            return false;
        }

        filtered_iter.all(|e| e.completed)
    }

    fn toggle_all(&mut self, value: bool) {
        for entry in self.entries.iter_mut() {
            if self.filter.fit(entry) {
                entry.completed = value;
            }
        }
    }

    fn clear_completed(&mut self) {
        let entries = self
            .entries
            .drain(..)
            .filter(|e| Filter::Active.fit(e))
            .collect();
        self.entries = entries;
    }

    fn toggle(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.completed = !entry.completed;
    }

    fn toggle_edit(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.editing = !entry.editing;
    }

    fn complete_edit(&mut self, idx: usize, val: String) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.description = val;
        entry.editing = !entry.editing;
    }

    fn remove(&mut self, idx: usize) {
        let idx = {
            let filter = self.filter.clone();
            let entries = self
                .entries
                .iter()
                .enumerate()
                .filter(|&(_, e)| filter.fit(e))
                .collect::<Vec<_>>();
            let &(idx, _) = entries.get(idx).unwrap();
            idx
        };
        self.entries.remove(idx);
    }
}
