# Constants File Template

This document provides a template for the new `constants.rs` file that should be created in the utils module to centralize all constants used throughout the application.

```rust
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
        pub const BG_CLASS: &str = "bg-gray-100 dark:bg-gray-900";

        /// Button classes for primary actions
        pub const PRIMARY_BUTTON_CLASS: &str = "rounded px-2 py-1 text-white bg-blue-500 hover:bg-blue-600";

        /// Button classes for secondary actions
        pub const SECONDARY_BUTTON_CLASS: &str = "rounded px-2 py-1 text-gray-700 bg-gray-200 hover:bg-gray-300 dark:text-white dark:bg-gray-600 dark:hover:bg-gray-700";
    }
}

/// Todo-related constants
pub mod todo {
    /// Default tag options for todos
    pub const DEFAULT_TAGS: [&str; 4] = ["Work", "Personal", "Urgent", "Shopping"];

    /// Maximum length for todo text
    pub const MAX_TODO_TEXT_LENGTH: usize = 280;

    /// Maximum number of tags per todo
    pub const MAX_TAGS_PER_TODO: usize = 5;
}

/// Application-wide constants
pub mod app {
    /// Application name
    pub const APP_NAME: &str = "Dioxus Todo App";

    /// Application version
    pub const APP_VERSION: &str = "0.1.0";
}
```

## Usage Examples

### In storage.rs
```rust
use crate::utils::constants::storage::TODO_STORAGE_KEY;

// Instead of:
// const STORAGE_KEY: &str = "dioxus-todo-app";

// Use:
pub fn load<T: DeserializeOwned>(key: &str) -> Result<T, StorageError> {
    // ...
}

// Example usage:
let todos = load::<TodoList>(TODO_STORAGE_KEY)?;
```

### In main.rs
```rust
use crate::utils::constants::ui::window::{DEFAULT_WIDTH, DEFAULT_HEIGHT};
use crate::utils::constants::app::APP_NAME;

// Instead of:
// .with_inner_size(dioxus_desktop::LogicalSize::new(800.0, 1200.0))
// .with_title("Dioxus Todo App")

// Use:
.with_inner_size(dioxus_desktop::LogicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT))
.with_title(APP_NAME)
```

### In todo_app.rs
```rust
use crate::utils::constants::todo::DEFAULT_TAGS;
use crate::utils::constants::ui::css::BG_CLASS;

// Instead of:
// const DEFAULT_TAGS: [&str; 4] = ["Work", "Personal", "Urgent", "Shopping"];
// let app_bg_class = "bg-gray-100 dark:bg-gray-900";

// Use:
let (todo_list, mut filter, mut selected_tag, operations, sorted_tags) =
    use_todo_state(&DEFAULT_TAGS);

// And:
div {
    class: "h-full {BG_CLASS} py-8 px-4 transition-colors duration-300",
    // ...
}
```
