use todo::models::{FilterState, Todo, TodoList};

#[test]
fn test_todo_creation() {
    let todo = Todo::new(1, "Test todo".to_string());
    assert_eq!(todo.id, 1);
    assert_eq!(todo.text, "Test todo");
    assert!(!todo.completed);
}

#[test]
fn test_todo_toggle() {
    let mut todo = Todo::new(1, "Test todo".to_string());
    assert!(!todo.completed);

    todo.toggle();
    assert!(todo.completed);

    todo.toggle();
    assert!(!todo.completed);
}

#[test]
fn test_filter_state_matches() {
    let active_todo = Todo::new(1, "Active todo".to_string());
    let mut completed_todo = Todo::new(2, "Completed todo".to_string());
    completed_todo.toggle();

    assert!(FilterState::All.matches(&active_todo));
    assert!(FilterState::All.matches(&completed_todo));

    assert!(FilterState::Active.matches(&active_todo));
    assert!(!FilterState::Active.matches(&completed_todo));

    assert!(!FilterState::Completed.matches(&active_todo));
    assert!(FilterState::Completed.matches(&completed_todo));
}

#[test]
fn test_todo_list_operations() {
    let mut list = TodoList::new();

    // Add todos
    let id1 = list.add("First todo".to_string());
    let id2 = list.add("Second todo".to_string());

    // Verify counts
    assert_eq!(list.active_count(), 2);
    assert_eq!(list.completed_count(), 0);

    // Toggle completion
    assert!(list.toggle(id1));
    assert_eq!(list.active_count(), 1);
    assert_eq!(list.completed_count(), 1);

    // Update text
    assert!(list.update_text(id2, "Updated second todo".to_string()));
    let todos = list.all();
    assert!(todos.iter().any(|t| t.text == "Updated second todo"));

    // Remove todo
    assert!(list.remove(id1).is_some());
    assert_eq!(list.active_count(), 1);
    assert_eq!(list.completed_count(), 0);

    // Clear completed (none should be completed now)
    let cleared = list.clear_completed();
    assert_eq!(cleared, 0);
}

#[test]
fn test_todo_list_filtering() {
    let mut list = TodoList::new();

    // Add todos
    let id1 = list.add("First todo".to_string());
    let id2 = list.add("Second todo".to_string());
    let id3 = list.add("Third todo".to_string());

    // Mark some as completed
    list.toggle(id1);
    list.toggle(id3);

    // Test filtered view
    let all = list.all();
    assert_eq!(all.len(), 3);

    let filtered = list.filtered(FilterState::Active);
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].id, id2);

    let completed = list.filtered(FilterState::Completed);
    assert_eq!(completed.len(), 2);
    assert!(completed.iter().any(|t| t.id == id1));
    assert!(completed.iter().any(|t| t.id == id3));

    // Clear completed
    let cleared = list.clear_completed();
    assert_eq!(cleared, 2);
    assert_eq!(list.all().len(), 1);
}
