use crate::models::FilterState;
use dioxus::prelude::*;

pub fn use_keyboard_shortcuts<F, T>(
    mut filter_setter: F,
    mut theme_toggler: T,
) -> impl FnMut(Event<KeyboardData>) + 'static
where
    F: FnMut(FilterState) + 'static,
    T: FnMut(()) + 'static,
{
    move |evt: Event<KeyboardData>| {
        if evt.modifiers().ctrl() {
            let key = evt.key().to_string();
            match key.as_str() {
                "a" => {
                    filter_setter(FilterState::All);
                    evt.prevent_default();
                }
                "c" => {
                    filter_setter(FilterState::Completed);
                    evt.prevent_default();
                }
                "v" => {
                    filter_setter(FilterState::Active);
                    evt.prevent_default();
                }
                "d" => {
                    theme_toggler(());
                    evt.prevent_default();
                }
                _ => {}
            }
        }
    }
}
