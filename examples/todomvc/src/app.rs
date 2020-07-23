use sauron::{
    dom::events::KeyboardEvent,
    html::{attributes::*, *},
    prelude::*,
    Cmd, Component, Node,
};

pub struct Model {
    entries: Vec<Entry>,
    filter: Filter,
    value: String,
    edit_value: String,
}

struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

pub enum Msg {
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

impl Model {
    pub fn new() -> Self {
        Model {
            entries: vec![],
            filter: Filter::All,
            value: "".into(),
            edit_value: "".into(),
        }
    }
}

impl Component<Msg> for Model {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Add => {
                let entry = Entry {
                    description: self.value.clone(),
                    completed: false,
                    editing: false,
                };
                self.entries.push(entry);
                self.value = "".to_string();
            }
            Msg::Edit(idx) => {
                let edit_value = self.edit_value.clone();
                self.complete_edit(idx, edit_value);
                self.edit_value = "".to_string();
            }
            Msg::Update(val) => {
                self.value = val;
            }
            Msg::UpdateEdit(val) => {
                self.edit_value = val;
            }
            Msg::Remove(idx) => {
                self.remove(idx);
            }
            Msg::SetFilter(filter) => {
                self.filter = filter;
            }
            Msg::ToggleEdit(idx) => {
                self.edit_value = self.entries[idx].description.clone();
                self.toggle_edit(idx);
            }
            Msg::ToggleAll => {
                let status = !self.is_all_completed();
                self.toggle_all(status);
            }
            Msg::Toggle(idx) => {
                self.toggle(idx);
            }
            Msg::ClearCompleted => {
                self.clear_completed();
            }
            Msg::Nope => {}
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        node! {
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>"todos"</h1>
                        {self.view_input()}
                    </header>
                    <section class="main">
                        <input
                            class="toggle-all"
                            r#type="checkbox"
                            checked={self.is_all_completed()}
                            on_click={|_| Msg::ToggleAll} />
                        <ul class="todo-list">
                            {for (i, e) in self.entries.iter().filter(|e| self.filter.fit(e)).enumerate() {
                                view_entry(i, e)
                            }}
                        </ul>
                    </section>
                    <footer class="footer">
                        <span class="todo-count">
                            <strong>{text(format!(
                                "{}",
                                self.total()
                            ))}" item(s) left"</strong>
                        </span>
                        <ul class="filters">
                            {self.view_filter(Filter::All)}
                            {self.view_filter(Filter::Active)}
                            {self.view_filter(Filter::Completed)}
                        </ul>
                        <button class="clear-completed" on_click={|_| Msg::ClearCompleted}>
                            {text(format!(
                                "Clear completed ({})",
                                self.total_completed()
                            ))}
                        </button>
                    </footer>
                </section>
                <footer class="info">
                    <p>"Double-click to edit a todo"</p>
                    <p>"Written by " <a href="https://github.com/ivanceras/" target="_blank">"Jovansonlee Cesar"</a></p>
                    <p>"Part of " <a href="http://todomvc.com/" target="_blank">"TodoMVC"</a></p>
                </footer>
            </div>
        }
    }
}

impl Model {
    fn view_filter(&self, filter: Filter) -> Node<Msg> {
        node! {
            <li class={if self.filter == filter {
                "selected"
            } else {
                "not-selected"
            }}
            href={filter.to_string()}
            on_click={move |_| Msg::SetFilter(filter)}>
                {text(filter.to_string())}
            </li>
        }
    }

    fn view_input(&self) -> Node<Msg> {
        node! {
            <input
                class="new-todo"
                id="new-todo"
                placeholder="What needs to be done?"
                value={self.value.to_string()}
                on_input={|v: InputEvent| Msg::Update(v.value.to_string())}
                on_keypress={|event: KeyboardEvent| {
                    if event.key() == "Enter" {
                        Msg::Add
                    } else {
                        Msg::Nope
                    }
                }} />
        }
    }
}

fn view_entry(idx: usize, entry: &Entry) -> Node<Msg> {
    node! {
        <li {classes_flag([
            ("todo", true),
            ("editing", entry.editing),
            ("completed", entry.completed),
        ])}>
            <div class="view">
                <input class="toggle" r#type="checkbox" checked={entry.completed} on_click={move |_| Msg::Toggle(idx)} />
                <label on_doubleclick={move |_| Msg::ToggleEdit(idx)}>{text(entry.description.clone())}</label>
                <button class="destroy" on_click={move |_| Msg::Remove(idx)} />
            </div>
            { view_entry_edit_input((idx, &entry)) }
        </li>
    }
}

fn view_entry_edit_input((idx, entry): (usize, &Entry)) -> Node<Msg> {
    if entry.editing {
        input(
            vec![
                class("edit"),
                r#type("text"),
                value(&entry.description),
                on_input(|input: InputEvent| {
                    Msg::UpdateEdit(input.value.to_string())
                }),
                on_blur(move |_| Msg::Edit(idx)),
                on_keypress(move |event: KeyboardEvent| {
                    if event.key() == "Enter" {
                        Msg::Edit(idx)
                    } else {
                        Msg::Nope
                    }
                }),
            ],
            vec![],
        )
    } else {
        input(vec![r#type("hidden")], vec![])
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl ToString for Filter {
    fn to_string(&self) -> String {
        match *self {
            Filter::All => "#/".to_string(),
            Filter::Active => "#/active".to_string(),
            Filter::Completed => "#/completed".to_string(),
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

impl Model {
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
