use dioxus::prelude::*;

mod components;
mod models;
mod utils;

use components::TodoApp;
use dioxus_logger::tracing::Level;
use utils::constants::app::APP_NAME;
use utils::constants::ui::window::{DEFAULT_HEIGHT, DEFAULT_WIDTH};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    #[cfg(feature = "desktop")]
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus_desktop::Config::new()
                .with_window(
                    dioxus_desktop::WindowBuilder::new()
                        .with_title(APP_NAME)
                        .with_inner_size(dioxus_desktop::LogicalSize::new(
                            DEFAULT_WIDTH,
                            DEFAULT_HEIGHT,
                        ))
                        .with_resizable(true),
                )
                .with_window(dioxus_desktop::WindowBuilder::new().with_resizable(true)),
        )
        .launch(App);

    #[cfg(not(feature = "desktop"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Title { "{APP_NAME}" }
        div { class: "h-screen bg-gray-100 overflow-hidden", TodoApp {} }
    }
}
