use dioxus::prelude::*;

/// Component for displaying keyboard shortcuts help
#[component]
pub fn KeyboardShortcuts(is_dark_mode: bool) -> Element {
    let text_secondary_class = "text-gray-600 dark:text-gray-400";

    rsx! {
        div {
            class: "mt-6 text-xs {text_secondary_class} text-center transition-colors",
            p { "Keyboard shortcuts:" }
            p { "Ctrl+A: All todos | Ctrl+C: Completed todos | Ctrl+V: Active todos | Ctrl+D: Toggle dark mode" }
        }
    }
}
