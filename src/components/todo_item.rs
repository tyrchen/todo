use crate::models::Todo;
use chrono::{DateTime, Local, Utc};
use dioxus::prelude::*;

/// Props for the TodoItem component.
#[derive(Props, PartialEq, Clone)]
pub struct TodoItemProps {
    /// The todo item to display
    pub todo: Todo,
    /// Callback when the todo is toggled
    pub on_toggle: EventHandler<usize>,
    /// Callback when the todo is deleted
    pub on_delete: EventHandler<usize>,
    /// Callback when the todo text is updated
    pub on_update: EventHandler<(usize, String)>,
    /// Callback when the due date is updated
    pub on_due_date_change: EventHandler<(usize, Option<DateTime<Utc>>)>,
    /// Callback when a tag is added
    pub on_tag_add: EventHandler<(usize, String)>,
    /// Callback when a tag is removed
    pub on_tag_remove: EventHandler<(usize, String)>,
    /// Whether dark mode is enabled
    #[props(default = false)]
    pub is_dark_mode: bool,
    /// List of default tags to suggest
    pub default_tags: Option<Vec<String>>,
}

/// Renders a single todo item with toggle, edit, and delete functionality.
#[component]
pub fn TodoItem(props: TodoItemProps) -> Element {
    let todo_id = props.todo.id;
    let initial_text = props.todo.text.clone();
    let todo_tags = props.todo.tags.clone();
    let todo_due_date = props.todo.due_date;
    let todo_completed = props.todo.completed;

    let mut editing = use_signal(|| false);
    let mut edit_text = use_signal(|| initial_text.clone());
    let mut date_editing = use_signal(|| false);
    let mut tag_editing = use_signal(|| false);
    let mut new_tag = use_signal(String::new);

    let default_tags_list = props.default_tags.clone().unwrap_or_default();

    let initial_text_for_toggle = initial_text.clone();
    let toggle_editing = move |_| {
        let current_editing = editing();
        editing.set(!current_editing);
        if !current_editing {
            edit_text.set(initial_text_for_toggle.clone());
            tag_editing.set(false);
            date_editing.set(false);
        }
    };

    let initial_text_for_edit = initial_text.clone();
    let handle_edit = move |evt: Event<FormData>| {
        evt.prevent_default();
        let current_edit_text = edit_text.read().trim().to_string();
        if !current_edit_text.is_empty() && current_edit_text != initial_text_for_edit {
            props.on_update.call((todo_id, current_edit_text));
            editing.set(false);
        } else {
            editing.set(false);
            edit_text.set(initial_text_for_edit.clone());
        }
    };

    let toggle_date_editing = move |_| {
        let is_editing = !date_editing();
        date_editing.set(is_editing);
        if is_editing {
            editing.set(false);
            tag_editing.set(false);
        }
    };

    let handle_date_change = move |evt: Event<FormData>| {
        evt.prevent_default();
        let date_str = evt.value();
        let due_date = if date_str.is_empty() {
            None
        } else {
            DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", date_str))
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
        };
        if due_date != todo_due_date {
            props.on_due_date_change.call((todo_id, due_date));
        }
        date_editing.set(false);
    };

    let toggle_tag_editing = move |_| {
        let is_editing = !tag_editing();
        tag_editing.set(is_editing);
        if is_editing {
            editing.set(false);
            date_editing.set(false);
            new_tag.set(String::new());
        }
    };

    let handle_tag_add = {
        let mut new_tag = new_tag;
        let on_tag_add = props.on_tag_add;
        move |evt: Event<FormData>| {
            evt.prevent_default();
            let tag = new_tag.read().trim().to_string();
            if !tag.is_empty() {
                on_tag_add.call((todo_id, tag));
                new_tag.set(String::new());
            }
        }
    };

    let add_default_tag = {
        let on_tag_add = props.on_tag_add;
        move |tag: String| {
            on_tag_add.call((todo_id, tag));
        }
    };

    let initial_text_for_keypress = initial_text.clone();
    let handle_key_press = {
        let mut editing = editing;
        let mut edit_text = edit_text;
        let mut date_editing = date_editing;
        let mut tag_editing = tag_editing;
        move |evt: Event<KeyboardData>| {
            if evt.key().to_string() == "Escape" {
                if editing() {
                    editing.set(false);
                    edit_text.set(initial_text_for_keypress.clone());
                }
                if date_editing() {
                    date_editing.set(false);
                }
                if tag_editing() {
                    tag_editing.set(false);
                }
            }
        }
    };

    let bg_class = if todo_completed {
        if props.is_dark_mode {
            "bg-gray-800/50 hover:bg-gray-700/50"
        } else {
            "bg-gray-50 hover:bg-gray-100"
        }
    } else if props.is_dark_mode {
        "bg-gray-800 hover:bg-gray-750"
    } else {
        "bg-white hover:bg-gray-50"
    };

    let text_class = if todo_completed {
        "line-through text-gray-500"
    } else if props.is_dark_mode {
        "text-gray-200"
    } else {
        "text-gray-800"
    };

    let border_class = if props.is_dark_mode {
        "border-gray-700"
    } else {
        "border-gray-200"
    };
    let input_bg_class = if props.is_dark_mode {
        "bg-gray-700 text-gray-200 placeholder:text-gray-400"
    } else {
        "bg-white text-gray-800 placeholder:text-gray-400"
    };
    let button_text_class = if props.is_dark_mode {
        "text-gray-400"
    } else {
        "text-gray-500"
    };
    let date_text_class = if props.is_dark_mode {
        "text-gray-400"
    } else {
        "text-gray-600"
    };
    let date_icon_class = if props.is_dark_mode {
        "text-blue-400"
    } else {
        "text-blue-600"
    };
    let tag_bg_class = if props.is_dark_mode {
        "bg-blue-900/70 hover:bg-blue-800/70"
    } else {
        "bg-blue-100 hover:bg-blue-200"
    };
    let tag_text_class = if props.is_dark_mode {
        "text-blue-300"
    } else {
        "text-blue-800"
    };
    let add_tag_button_class = if props.is_dark_mode {
        "bg-green-700 hover:bg-green-600"
    } else {
        "bg-green-500 hover:bg-green-600"
    };
    let tag_suggestion_button_class = if props.is_dark_mode {
        "text-xs px-2.5 py-0.5 rounded-full border border-gray-600 bg-gray-700 text-gray-300 opacity-80 hover:opacity-100 hover:border-gray-500"
    } else {
        "text-xs px-2.5 py-0.5 rounded-full border border-gray-300 bg-gray-100 text-gray-700 opacity-80 hover:opacity-100 hover:border-gray-400"
    };

    let due_date_display =
        todo_due_date.map(|dt| dt.with_timezone(&Local).format("%b %d, %Y").to_string());

    // Add state for tag collapse functionality
    let mut tags_collapsed = use_signal(|| todo_tags.len() > 3);

    // Determine how many tags to show
    let visible_tags = if tags_collapsed() && !tag_editing() {
        todo_tags.iter().take(2).cloned().collect::<Vec<String>>()
    } else {
        todo_tags.clone()
    };

    rsx! {
      li {
        class: "group flex flex-col p-4 border-b {border_class} {bg_class} transition-all duration-200 ease-in-out",
        onkeydown: handle_key_press,

        div { class: "flex items-center w-full",
          div { class: "flex-shrink-0 mr-4",
            input {
              r#type: "checkbox",
              class: "w-5 h-5 text-blue-500 dark:text-blue-400 rounded border-gray-300 dark:border-gray-600 focus:ring-offset-0 focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 dark:bg-gray-700 dark:checked:bg-blue-400 dark:checked:border-blue-400",
              checked: todo_completed,
              onclick: move |_| props.on_toggle.call(todo_id),
              aria_label: "Toggle todo completion",
            }
          }

          div { class: "flex-1 flex flex-wrap items-center gap-1.5",
            if editing() {
              form { class: "flex-1 mr-2", onsubmit: handle_edit,
                input {
                  class: "w-full px-3 py-1.5 border {border_class} {input_bg_class} rounded shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 transition-colors duration-200 text-sm",
                  value: "{edit_text.read()}",
                  oninput: move |evt| edit_text.set(evt.value()),
                  autofocus: true,
                  onblur: {
                      let mut editing = editing;
                      let mut edit_text = edit_text;
                      let initial_text_for_blur = initial_text.clone();
                      move |_| {
                          let current_edit_text = edit_text.read().trim().to_string();
                          if current_edit_text.is_empty() || current_edit_text == initial_text_for_blur
                          {
                              editing.set(false);
                              edit_text.set(initial_text_for_blur.clone());
                          } else {
                              editing.set(false);
                          }
                      }
                  },
                }
              }
            } else {
              div {
                class: "cursor-pointer mr-2 {text_class} transition-colors duration-200 text-sm",
                ondoubleclick: toggle_editing.clone(),
                span { "{initial_text}" }
              }

              // Show tags inline with todo text
              if !todo_tags.is_empty() && !tag_editing() {
                div { class: "flex flex-wrap items-center gap-1.5 ml-2",
                  {
                      visible_tags
                          .iter()
                          .map(|tag| {
                              let tag_clone = tag.clone();
                              rsx! {
                                span {
                                  key: "tag-{tag_clone}",
                                  class: "{tag_bg_class} {tag_text_class} text-xs px-2 py-0.5 rounded-full flex items-center transition-colors duration-200",
                                  span { "{tag}" }
                                }
                              }
                          })
                  }

                  // Show tag count indicator if collapsed
                  if tags_collapsed() && todo_tags.len() > 2 {
                    button {
                      class: "text-xs px-2 py-0.5 rounded-full bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors",
                      onclick: move |_| tags_collapsed.set(false),
                      "+{todo_tags.len() - 2} more"
                    }
                  } else if !tags_collapsed() && todo_tags.len() > 3 {
                    button {
                      class: "text-xs px-2 py-0.5 rounded-full bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors",
                      onclick: move |_| tags_collapsed.set(true),
                      "Show less"
                    }
                  }
                }
              }
            }
          }

          div { class: "flex flex-shrink-0 space-x-1.5 opacity-0 group-hover:opacity-100 focus-within:opacity-100 transition-opacity duration-150",
            if !editing() {
              button {
                r#type: "button",
                class: "p-1.5 rounded {button_text_class} hover:bg-gray-200 dark:hover:bg-gray-700 hover:text-blue-600 dark:hover:text-blue-400 transition-colors duration-150",
                title: "Edit task text",
                onclick: toggle_editing,
                svg {
                  xmlns: "http://www.w3.org/2000/svg",
                  fill: "none",
                  view_box: "0 0 24 24",
                  stroke_width: "1.5",
                  stroke: "currentColor",
                  class: "w-4 h-4",
                  path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10",
                  }
                }
              }
              button {
                r#type: "button",
                class: "p-1.5 rounded {button_text_class} hover:bg-gray-200 dark:hover:bg-gray-700 hover:text-green-600 dark:hover:text-green-400 transition-colors duration-150",
                title: "Edit due date",
                onclick: toggle_date_editing,
                svg {
                  xmlns: "http://www.w3.org/2000/svg",
                  fill: "none",
                  view_box: "0 0 24 24",
                  stroke_width: "1.5",
                  stroke: "currentColor",
                  class: "w-4 h-4",
                  path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5",
                  }
                }
              }
              button {
                r#type: "button",
                class: "p-1.5 rounded {button_text_class} hover:bg-gray-200 dark:hover:bg-gray-700 hover:text-purple-600 dark:hover:text-purple-400 transition-colors duration-150",
                title: "Edit tags",
                onclick: toggle_tag_editing,
                svg {
                  xmlns: "http://www.w3.org/2000/svg",
                  fill: "none",
                  view_box: "0 0 24 24",
                  stroke_width: "1.5",
                  stroke: "currentColor",
                  class: "w-4 h-4",
                  path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M9.568 3H5.25A2.25 2.25 0 003 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 005.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 009.568 3z",
                  }
                  path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M6 6h.008v.008H6V6z",
                  }
                }
              }
            }
            button {
              r#type: "button",
              class: "p-1.5 rounded {button_text_class} hover:bg-gray-200 dark:hover:bg-gray-700 hover:text-red-600 dark:hover:text-red-400 transition-colors duration-150",
              title: "Delete task",
              onclick: move |_| props.on_delete.call(todo_id),
              svg {
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                class: "w-4 h-4",
                path {
                  stroke_linecap: "round",
                  stroke_linejoin: "round",
                  d: "M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0",
                }
              }
            }
          }
        }

        if date_editing() {
          form {
            class: "mt-3 flex items-center space-x-2",
            onsubmit: handle_date_change,
            label { class: "text-xs font-medium {date_text_class}", "Due:" }
            input {
              r#type: "date",
              class: "px-2 py-1 border {border_class} {input_bg_class} rounded shadow-sm focus:outline-none focus:ring-1 focus:ring-blue-400 transition-colors text-xs w-36",
              value: todo_due_date.map(|dt| dt.format("%Y-%m-%d").to_string()).unwrap_or_default(),
              onchange: handle_date_change,
            }
            button {
              r#type: "button",
              class: "p-1 rounded {button_text_class} hover:bg-gray-200 dark:hover:bg-gray-700 hover:text-red-600 dark:hover:text-red-400 transition-colors",
              onclick: toggle_date_editing,
              svg {
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                class: "w-4 h-4",
                path {
                  stroke_linecap: "round",
                  stroke_linejoin: "round",
                  d: "M6 18L18 6M6 6l12 12",
                }
              }
            }
          }
        } else if let Some(date_str) = due_date_display {
          div { class: "mt-2 text-xs flex items-center {date_text_class} transition-colors duration-200",
            span { class: "{date_icon_class} mr-1.5",
              svg {
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                class: "w-3.5 h-3.5",
                path {
                  stroke_linecap: "round",
                  stroke_linejoin: "round",
                  d: "M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5",
                }
              }
            }
            span { "Due: {date_str}" }
          }
        }

        // Replace the existing tag section with the new one that only shows when editing
        if tag_editing() {
          div {
            class: "mt-3 flex flex-wrap items-center gap-1.5",
            tabindex: "0",
            onblur: move |_| {
                tag_editing.set(false);
            },

            // Show all tags when editing
            {
                todo_tags
                    .iter()
                    .map(|tag| {
                        let tag_clone = tag.clone();
                        let on_tag_remove = props.on_tag_remove;
                        rsx! {
                          span {
                            key: "tag-{tag_clone}",
                            class: "{tag_bg_class} {tag_text_class} text-xs px-2.5 py-0.5 rounded-full flex items-center transition-colors duration-200",
                            span { class: "mr-1", "{tag}" }
                            button {
                              class: "opacity-70 hover:opacity-100 focus:outline-none",
                              onclick: move |_| on_tag_remove.call((todo_id, tag_clone.clone())),
                              svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2.5",
                                stroke: "currentColor",
                                class: "w-3 h-3",
                                path {
                                  stroke_linecap: "round",
                                  stroke_linejoin: "round",
                                  d: "M6 18L18 6M6 6l12 12",
                                }
                              }
                            }
                          }
                        }
                    })
            }

            {
                default_tags_list
                    .iter()
                    .filter(|dt| !todo_tags.contains(*dt))
                    .map(|default_tag| {
                        let tag_to_add = default_tag.clone();
                        let add_default_tag_clone = add_default_tag;
                        rsx! {
                          button {
                            key: "default-tag-{tag_to_add}",
                            r#type: "button",
                            class: "{tag_suggestion_button_class}",
                            onclick: move |_| add_default_tag_clone(tag_to_add.clone()),
                            "+ {default_tag}"
                          }
                        }
                    })
            }

            form {
              class: "flex items-center",
              onsubmit: handle_tag_add,
              input {
                class: "text-xs px-2 py-1 border {border_class} {input_bg_class} rounded-l focus:outline-none focus:ring-1 focus:ring-blue-400 w-24 transition-colors duration-200",
                placeholder: "New tag...",
                value: "{new_tag.read()}",
                oninput: move |evt| new_tag.set(evt.value()),
              }
              button {
                r#type: "submit",
                class: "px-2 py-1 {add_tag_button_class} text-white text-xs rounded-r focus:outline-none focus:ring-1 focus:ring-green-400 transition-colors duration-200",
                "Add"
              }
            }
            button {
              r#type: "button",
              class: "p-1 rounded {button_text_class} hover:bg-gray-200 dark:hover:bg-gray-700 hover:text-red-600 dark:hover:text-red-400 transition-colors",
              onclick: toggle_tag_editing,
              svg {
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                class: "w-4 h-4",
                path {
                  stroke_linecap: "round",
                  stroke_linejoin: "round",
                  d: "M6 18L18 6M6 6l12 12",
                }
              }
            }
          }
        }
      }
    }
}
