use crate::utils;
use dioxus::prelude::*;

const THEME_STORAGE_KEY: &str = "dioxus-todo-app-theme";

#[cfg(target_arch = "wasm32")]
use web_sys::window;

/// Logic for managing theme state and operations
pub fn use_theme_manager() -> (Signal<bool>, impl FnMut(()) + Clone) {
    let mut is_dark_mode = use_signal(|| {
        // Try to load from localStorage first
        if let Ok(theme) = utils::load::<String>(THEME_STORAGE_KEY) {
            return theme == "dark";
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            false // Default to light mode on non-wasm targets
        }

        #[cfg(target_arch = "wasm32")]
        // Otherwise detect from system preference using web-sys
        window()
            .and_then(|win| win.match_media("(prefers-color-scheme: dark)").ok())
            .flatten() // Flatten Option<Result<Option<MediaQueryList>, JsValue>>
            .map_or(false, |mql| mql.matches())
    });

    // Save theme preference whenever it changes and update HTML class
    use_effect(move || {
        let theme = if is_dark_mode() { "dark" } else { "light" };
        let _ = utils::save(THEME_STORAGE_KEY, &theme);

        #[cfg(target_arch = "wasm32")]
        // Also update the html class for Tailwind dark mode selector
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(html_element) = document.document_element() {
                    if is_dark_mode() {
                        let _ = html_element.class_list().add_1("dark");
                    } else {
                        let _ = html_element.class_list().remove_1("dark");
                    }
                }
            }
        }
    });

    let toggle_theme = move |_| {
        is_dark_mode.set(!is_dark_mode());
    };

    (is_dark_mode, toggle_theme)
}
