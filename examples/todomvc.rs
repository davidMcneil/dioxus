#![allow(non_upper_case_globals, non_snake_case)]
use dioxus::prelude::*;
use im_rc::HashMap;
use std::rc::Rc;

fn main() -> anyhow::Result<()> {
    dioxus::desktop::launch(App, |c| c)
}

#[derive(PartialEq)]
pub enum FilterState {
    All,
    Active,
    Completed,
}

#[derive(Debug, PartialEq)]
pub struct TodoItem {
    pub id: u32,
    pub checked: bool,
    pub contents: String,
}

const STYLE: &str = include_str!("./_examples/todomvc/style.css");
const App: FC<()> = |cx| {
    let draft = use_state(cx, || "".to_string());
    let todos = use_state(cx, || HashMap::<u32, Rc<TodoItem>>::new());
    let filter = use_state(cx, || FilterState::All);

    let todolist = todos
        .iter()
        .filter(|(id, item)| match *filter {
            FilterState::All => true,
            FilterState::Active => !item.checked,
            FilterState::Completed => item.checked,
        })
        .map(|(id, todo)| {
            rsx!(TodoEntry {
                key: "{id}",
                todo: todo.clone()
            })
        })
        .collect::<Vec<_>>();

    let items_left = todolist.len();
    let item_text = match items_left {
        1 => "item",
        _ => "items",
    };

    cx.render(rsx! {
        div { id: "app"
            style {"{STYLE}"}
            div {
                header { class: "header"
                    h1 {"todos"}
                    input {
                        class: "new-todo"
                        placeholder: "What needs to be done?"
                        value: "{draft}"
                        oninput: move |evt| draft.set(evt.value())
                    }
                }
                {todolist}
                {(!todos.is_empty()).then(|| rsx!(
                    footer {
                        span { strong {"{items_left}"} span {"{item_text} left"} }
                        ul { class: "filters"
                            li { class: "All", a { href: "", onclick: move |_| filter.set(FilterState::All), "All" }}
                            li { class: "Active", a { href: "active", onclick: move |_| filter.set(FilterState::Active), "Active" }}
                            li { class: "Completed", a { href: "completed", onclick: move |_| filter.set(FilterState::Completed), "Completed" }}
                        }
                    }
                ))}
            }
            footer { class: "info"
                p {"Double-click to edit a todo"}
                p { "Created by ", a { "jkelleyrtp", href: "http://github.com/jkelleyrtp/" }}
                p { "Part of ", a { "TodoMVC", href: "http://todomvc.com" }}
            }
        }
    })
};

#[derive(PartialEq, Props)]
pub struct TodoEntryProps {
    todo: std::rc::Rc<TodoItem>,
}

pub fn TodoEntry(cx: Context<TodoEntryProps>) -> DomTree {
    let TodoEntryProps { todo } = *cx;
    let is_editing = use_state(cx, || false);
    let contents = "";

    cx.render(rsx! (
        li {
            "{todo.id}"
            input {
                class: "toggle"
                r#type: "checkbox"
                "{todo.checked}"
            }
           {is_editing.then(|| rsx!{
                input {
                    value: "{contents}"
                }
            })}
        }
    ))
}