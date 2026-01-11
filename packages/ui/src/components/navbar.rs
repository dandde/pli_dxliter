use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar",
            Link {
                to: Route::Landing {},
                class: "nav-brand",
                Logo {}
                span { "Lipika" }
            }
            div { class: "nav-links",
                div { class: "nav-item tooltip",
                    Link { to: Route::Landing {},
                        svg {
                            class: "nav-icon",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" }
                        }
                    }
                    span { class: "tooltip-text", "Home" }
                }
                div { class: "nav-item tooltip",
                    Link { to: Route::TransliterationApp {},
                        svg {
                            class: "nav-icon",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" }
                        }
                    }
                    span { class: "tooltip-text", "App" }
                }
                div { class: "nav-item tooltip",
                    Link { to: Route::FileUpload {},
                        svg {
                            class: "nav-icon",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" }
                        }
                    }
                    span { class: "tooltip-text", "File Upload" }
                }
                div { class: "nav-item tooltip",
                    Link { to: Route::Landing {}, // Mapping About to Landing for now as it contains features
                        svg {
                            class: "nav-icon",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            circle { cx: "12", cy: "12", r: "10" }
                            line { x1: "12", y1: "16", x2: "12", y2: "12" }
                            line { x1: "12", y1: "8", x2: "12.01", y2: "8" }
                        }
                    }
                    span { class: "tooltip-text", "About" }
                }
            }
        }
    }
}

#[component]
fn Logo() -> Element {
    rsx! {
        svg {
            class: "brand-logo",
            view_box: "0 0 100 100",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            // Outer circular arrows (simplified representation of the app icon)
            path { d: "M20 30 A 35 35 0 1 1 80 70 M80 70 L 70 70 M80 70 L 80 60" }
            path { d: "M80 70 A 35 35 0 1 1 20 30 M20 30 L 30 30 M20 30 L 20 40" }

            // Indic characters in the center: क (Dev), ක (Sin), ക (Mal), ก (Thai)
            // Using text elements for crispness and easy color matching
            text {
                x: "50",
                y: "55",
                text_anchor: "middle",
                font_size: "18",
                font_family: "serif",
                fill: "currentColor",
                "क ක"
            }
            text {
                x: "50",
                y: "72",
                text_anchor: "middle",
                font_size: "18",
                font_family: "serif",
                fill: "currentColor",
                "ക ก"
            }
        }
    }
}
