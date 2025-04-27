use crate::models::{FilterState, TodoList};
use crate::utils;
use crate::utils::constants::storage::TODO_STORAGE_KEY;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use std::collections::HashSet;

// Type definition for the due date callback
pub type DueDateCallback = Box<dyn FnMut((usize, Option<DateTime<Utc>>)) + 'static>;

pub struct TodoOperations {
    pub add_todo: Box<dyn FnMut(String) + 'static>,
    pub toggle_todo: Box<dyn FnMut(usize) + 'static>,
    pub delete_todo: Box<dyn FnMut(usize) + 'static>,
    pub update_todo: Box<dyn FnMut((usize, String)) + 'static>,
    pub set_due_date: DueDateCallback,
    pub add_tag_to_todo: Box<dyn FnMut((usize, String)) + 'static>,
    pub remove_tag_from_todo: Box<dyn FnMut((usize, String)) + 'static>,
    pub clear_completed: Box<dyn FnMut(()) + 'static>,
    pub reorder_todo: Box<dyn FnMut((usize, usize)) + 'static>,
}

// Type definition for the return value of use_todo_state
pub type TodoStateReturn = (
    Signal<TodoList>,
    Signal<FilterState>,
    Signal<Option<String>>,
    TodoOperations,
    Vec<String>,
);

pub fn use_todo_state(default_tags: &[&str]) -> TodoStateReturn {
    // State
    let mut todo_list = use_signal(TodoList::default);
    let filter = use_signal(|| FilterState::All);
    let mut selected_tag = use_signal(|| None::<String>);

    // Load todos from localStorage on component mount
    use_effect(move || {
        if let Ok(loaded_todos) = utils::load::<TodoList>(TODO_STORAGE_KEY) {
            todo_list.set(loaded_todos);
        }
    });

    // Save todos to localStorage whenever they change
    use_effect(move || {
        let _ = utils::save(TODO_STORAGE_KEY, &todo_list.read() as &TodoList);
    });

    // Event handlers
    let add_todo = Box::new(move |text: String| {
        let list = &mut todo_list.write();
        list.add(text);
    });

    let toggle_todo = Box::new(move |id: usize| {
        let list = &mut todo_list.write();
        list.toggle(id);
    });

    let delete_todo = Box::new(move |id: usize| {
        let list = &mut todo_list.write();
        list.remove(id);
    });

    let update_todo = Box::new(move |(id, text): (usize, String)| {
        let list = &mut todo_list.write();
        list.update_text(id, text);
    });

    let set_due_date = Box::new(move |(id, date): (usize, Option<DateTime<Utc>>)| {
        let list = &mut todo_list.write();
        list.set_due_date(id, date);
    });

    let add_tag_to_todo = Box::new(move |(id, tag): (usize, String)| {
        let list = &mut todo_list.write();
        list.add_tag(id, tag);
    });

    let remove_tag_from_todo = Box::new(move |(id, tag): (usize, String)| {
        let list = &mut todo_list.write();
        list.remove_tag(id, &tag);
    });

    let clear_completed = Box::new(move |_| {
        let list = &mut todo_list.write();
        list.clear_completed();
    });

    let reorder_todo = Box::new(move |(source_id, target_id): (usize, usize)| {
        let list = &mut todo_list.write();
        list.reorder(source_id, target_id);
    });

    let _select_tag = move |tag: Option<String>| {
        selected_tag.set(tag);
    };

    // Combine default and user tags, ensuring uniqueness and sorting
    let all_current_tags = todo_list.read().all_tags();
    let mut combined_tags = default_tags
        .iter()
        .map(|&s| s.to_string())
        .collect::<HashSet<_>>();

    combined_tags.extend(all_current_tags);
    let mut sorted_tags = combined_tags.into_iter().collect::<Vec<_>>();
    sorted_tags.sort();

    let operations = TodoOperations {
        add_todo,
        toggle_todo,
        delete_todo,
        update_todo,
        set_due_date,
        add_tag_to_todo,
        remove_tag_from_todo,
        clear_completed,
        reorder_todo,
    };

    (todo_list, filter, selected_tag, operations, sorted_tags)
}
