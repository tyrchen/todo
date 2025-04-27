use super::todo_item::TodoItem;
use crate::models::{FilterState, Todo};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

/// Props for the TodoList component.
#[derive(Props, PartialEq, Clone)]
pub struct TodoListProps {
    /// The list of todos to display
    pub todos: Vec<Todo>,
    /// The current filter state
    pub filter: FilterState,
    /// Search text to filter todos by
    #[props(default = String::new())]
    pub search_text: String,
    /// Callback when a todo is toggled
    pub on_toggle: EventHandler<usize>,
    /// Callback when a todo is deleted
    pub on_delete: EventHandler<usize>,
    /// Callback when a todo is updated
    pub on_update: EventHandler<(usize, String)>,
    /// Callback when a todo's due date is changed
    pub on_due_date_change: EventHandler<(usize, Option<DateTime<Utc>>)>,
    /// Callback when a tag is added to a todo
    pub on_tag_add: EventHandler<(usize, String)>,
    /// Callback when a tag is removed from a todo
    pub on_tag_remove: EventHandler<(usize, String)>,
    /// Callback when a todo is reordered via drag and drop
    pub on_reorder: EventHandler<(usize, usize)>,
    /// Optional selected tag for filtering
    pub selected_tag: Option<String>,
    /// Whether dark mode is enabled
    #[props(default = false)]
    pub is_dark_mode: bool,
    /// List of default tags to suggest
    pub default_tags: Option<Vec<String>>,
}

/// Component that renders a list of TodoItems.
#[component]
pub fn TodoList(props: TodoListProps) -> Element {
    // State to track drag and drop
    let mut drag_item = use_signal(|| None::<usize>);
    let mut drag_over_item = use_signal(|| None::<usize>);

    // Dynamic classes based on dark mode
    let container_bg_class = if props.is_dark_mode {
        "bg-gray-800"
    } else {
        "bg-white"
    };
    let text_class = if props.is_dark_mode {
        "text-gray-400"
    } else {
        "text-gray-500"
    };
    let border_class = if props.is_dark_mode {
        "divide-gray-700 border-gray-700"
    } else {
        "divide-gray-200 border-gray-200"
    };

    // Filter todos based on the current filter state, selected tag, and search text
    let filtered_todos = props
        .todos
        .iter()
        .filter(|todo| {
            // Filter state match
            let filter_match = props.filter.matches(todo);

            // Tag match
            let tag_match = match &props.selected_tag {
                Some(tag) => todo.tags.contains(tag),
                None => true,
            };

            // Search text match
            let search_match = if props.search_text.is_empty() {
                true
            } else {
                // Case-insensitive search
                let search_term = props.search_text.to_lowercase();
                let todo_text = todo.text.to_lowercase();

                // Search in todo text
                let text_match = todo_text.contains(&search_term);

                // Search in tags
                let tags_match = todo
                    .tags
                    .iter()
                    .any(|tag| tag.to_lowercase().contains(&search_term));

                text_match || tags_match
            };

            filter_match && tag_match && search_match
        })
        .cloned()
        .collect::<Vec<_>>();

    // Provide an empty Vec if default_tags is None
    let default_tags_list = props.default_tags.clone().unwrap_or_default();

    // Drag handlers
    let on_reorder = props.on_reorder;

    // Determine empty state message
    let empty_state_message = if props.todos.is_empty() {
        "Add your first todo above! ✨".to_string()
    } else if !props.search_text.is_empty() {
        format!("No todos match your search: '{}'", props.search_text)
    } else if props.selected_tag.is_some() {
        "No todos found with the selected tag.".to_string()
    } else {
        match props.filter {
            FilterState::Active => "All tasks done! 🎉".to_string(),
            FilterState::Completed => "No completed tasks yet.".to_string(),
            FilterState::All => "No tasks match the current filter.".to_string(),
        }
    };

    rsx! {
        div { class: "{container_bg_class} rounded-lg shadow-md overflow-hidden transition-colors duration-300 border {border_class} h-[400px] overflow-y-auto",

            if filtered_todos.is_empty() {
                div { class: "p-8 text-center {text_class} transition-colors duration-300 text-lg italic",
                    "{empty_state_message}"
                }
            } else {
                ul { class: "divide-y {border_class} transition-colors duration-300 h-max ",
                    for todo in filtered_todos {
                        {
                            let todo_id = todo.id;
                            rsx! {
                                li {
                                    key: "todo-{todo_id}",
                                    class: "relative transition-colors duration-200 cursor-move",
                                    draggable: "true",
                                    ondragstart: move |_| {
                                        drag_item.set(Some(todo_id));
                                    },
                                    ondragenter: move |_| {
                                        drag_over_item.set(Some(todo_id));
                                    },
                                    ondragend: move |_: Event<DragData>| {
                                        if let (Some(source_id), Some(target_id)) = (drag_item(), drag_over_item()) {
                                            if source_id != target_id {
                                                on_reorder.call((source_id, target_id));
                                            }
                                        }
                                        drag_item.set(None);
                                        drag_over_item.set(None);
                                    },
                                    ondragover: move |evt| evt.prevent_default(),

                                    // Add subtle highlight when dragging over this item
                                    style: if drag_over_item() == Some(todo_id) && drag_item() != Some(todo_id) { "box-shadow: inset 0 -2px 0 0 rgba(79, 70, 229, 0.5); background-color: rgba(79, 70, 229, 0.1);" } else { "" },

                                    TodoItem {
                                        todo: todo.clone(),
                                        on_toggle: props.on_toggle,
                                        on_delete: props.on_delete,
                                        on_update: props.on_update,
                                        on_due_date_change: props.on_due_date_change,
                                        on_tag_add: props.on_tag_add,
                                        on_tag_remove: props.on_tag_remove,
                                        is_dark_mode: props.is_dark_mode,
                                        default_tags: default_tags_list.clone(),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
