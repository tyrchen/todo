use dioxus::prelude::*;

/// Component for filtering todos by tags
#[component]
pub fn TagsFilter(
    tags: Vec<String>,
    selected_tag: Option<String>,
    on_select_tag: EventHandler<Option<String>>,
    is_dark_mode: bool,
) -> Element {
    let text_secondary_class = "text-gray-600 dark:text-gray-400";
    let border_class = "border-gray-200 dark:border-gray-700";

    // Only render if there are tags
    if tags.is_empty() {
        return rsx! {
            div {}
        };
    }

    rsx! {
        div { class: "mt-6 mb-4 flex flex-wrap items-center {border_class} border-b pb-4",
            span { class: "mr-2 text-sm font-medium {text_secondary_class} transition-colors",
                "Filter by tag:"
            }

            // "All" tag option
            {
                let all_base_class = "text-xs px-3 py-1 rounded-full mr-1.5 mb-1.5 hover:opacity-80 transition-all border";
                let (all_bg_text, all_border) = if is_dark_mode {
                    ("bg-gray-700 text-gray-200", "border-gray-600")
                } else {
                    ("bg-gray-200 text-gray-700", "border-gray-300")
                };
                let all_selected_class = if selected_tag.is_none() {
                    " ring-2 ring-blue-500 ring-offset-1 dark:ring-offset-gray-900"
                } else {
                    ""
                };
                let final_all_class = format!(
                    "{} {} {} {}",
                    all_base_class,
                    all_bg_text,
                    all_border,
                    all_selected_class,
                );
                rsx! {
                    button { class: "{final_all_class}", onclick: move |_| on_select_tag.call(None), "All" }
                }
            }

            // Render tag buttons
            {
                tags.iter()
                    .map(|tag| {
                        let tag_clone = tag.clone();
                        let is_selected = selected_tag.as_ref() == Some(tag);
                        let base_tag_class = "text-xs px-3 py-1 rounded-full mr-1.5 mb-1.5 hover:opacity-80 transition-opacity border";
                        let (tag_bg_text, tag_border) = if is_dark_mode {
                            ("bg-blue-900 text-blue-200", "border-blue-700")
                        } else {
                            ("bg-blue-100 text-blue-800", "border-blue-300")
                        };
                        let selected_class = if is_selected {
                            " ring-2 ring-blue-500 ring-offset-1 dark:ring-offset-gray-900"
                        } else {
                            ""
                        };
                        let final_tag_class = format!(
                            "{} {} {} {}",
                            base_tag_class,
                            tag_bg_text,
                            tag_border,
                            selected_class,
                        );
                        rsx! {
                            button {
                                key: "{tag_clone}", // Use the tag itself as key
                                class: "{final_tag_class}",
                                onclick: move |_| {
                                    if is_selected {
                                        on_select_tag.call(None);
                                    } else {
                                        on_select_tag.call(Some(tag_clone.clone()));
                                    }
                                },
                                "{tag}"
                            }
                        }
                    })
            }
        }
    }
}
