use dioxus::prelude::*;

/// Component for displaying the app header with title and theme toggle
#[component]
pub fn AppHeader(
    #[props(into)] title: String,
    is_dark_mode: bool,
    on_toggle_theme: EventHandler<()>,
) -> Element {
    let text_class = "text-gray-800 dark:text-gray-200";
    let text_secondary_class = "text-gray-600 dark:text-gray-400";

    rsx! {
        div {
            class: "flex justify-between items-center mb-8",
            h1 {
                class: "text-2xl sm:text-3xl font-bold {text_class} transition-colors",
                "{title}"
            }
            // Dark mode toggle
            button {
                class: "p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors {text_secondary_class}",
                onclick: move |_| on_toggle_theme.call(()),
                aria_label: "Toggle dark mode",
                if is_dark_mode { "🌞" } else { "🌙" }
            }
        }
    }
}
