use crate::components::keyboard_shortcuts_handler::use_keyboard_shortcuts;
use crate::components::theme_manager::use_theme_manager;
use crate::components::todo_state::use_todo_state;
use crate::components::{
    AppHeader, FilterBar, KeyboardShortcuts, SearchBox, TagsFilter, TodoForm,
    TodoList as TodoListComponent,
};
use crate::models::FilterState;
use crate::utils::constants::todo::DEFAULT_TAGS;
use crate::utils::theme;
use dioxus::prelude::*;

/// Main component for the Todo application.
#[component]
pub fn TodoApp() -> Element {
    // Theme management
    let (is_dark_mode, toggle_theme) = use_theme_manager();

    // Todo state management
    let (todo_list, mut filter, mut selected_tag, operations, sorted_tags) =
        use_todo_state(&DEFAULT_TAGS);

    // Search state
    let mut search_text = use_signal(String::new);

    // Extract operations
    let add_todo = operations.add_todo;
    let toggle_todo = operations.toggle_todo;
    let delete_todo = operations.delete_todo;
    let update_todo = operations.update_todo;
    let set_due_date = operations.set_due_date;
    let add_tag_to_todo = operations.add_tag_to_todo;
    let remove_tag_from_todo = operations.remove_tag_from_todo;
    let mut clear_completed = operations.clear_completed;
    let reorder_todo = operations.reorder_todo;

    // Set filter handler
    let change_filter = move |new_filter: FilterState| {
        filter.set(new_filter);
    };

    // Selected tag handler
    let select_tag = move |tag: Option<String>| {
        selected_tag.set(tag);
    };

    // Search handler
    let on_search = move |text: String| {
        search_text.set(text);
    };

    // Keyboard shortcut handler
    let handle_key_down = use_keyboard_shortcuts(change_filter, toggle_theme.clone());

    // Get current todos as vector
    let todos = todo_list.read().all();
    let active_count = todo_list.read().active_count();
    let completed_count = todo_list.read().completed_count();

    // Get container class from theme utilities
    let container_class = theme::container_class(is_dark_mode());

    rsx! {
        div {
            class: "h-full {container_class} py-8 px-4",
            tabindex: "0",
            onkeydown: handle_key_down,

            div { class: "max-w-2xl mx-auto sm:px-6 lg:px-8",

                // App header
                AppHeader {
                    title: "Dioxus Todo App",
                    is_dark_mode: is_dark_mode(),
                    on_toggle_theme: toggle_theme,
                }

                // Todo form
                TodoForm { on_add: add_todo, is_dark_mode: is_dark_mode() }

                // Search box
                SearchBox {
                    search_term: search_text(),
                    on_search,
                    is_dark_mode: is_dark_mode(),
                }

                // Tags filter
                TagsFilter {
                    tags: sorted_tags.clone(),
                    selected_tag: selected_tag(),
                    on_select_tag: select_tag,
                    is_dark_mode: is_dark_mode(),
                }

                // Todo list
                div { class: "transition-all duration-300 mt-4",
                    TodoListComponent {
                        todos,
                        filter: filter(),
                        search_text: search_text(),
                        on_toggle: toggle_todo,
                        on_delete: delete_todo,
                        on_update: update_todo,
                        on_due_date_change: set_due_date,
                        on_tag_add: add_tag_to_todo,
                        on_tag_remove: remove_tag_from_todo,
                        on_reorder: reorder_todo,
                        selected_tag: selected_tag(),
                        is_dark_mode: is_dark_mode(),
                        default_tags: Some(DEFAULT_TAGS.iter().map(|s| s.to_string()).collect()),
                    }
                }

                // Filter bar
                FilterBar {
                    filter: filter(),
                    on_filter_change: change_filter,
                    active_count,
                    completed_count,
                    on_clear_completed: move |_| clear_completed(()),
                    is_dark_mode: is_dark_mode(),
                }

                // Keyboard shortcuts help
                KeyboardShortcuts { is_dark_mode: is_dark_mode() }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::dioxus_core::Mutations;

    #[test]
    fn test_todo_app_rendering() {
        let mut app = VirtualDom::new(|| {
            rsx! {
                TodoApp {}
            }
        });

        app.rebuild(&mut Mutations::default());
        // Note: In a real test environment, you would want to verify the rendered output
        // This is a basic structure that can be expanded with more detailed assertions
    }

    #[test]
    fn test_todo_app_initial_state() {
        let mut app = VirtualDom::new(|| {
            rsx! {
                TodoApp {}
            }
        });

        app.rebuild(&mut Mutations::default());
        // Note: In a real test environment, you would want to verify the initial state
        // This includes checking that the todo list is empty and filter is set to All
    }

    #[test]
    fn test_todo_app_components_rendering() {
        let mut app = VirtualDom::new(|| {
            rsx! {
                TodoApp {}
            }
        });

        app.rebuild(&mut Mutations::default());
        // Note: In a real test environment, you would want to verify that all child components
        // (TodoForm, TodoList, FilterBar) are properly rendered
    }
}
