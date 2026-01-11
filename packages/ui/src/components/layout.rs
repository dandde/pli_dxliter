use crate::components::Navbar;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "app-container",
            Navbar {}
            Outlet::<Route> {}
        }
    }
}
