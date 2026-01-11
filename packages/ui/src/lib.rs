// use dioxus_router::prelude::*; // If using separate crate
// In Dioxus 0.7 with "router" feature, it might be in prelude or dioxus::router
use dioxus::prelude::*;

pub mod components;
pub use components::*;

pub mod transliteration;
pub use transliteration::*;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Landing {},
        #[route("/app")]
        TransliterationApp {},
        #[route("/upload")]
        FileUpload {},
    #[end_layout]
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

#[component]
fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { class: "container",
            h1 { "404 - Page Not Found" }
            p { "The page you are looking for does not exist." }
            Link { to: Route::Landing {}, "Go Home" }
        }
    }
}
