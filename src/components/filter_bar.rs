use crate::models::FilterState;
use dioxus::prelude::*;

/// Props for the FilterBar component.
#[derive(Props, PartialEq, Clone)]
pub struct FilterBarProps {
    /// The current filter state
    pub filter: FilterState,
    /// Callback when the filter is changed
    pub on_filter_change: EventHandler<FilterState>,
    /// The number of active (not completed) todos
    pub active_count: usize,
    /// The number of completed todos
    pub completed_count: usize,
    /// Callback when clear completed is clicked
    pub on_clear_completed: EventHandler<()>,
    /// Whether dark mode is enabled
    #[props(default = false)]
    pub is_dark_mode: bool,
}

/// Component for filtering todos and showing counts.
#[component]
pub fn FilterBar(props: FilterBarProps) -> Element {
    // Dynamic classes based on dark mode
    let container_bg_class = if props.is_dark_mode {
        "bg-gray-800"
    } else {
        "bg-white"
    };
    let text_class = if props.is_dark_mode {
        "text-gray-400"
    } else {
        "text-gray-600"
    };
    let clear_btn_class = if props.is_dark_mode {
        "text-gray-400 hover:text-red-400"
    } else {
        "text-gray-500 hover:text-red-500"
    };

    let filter_button = move |filter: FilterState, label: &'static str| {
        let is_active = props.filter == filter;
        let active_btn_class = if props.is_dark_mode {
            "px-3 py-1 rounded bg-blue-600 text-white"
        } else {
            "px-3 py-1 rounded bg-blue-500 text-white"
        };

        let inactive_btn_class = if props.is_dark_mode {
            "px-3 py-1 rounded bg-gray-700 text-gray-300 hover:bg-gray-600"
        } else {
            "px-3 py-1 rounded bg-gray-100 text-gray-600 hover:bg-gray-200"
        };

        rsx! {
          button {
            r#type: "button",
            class: if is_active { active_btn_class } else { inactive_btn_class },
            onclick: move |_| props.on_filter_change.call(filter),
            "{label}"
          }
        }
    };

    rsx! {
      div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between p-4 {container_bg_class} rounded-lg shadow mt-4 transition-colors duration-300",

        // Item count
        div { class: "mb-2 sm:mb-0 {text_class} transition-colors duration-300",
          if props.active_count == 1 {
            "{props.active_count} item left"
          } else {
            "{props.active_count} items left"
          }
        }

        // Filter buttons
        div { class: "flex space-x-2 mb-2 sm:mb-0",
          {filter_button(FilterState::All, "All")}
          {filter_button(FilterState::Active, "Active")}
          {filter_button(FilterState::Completed, "Completed")}
        }

        // Clear completed button (only shown if there are completed todos)
        if props.completed_count > 0 {
          button {
            r#type: "button",
            class: "{clear_btn_class} transition-colors duration-300",
            onclick: move |_| props.on_clear_completed.call(()),
            "Clear completed ({props.completed_count})"
          }
        }
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::dioxus_core::Mutations;

    #[test]
    fn test_filter_button_rendering() {
        let mut app = VirtualDom::new(|| {
            rsx! {
              FilterBar {
                filter: FilterState::All,
                active_count: 5,
                completed_count: 3,
                on_filter_change: move |_| {},
                on_clear_completed: move |_| {},
              }
            }
        });

        app.rebuild(&mut Mutations::default());
        // Note: In a real test environment, you would want to verify the rendered output
        // This is a basic structure that can be expanded with more detailed assertions
    }

    #[test]
    fn test_filter_button_active_state() {
        let mut app = VirtualDom::new(|| {
            rsx! {
              FilterBar {
                filter: FilterState::Active,
                active_count: 2,
                completed_count: 1,
                on_filter_change: move |_| {},
                on_clear_completed: move |_| {},
              }
            }
        });

        app.rebuild(&mut Mutations::default());
        // Note: In a real test environment, you would want to verify the active state styling
    }

    #[test]
    fn test_clear_completed_visibility() {
        let mut app = VirtualDom::new(|| {
            rsx! {
              FilterBar {
                filter: FilterState::All,
                active_count: 2,
                completed_count: 0,
                on_filter_change: move |_| {},
                on_clear_completed: move |_| {},
              }
            }
        });

        app.rebuild(&mut Mutations::default());
        // Note: In a real test environment, you would want to verify that the clear completed button is not visible
    }
}
