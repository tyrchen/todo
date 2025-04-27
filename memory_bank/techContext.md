# Todo App Technical Context

## Framework: Dioxus 0.6

### Core Concepts
- Component-based architecture similar to React
- Uses the RSX macro for declarative UI
- Supports web, desktop, and mobile platforms
- Reactive state management through hooks
- Router for navigation between views

### Key Components
- `#[component]` attribute for defining components
- `rsx!` macro for creating UI elements
- `use_state` and other hooks for state management
- `Link` and `Route` for navigation
- Event handlers for user interactions

## Styling: Tailwind CSS

### Implementation
- Integrated via npx tailwindcss CLI
- CSS file generated from input.css
- Utility-first approach to styling
- Responsive design using Tailwind breakpoints
- Custom configuration in tailwind.config.js

## Build System

### Dioxus CLI (dx)
- Used for development and building
- Supports multiple platforms via --platform flag
- Manages asset bundling
- Handles hot reloading

### Cargo
- Rust package manager
- Manages Rust dependencies
- Configured via Cargo.toml

## Project Structure

### Current Setup
- src/main.rs - Entry point with basic routing
- assets/ - Static assets including CSS
- Tailwind CSS integration
- Basic component structure

## Data Management

### Planned Approach
- Component-level state for UI interactions
- App-level state for todo items
- Local storage for persistence
- Serialization/deserialization of todo data

## Development Environment

### Requirements
- Rust and Cargo installed
- Node.js and npm for Tailwind CSS
- Dioxus CLI (dx) for development server
- Knowledge of Rust and Dioxus components
