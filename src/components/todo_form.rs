use dioxus::prelude::*;

/// Props for the TodoForm component.
#[derive(Props, PartialEq, Clone)]
pub struct TodoFormProps {
    /// Callback when a new todo is submitted
    pub on_add: EventHandler<String>,
    /// Whether dark mode is enabled
    #[props(default = false)]
    pub is_dark_mode: bool,
}

/// Form component for adding new todos.
#[component]
pub fn TodoForm(props: TodoFormProps) -> Element {
    let mut input_text = use_signal(String::new);

    let handle_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        let text = input_text.read().trim().to_string();
        if !text.is_empty() {
            props.on_add.call(text);
            *input_text.write() = String::new();
        }
    };

    // Dynamic classes based on dark mode
    let form_bg_class = if props.is_dark_mode {
        "bg-gray-800"
    } else {
        "bg-white"
    };
    let input_border_class = if props.is_dark_mode {
        "border-gray-700"
    } else {
        "border-gray-300"
    };
    let input_bg_class = if props.is_dark_mode {
        "bg-gray-700 text-gray-200"
    } else {
        "bg-white text-gray-900"
    };
    let button_bg_class = if props.is_dark_mode {
        "bg-blue-600 hover:bg-blue-700"
    } else {
        "bg-blue-500 hover:bg-blue-600"
    };

    rsx! {
      form {
        class: "flex items-center p-4 {form_bg_class} rounded-lg shadow mb-6 transition-colors duration-300",
        onsubmit: handle_submit,

        input {
          class: "flex-1 px-4 py-2 border {input_border_class} {input_bg_class} rounded-l-lg focus:outline-none focus:ring-2 focus:ring-blue-300 transition-colors duration-300",
          r#type: "text",
          placeholder: "What needs to be done?",
          value: "{input_text.read()}",
          oninput: move |evt| *input_text.write() = evt.value().clone(),
          autofocus: true,
        }

        button {
          class: "px-4 py-2 {button_bg_class} text-white rounded-r-lg focus:outline-none focus:ring-2 focus:ring-blue-300 transition-colors duration-300",
          r#type: "submit",
          "Add Todo"
        }
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::dioxus_core::Mutations;

    #[test]
    fn test_todo_form_rendering() {
        let mut app = VirtualDom::new(|| {
            rsx! {
              TodoForm { on_add: move |_| {} }
            }
        });

        app.rebuild(&mut Mutations::default());
        // Note: In a real test environment, you would want to verify the rendered output
        // This is a basic structure that can be expanded with more detailed assertions
    }

    #[test]
    fn test_todo_form_empty_input() {
        let mut app = VirtualDom::new(|| {
            rsx! {
              TodoForm { on_add: move |_| {} }
            }
        });

        app.rebuild(&mut Mutations::default());
        // Note: In a real test environment, you would want to verify that the input is initially empty
        // and that submitting an empty form doesn't trigger the on_add callback
    }

    #[test]
    fn test_todo_form_input_handling() {
        let mut app = VirtualDom::new(|| {
            rsx! {
              TodoForm { on_add: move |_| {} }
            }
        });

        app.rebuild(&mut Mutations::default());
        // Note: In a real test environment, you would want to simulate input events
        // and verify that the input state is updated correctly
    }
}
