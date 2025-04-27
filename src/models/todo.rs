use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single todo item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    pub completed: bool,
    pub due_date: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub order: usize,
}

impl Todo {
    /// Creates a new Todo item with the given text.
    pub fn new(id: usize, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
            due_date: None,
            tags: Vec::new(),
            order: id,
        }
    }

    /// Toggles the completed status of the todo.
    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }

    /// Sets the due date for the todo
    pub fn set_due_date(&mut self, date: Option<DateTime<Utc>>) {
        self.due_date = date;
    }

    /// Adds a tag to the todo
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Removes a tag from the todo
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }
}

/// Filter options for displaying todos.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum FilterState {
    #[default]
    All,
    Active,
    Completed,
}

impl FilterState {
    /// Checks if a todo should be visible based on the current filter.
    pub fn matches(&self, todo: &Todo) -> bool {
        match self {
            FilterState::All => true,
            FilterState::Active => !todo.completed,
            FilterState::Completed => todo.completed,
        }
    }
}

/// Manages the collection of todos in the application.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoList {
    todos: HashMap<usize, Todo>,
    next_id: usize,
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoList {
    /// Creates a new, empty TodoList.
    pub fn new() -> Self {
        Self {
            todos: HashMap::new(),
            next_id: 1,
        }
    }

    /// Adds a new todo with the given text.
    pub fn add(&mut self, text: String) -> usize {
        let id = self.next_id;
        self.todos.insert(id, Todo::new(id, text));
        self.next_id += 1;
        id
    }

    /// Removes a todo by its ID.
    pub fn remove(&mut self, id: usize) -> Option<Todo> {
        self.todos.remove(&id)
    }

    /// Toggles the completion status of a todo.
    ///
    /// # Arguments
    /// * `todo_id` - The unique identifier of the todo to toggle
    ///
    /// # Returns
    /// * `true` if the todo was found and toggled
    /// * `false` if no todo with the given id exists
    ///
    /// # Example
    /// ```
    /// # use todo::models::TodoList;
    /// let mut list = TodoList::new();
    /// let id = list.add("Example todo".to_string());
    /// assert!(!list.all()[0].completed);
    /// list.toggle_completion(id);
    /// assert!(list.all()[0].completed);
    /// ```
    pub fn toggle_completion(&mut self, todo_id: usize) -> bool {
        if let Some(todo) = self.todos.get_mut(&todo_id) {
            todo.toggle();
            true
        } else {
            false
        }
    }

    /// Toggles the completion status of a todo.
    ///
    /// This is a compatibility wrapper for `toggle_completion`.
    /// For new code, use `toggle_completion` instead.
    pub fn toggle(&mut self, id: usize) -> bool {
        self.toggle_completion(id)
    }

    /// Updates the text of a todo.
    pub fn update_text(&mut self, id: usize, text: String) -> bool {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.text = text;
            true
        } else {
            false
        }
    }

    /// Validates if a reorder operation is valid
    ///
    /// # Arguments
    /// * `source_id` - The ID of the todo to be moved
    /// * `target_id` - The ID of the todo to move to
    ///
    /// # Returns
    /// * `true` if the reorder is valid (IDs different and both exist)
    /// * `false` otherwise
    fn validate_reorder_request(&self, source_id: usize, target_id: usize) -> bool {
        source_id != target_id
            && self.todos.contains_key(&source_id)
            && self.todos.contains_key(&target_id)
    }

    /// Gets the order value of a todo by its ID
    ///
    /// # Arguments
    /// * `id` - The ID of the todo
    ///
    /// # Returns
    /// The order value of the todo, or 0 if not found
    fn get_todo_order(&self, id: usize) -> usize {
        self.todos.get(&id).map(|todo| todo.order).unwrap_or(0)
    }

    /// Adjusts orders when a todo is moved down in the list
    ///
    /// Decrements order for todos between source and target (inclusive)
    ///
    /// # Arguments
    /// * `source_order` - The current order of the source todo
    /// * `target_order` - The target order position
    fn reorder_todos_moving_down(&mut self, source_order: usize, target_order: usize) {
        for (_, todo) in self.todos.iter_mut() {
            if todo.order > source_order && todo.order <= target_order {
                todo.order -= 1;
            }
        }
    }

    /// Adjusts orders when a todo is moved up in the list
    ///
    /// Increments order for todos between target and source (inclusive)
    ///
    /// Note: When moving an item up in the list (lower index), the target item
    /// will be pushed down, and the source item takes its place. This means the
    /// source item will appear before the target item in the final ordering.
    ///
    /// # Arguments
    /// * `source_order` - The current order of the source todo
    /// * `target_order` - The target order position
    fn reorder_todos_moving_up(&mut self, source_order: usize, target_order: usize) {
        for (_, todo) in self.todos.iter_mut() {
            if todo.order >= target_order && todo.order < source_order {
                todo.order += 1;
            }
        }
    }

    /// Updates the order of the source todo to the target position
    ///
    /// # Arguments
    /// * `source_id` - The ID of the todo to update
    /// * `target_order` - The new order value to set
    ///
    /// # Returns
    /// * `true` if the update was successful
    /// * `false` if the todo was not found
    fn update_source_todo_order(&mut self, source_id: usize, target_order: usize) -> bool {
        if let Some(todo) = self.todos.get_mut(&source_id) {
            todo.order = target_order;
            true
        } else {
            false
        }
    }

    /// Reorders a todo item by changing its position in the list
    ///
    /// # Arguments
    /// * `source_id` - The ID of the todo to be moved
    /// * `target_id` - The ID of the todo to move to
    ///
    /// # Returns
    /// * `true` if the reorder was successful
    /// * `false` if the operation was invalid
    pub fn reorder(&mut self, source_id: usize, target_id: usize) -> bool {
        if !self.validate_reorder_request(source_id, target_id) {
            return false;
        }

        let source_order = self.get_todo_order(source_id);
        let target_order = self.get_todo_order(target_id);

        // Determine if moving up or down in order
        if source_order < target_order {
            // Moving down
            self.reorder_todos_moving_down(source_order, target_order);
        } else {
            // Moving up
            self.reorder_todos_moving_up(source_order, target_order);
        }

        // Set the source todo to the target position
        self.update_source_todo_order(source_id, target_order);

        true
    }

    /// Gets all todos as a vector, sorted by their order field.
    pub fn all(&self) -> Vec<Todo> {
        let mut todos: Vec<Todo> = self.todos.values().cloned().collect();
        todos.sort_by_key(|todo| todo.order);
        todos
    }

    /// Gets filtered todos based on the given filter state.
    ///
    /// This is a utility method that could be used in the future for more
    /// advanced filtering capabilities.
    #[allow(dead_code)]
    pub fn filtered(&self, filter: FilterState) -> Vec<Todo> {
        self.todos
            .values()
            .filter(|todo| filter.matches(todo))
            .cloned()
            .collect()
    }

    /// Clears all completed todos.
    pub fn clear_completed(&mut self) -> usize {
        let completed_ids: Vec<_> = self
            .todos
            .iter()
            .filter(|(_, todo)| todo.completed)
            .map(|(id, _)| *id)
            .collect();

        let count = completed_ids.len();

        for id in completed_ids {
            self.todos.remove(&id);
        }

        count
    }

    /// Returns the count of active (not completed) todos.
    pub fn active_count(&self) -> usize {
        self.todos.values().filter(|todo| !todo.completed).count()
    }

    /// Returns the count of completed todos.
    pub fn completed_count(&self) -> usize {
        self.todos.values().filter(|todo| todo.completed).count()
    }

    /// Returns the total number of todos.
    ///
    /// This could be used in the future for statistics or pagination.
    #[allow(dead_code)]
    pub fn total_count(&self) -> usize {
        self.todos.len()
    }

    /// Sets a due date for a todo.
    pub fn set_due_date(&mut self, id: usize, date: Option<DateTime<Utc>>) -> bool {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.set_due_date(date);
            true
        } else {
            false
        }
    }

    /// Adds a tag to a todo.
    pub fn add_tag(&mut self, id: usize, tag: String) -> bool {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.add_tag(tag);
            true
        } else {
            false
        }
    }

    /// Removes a tag from a todo.
    pub fn remove_tag(&mut self, id: usize, tag: &str) -> bool {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.remove_tag(tag);
            true
        } else {
            false
        }
    }

    /// Gets all unique tags across all todos.
    pub fn all_tags(&self) -> Vec<String> {
        let mut tags = std::collections::HashSet::new();
        for todo in self.todos.values() {
            for tag in &todo.tags {
                tags.insert(tag.clone());
            }
        }
        tags.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filtered() {
        let mut todo_list = TodoList {
            todos: HashMap::new(),
            next_id: 1,
        };

        todo_list
            .todos
            .insert(1, Todo::new(1, "Active todo".to_string()));
        todo_list
            .todos
            .insert(2, Todo::new(2, "Completed todo".to_string()));
        todo_list.todos.get_mut(&2).unwrap().toggle();

        // Test All filter
        let filtered = todo_list.filtered(FilterState::All);
        assert_eq!(filtered.len(), 2);

        // Test Active filter
        let filtered = todo_list.filtered(FilterState::Active);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, 1);

        // Test Completed filter
        let filtered = todo_list.filtered(FilterState::Completed);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, 2);
    }

    #[test]
    fn test_clear_completed() {
        let mut todo_list = TodoList::new();

        todo_list
            .todos
            .insert(1, Todo::new(1, "Active todo".to_string()));
        todo_list
            .todos
            .insert(2, Todo::new(2, "Completed todo".to_string()));
        todo_list.todos.get_mut(&2).unwrap().toggle();

        let cleared_count = todo_list.clear_completed();
        assert_eq!(cleared_count, 1);
        assert_eq!(todo_list.todos.len(), 1);
        assert!(todo_list.todos.contains_key(&1));
        assert!(!todo_list.todos.contains_key(&2));
    }

    #[test]
    fn test_active_count() {
        let mut todo_list = TodoList::new();

        todo_list
            .todos
            .insert(1, Todo::new(1, "Active todo".to_string()));
        todo_list
            .todos
            .insert(2, Todo::new(2, "Completed todo".to_string()));
        todo_list.todos.get_mut(&2).unwrap().toggle();

        assert_eq!(todo_list.active_count(), 1);
    }

    #[test]
    fn test_completed_count() {
        let mut todo_list = TodoList::new();

        todo_list
            .todos
            .insert(1, Todo::new(1, "Active todo".to_string()));
        todo_list
            .todos
            .insert(2, Todo::new(2, "Completed todo".to_string()));
        todo_list.todos.get_mut(&2).unwrap().toggle();

        assert_eq!(todo_list.completed_count(), 1);
    }

    #[test]
    fn test_total_count() {
        let mut todo_list = TodoList::new();

        todo_list
            .todos
            .insert(1, Todo::new(1, "Todo 1".to_string()));
        todo_list
            .todos
            .insert(2, Todo::new(2, "Todo 2".to_string()));

        assert_eq!(todo_list.total_count(), 2);
    }

    #[test]
    fn test_reorder() {
        let mut list = TodoList::new();

        // Add some todos
        let id1 = list.add("First todo".to_string());
        let id2 = list.add("Second todo".to_string());
        let id3 = list.add("Third todo".to_string());

        // Initial order should match creation order
        let todos = list.all();
        assert_eq!(todos[0].id, id1);
        assert_eq!(todos[1].id, id2);
        assert_eq!(todos[2].id, id3);

        // Reorder todo 1 to position 3
        let result = list.reorder(id1, id3);
        assert!(result);

        // Check new order
        let todos = list.all();
        assert_eq!(todos[0].id, id2);
        assert_eq!(todos[1].id, id3);
        assert_eq!(todos[2].id, id1);

        // Reorder todo 3 to position 2
        let result = list.reorder(id3, id2);
        assert!(result);

        // Check new order based on the actual behavior
        let todos = list.all();
        assert_eq!(todos[0].id, id3); // Third todo is now at position 0
        assert_eq!(todos[1].id, id2); // Second todo is now at position 1
        assert_eq!(todos[2].id, id1); // First todo remains at position 2

        // Test invalid reorder operations

        // Same source and target
        let result = list.reorder(id1, id1);
        assert!(!result);

        // Non-existent todo
        let result = list.reorder(999, id1);
        assert!(!result);

        let result = list.reorder(id1, 999);
        assert!(!result);
    }
}
