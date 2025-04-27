//! Application-wide constants
//!
//! This module contains all constants used throughout the application,
//! grouped by their usage context to maintain organization.

/// Storage-related constants
pub mod storage {
    /// Key used for storing todo data in local storage or database
    pub const TODO_STORAGE_KEY: &str = "dioxus-todo-app";
}

/// UI-related constants
#[allow(dead_code)]
pub mod ui {
    /// Default window dimensions
    pub mod window {
        /// Default window width (pixels)
        pub const DEFAULT_WIDTH: f64 = 800.0;

        /// Default window height (pixels)
        pub const DEFAULT_HEIGHT: f64 = 1200.0;
    }

    /// CSS class definitions for consistent theming
    pub mod css {
        /// Background classes with dark mode support
        pub const BG_DARK_CLASS: &str = "bg-gray-100 dark:bg-gray-900";

        /// Background classes for light mode
        pub const BG_LIGHT_CLASS: &str = "";

        /// Button classes for primary actions
        pub const PRIMARY_BUTTON_CLASS: &str =
            "rounded px-2 py-1 text-white bg-blue-500 hover:bg-blue-600";

        /// Button classes for secondary actions
        pub const SECONDARY_BUTTON_CLASS: &str = "rounded px-2 py-1 text-gray-700 bg-gray-200 hover:bg-gray-300 dark:text-white dark:bg-gray-600 dark:hover:bg-gray-700";
    }
}

/// Todo-related constants
#[allow(dead_code)]
pub mod todo {
    /// Default tag options for todos
    pub const DEFAULT_TAGS: [&str; 4] = ["Work", "Personal", "Urgent", "Shopping"];

    /// Maximum length for todo text
    pub const MAX_TODO_TEXT_LENGTH: usize = 280;

    /// Maximum number of tags per todo
    pub const MAX_TAGS_PER_TODO: usize = 5;
}

/// Application-wide constants
#[allow(dead_code)]
pub mod app {
    /// Application name
    pub const APP_NAME: &str = "Dioxus Todo App";

    /// Application version
    pub const APP_VERSION: &str = "0.1.0";
}
