//! Theme utilities for consistent UI styling
//!
//! This module provides utility functions for generating CSS classes
//! with support for dark mode and other theme variations.

use crate::utils::constants::ui::css::*;

/// Get the appropriate CSS class for a container element based on dark mode
///
/// # Arguments
/// * `is_dark_mode` - Whether dark mode is enabled
///
/// # Returns
/// CSS classes for the container
pub fn container_class(is_dark_mode: bool) -> String {
    if is_dark_mode {
        format!("{} transition-colors duration-300", BG_DARK_CLASS)
    } else {
        format!("{} transition-colors duration-300", BG_LIGHT_CLASS)
    }
}

/// Get the appropriate CSS class for a primary button
///
/// # Arguments
/// * `is_dark_mode` - Whether dark mode is enabled
/// * `disabled` - Whether the button is disabled
///
/// # Returns
/// CSS classes for the primary button
#[allow(dead_code)]
pub fn primary_button_class(_is_dark_mode: bool, disabled: bool) -> String {
    if disabled {
        format!("{} opacity-50 cursor-not-allowed", PRIMARY_BUTTON_CLASS)
    } else {
        PRIMARY_BUTTON_CLASS.to_string()
    }
}

/// Get the appropriate CSS class for a secondary button
///
/// # Arguments
/// * `is_dark_mode` - Whether dark mode is enabled
/// * `disabled` - Whether the button is disabled
///
/// # Returns
/// CSS classes for the secondary button
#[allow(dead_code)]
pub fn secondary_button_class(_is_dark_mode: bool, disabled: bool) -> String {
    if disabled {
        format!("{} opacity-50 cursor-not-allowed", SECONDARY_BUTTON_CLASS)
    } else {
        SECONDARY_BUTTON_CLASS.to_string()
    }
}

/// Get the appropriate CSS class for a card element
///
/// # Arguments
/// * `is_dark_mode` - Whether dark mode is enabled
///
/// # Returns
/// CSS classes for the card
#[allow(dead_code)]
pub fn card_class(is_dark_mode: bool) -> String {
    if is_dark_mode {
        "bg-gray-800 border-gray-700 shadow-md rounded-lg p-4".to_string()
    } else {
        "bg-white border border-gray-200 shadow-md rounded-lg p-4".to_string()
    }
}

/// Get the appropriate CSS class for an input element
///
/// # Arguments
/// * `is_dark_mode` - Whether dark mode is enabled
///
/// # Returns
/// CSS classes for the input
#[allow(dead_code)]
pub fn input_class(is_dark_mode: bool) -> String {
    if is_dark_mode {
        "bg-gray-700 border-gray-600 text-white rounded p-2 w-full".to_string()
    } else {
        "bg-white border border-gray-300 text-gray-900 rounded p-2 w-full".to_string()
    }
}
