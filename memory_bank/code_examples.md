# Code Examples for Refactoring

This document contains specific code examples from the codebase that need refactoring, organized by the categories identified in the refactoring plan.

## 1. Constants and Magic Numbers

### Example 1: Default Tags in todo_app.rs
```rust
// Current implementation
const DEFAULT_TAGS: [&str; 4] = ["Work", "Personal", "Urgent", "Shopping"];

// Should be moved to constants.rs
```

### Example 2: Storage Key in todo_state.rs
```rust
// Current implementation
const STORAGE_KEY: &str = "dioxus-todo-app";

// Should be moved to constants.rs
```

### Example 3: Window dimensions in main.rs
```rust
// Current implementation
.with_inner_size(dioxus_desktop::LogicalSize::new(800.0, 1200.0))

// Should be replaced with constants
```

## 2. Function Responsibility and Length

### Example 1: Complex reorder method in todo_list.rs
```rust
// Current implementation
pub fn reorder(&mut self, source_id: usize, target_id: usize) -> bool {
    if source_id == target_id
        || !self.todos.contains_key(&source_id)
        || !self.todos.contains_key(&target_id)
    {
        return false;
    }

    let source_order = self
        .todos
        .get(&source_id)
        .map(|todo| todo.order)
        .unwrap_or(0);
    let target_order = self
        .todos
        .get(&target_id)
        .map(|todo| todo.order)
        .unwrap_or(0);

    // Determine if moving up or down in order
    if source_order < target_order {
        // Moving down: all todos between source and target (inclusive) move up one position
        for (_, todo) in self.todos.iter_mut() {
            if todo.order > source_order && todo.order <= target_order {
                todo.order -= 1;
            }
        }
    } else {
        // Moving up: all todos between target and source (inclusive) move down one position
        for (_, todo) in self.todos.iter_mut() {
            if todo.order >= target_order && todo.order < source_order {
                todo.order += 1;
            }
        }
    }

    // Set the source todo to the target position
    if let Some(todo) = self.todos.get_mut(&source_id) {
        todo.order = target_order;
    }

    true
}

// Should be refactored into smaller functions:
// 1. validate_reorder_request()
// 2. get_todo_orders()
// 3. reorder_todos_moving_down()
// 4. reorder_todos_moving_up()
// 5. update_source_todo_order()
```

### Example 2: Large TodoApp component in todo_app.rs
```rust
// Current implementation (excerpt)
#[component]
pub fn TodoApp() -> Element {
    // Theme management
    let (is_dark_mode, toggle_theme) = use_theme_manager();

    // Todo state management
    let (todo_list, mut filter, mut selected_tag, operations, sorted_tags) =
        use_todo_state(&DEFAULT_TAGS);

    // Search state
    let mut search_text = use_signal(String::new);

    // ... more state setup ...
    // ... event handlers ...
    // ... rendering logic ...
}

// Should be broken down into smaller components:
// 1. TodoAppState - handling state management
// 2. TodoAppUI - handling rendering
// 3. TodoAppEventHandlers - handling events
```

## 3. Naming Improvements

### Example 1: Non-descriptive variable names
```rust
// Current implementation
for (_, t) in self.todos.iter_mut() {
    if t.completed {
        completed_count += 1;
    }
}

// Should be improved:
for (_, todo) in self.todos.iter_mut() {
    if todo.completed {
        completed_count += 1;
    }
}
```

### Example 2: Function parameter names
```rust
// Current implementation
fn toggle(&mut self, id: usize) -> bool {
    // ...
}

// Should be improved:
fn toggle_todo_completion(&mut self, todo_id: usize) -> bool {
    // ...
}
```

## 4. Error Handling Improvements

### Example 1: Inconsistent error handling in storage.rs
```rust
// Current implementation
pub fn load<T: DeserializeOwned>(key: &str) -> Result<T, StorageError> {
    let storage = get_storage()?;
    storage.load(key)
}

// Should provide better context:
pub fn load<T: DeserializeOwned>(key: &str) -> Result<T, StorageError> {
    let storage = get_storage().map_err(|err| {
        // Log error with context
        err
    })?;

    storage.load(key).map_err(|err| {
        // Add context to the error
        match err {
            StorageError::NotFound =>
                log::debug!("No data found for key: {}", key),
            _ => log::error!("Failed to load data for key {}: {:?}", key, err),
        }
        err
    })
}
```

## 5. Code Duplication

### Example 1: CSS class strings
```rust
// Current implementation (repeated in multiple components)
let app_bg_class = "bg-gray-100 dark:bg-gray-900";
let button_class = "rounded px-2 py-1 text-white bg-blue-500 hover:bg-blue-600";

// Should be extracted to a theme utils module
```

### Example 2: Event handler duplication
```rust
// Current implementation (similar patterns in different components)
let change_filter = move |new_filter: FilterState| {
    filter.set(new_filter);
};

let select_tag = move |tag: Option<String>| {
    selected_tag.set(tag);
};

// Should be generalized into a reusable signal setter
```

## 6. Component Architecture

### Example: Overly complex TodoApp component
```rust
// Current implementation handles both state and UI
#[component]
pub fn TodoApp() -> Element {
    // State management logic
    // ...

    // UI rendering
    rsx! {
        // ...
    }
}

// Should be split into separate concerns:
// 1. A state provider component
// 2. UI components that consume the state
```

## 7. Documentation Improvements

### Example: Incomplete function documentation
```rust
// Current implementation
/// Toggles the completed status of a todo.
pub fn toggle(&mut self, id: usize) -> bool {
    // ...
}

// Should be improved:
/// Toggles the completed status of a todo.
///
/// # Arguments
/// * `id` - The unique identifier of the todo to toggle
///
/// # Returns
/// * `true` if the todo was found and toggled
/// * `false` if no todo with the given id exists
///
/// # Example
/// ```
/// let mut list = TodoList::new();
/// let id = list.add("Example todo".to_string());
/// assert!(!list.all()[0].completed);
/// list.toggle(id);
/// assert!(list.all()[0].completed);
/// ```
pub fn toggle_todo_completion(&mut self, todo_id: usize) -> bool {
    // ...
}
```

## 8. Test Improvements

### Example: Incomplete test assertions
```rust
// Current implementation
#[test]
fn test_todo_app_rendering() {
    let mut app = VirtualDom::new(|| {
        rsx! {
            TodoApp {}
        }
    });

    app.rebuild(&mut Mutations::default());
    // No assertions
}

// Should be improved with actual assertions:
#[test]
fn test_todo_app_rendering() {
    let mut app = VirtualDom::new(|| {
        rsx! {
            TodoApp {}
        }
    });

    app.rebuild(&mut Mutations::default());

    // Add assertions to verify:
    // 1. Initial todo list is empty
    // 2. Default filter is "All"
    // 3. Form is rendered
    // 4. Child components are present
}
```
