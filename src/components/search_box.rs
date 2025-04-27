use dioxus::prelude::*;

/// Props for the SearchBox component
#[derive(Props, PartialEq, Clone)]
pub struct SearchBoxProps {
    /// Callback for when the search term changes
    pub on_search: EventHandler<String>,
    /// Current search term
    pub search_term: String,
    /// Whether dark mode is enabled
    #[props(default = false)]
    pub is_dark_mode: bool,
}

/// A component that renders a search input field
#[component]
pub fn SearchBox(props: SearchBoxProps) -> Element {
    // Dynamic classes based on dark mode
    let container_bg_class = if props.is_dark_mode {
        "bg-gray-800"
    } else {
        "bg-white"
    };
    let text_class = if props.is_dark_mode {
        "text-gray-100"
    } else {
        "text-gray-800"
    };
    let border_class = if props.is_dark_mode {
        "border-gray-700"
    } else {
        "border-gray-200"
    };
    let placeholder_class = if props.is_dark_mode {
        "placeholder-gray-500"
    } else {
        "placeholder-gray-400"
    };
    let focus_class = if props.is_dark_mode {
        "focus:ring-indigo-500 focus:border-indigo-500"
    } else {
        "focus:ring-indigo-600 focus:border-indigo-600"
    };
    let icon_class = if props.is_dark_mode {
        "text-gray-400"
    } else {
        "text-gray-500"
    };

    rsx! {
        div { class: "mb-4 {container_bg_class} rounded-lg shadow-md overflow-hidden transition-colors duration-300 border {border_class}",
            div { class: "relative flex items-center",
                // Search icon
                div { class: "absolute inset-y-0 left-3 flex items-center pointer-events-none {icon_class}",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-5 w-5",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        stroke: "currentColor",

                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                        }
                    }
                }

                // Search input
                input {
                    class: "block w-full py-3 pr-3 pl-10 {text_class} {placeholder_class} {container_bg_class} {focus_class} transition-colors duration-300 border-0 focus:ring-2 outline-none",
                    "type": "search",
                    placeholder: "Search todos...",
                    autocomplete: "off",
                    value: "{props.search_term}",
                    oninput: move |evt| props.on_search.call(evt.value()),
                    aria_label: "Search todos"
                }

                // Clear button (only shown when there is search text)
                if !props.search_term.is_empty() {
                    button {
                        class: "absolute right-3 {icon_class} hover:text-gray-700 dark:hover:text-gray-300 transition-colors duration-200",
                        r#type: "button",
                        title: "Clear search",
                        onclick: move |_| props.on_search.call(String::new()),
                        aria_label: "Clear search",

                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-5 w-5",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            stroke: "currentColor",

                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                "stroke-width": "2",
                                d: "M6 18L18 6M6 6l12 12"
                            }
                        }
                    }
                }
            }
        }
    }
}
