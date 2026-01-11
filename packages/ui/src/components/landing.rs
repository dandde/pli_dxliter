use crate::components::InteractiveTransliterationCard;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Landing() -> Element {
    rsx! {
        div { class: "container",
            h1 { "Transliteration for Sanskrit & Pali" }
            p { class: "subtitle", "A fast, modern, and cross-platform tool for Indosphere scripts." }

            div { class: "hero-actions",
                Link {
                    to: Route::TransliterationApp {},
                    class: "btn-primary",
                    "Open App"
                }
            }

            InteractiveTransliterationCard {}

            div { class: "features-grid",
                div { class: "feature-card",
                    h3 { "Vidyut Powered" }
                    p { "Uses the robust vidyut-lipi library for accurate transliteration." }
                }
                div { class: "feature-card",
                    h3 { "Multi-Script" }
                    p { "Supports Devanagari, IAST, SLP1, Harvard-Kyoto, and more." }
                }
                div { class: "feature-card",
                    h3 { "Responsive" }
                    p { "Designed for Web, Desktop, and Mobile platforms." }
                }
            }
        }
    }
}
