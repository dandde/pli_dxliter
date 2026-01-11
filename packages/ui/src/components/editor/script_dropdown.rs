use dioxus::prelude::*;
use vidyut_lipi::{Lipika, Scheme};

#[derive(Props, Clone, PartialEq)]
pub struct ScriptDropdownProps {
    pub label: String,
    pub current_script: String,
    pub scripts: Vec<(String, String)>,
    pub on_select: EventHandler<String>,
}

#[component]
pub fn ScriptDropdown(props: ScriptDropdownProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut lipika = Lipika::new();

    let scripts_with_badges: Vec<(String, String, String)> = props
        .scripts
        .iter()
        .map(|(id, name)| {
            let badge = if id == "Normal"
                || id == "HarvardKyoto"
                || id == "Iast"
                || id == "Slp1"
                || id == "Wx"
                || id == "Velthuis"
                || id == "Iso15919"
                || id == "Itrans"
            {
                // For romanized/technical scripts, use a clean 'a' or 'ā'
                if id == "Normal" {
                    "a".to_string()
                } else {
                    "ā".to_string()
                }
            } else {
                let target = id.parse::<Scheme>().unwrap_or(Scheme::Devanagari);
                lipika.transliterate("A", Scheme::HarvardKyoto, target)
            };
            (id.clone(), name.clone(), badge)
        })
        .collect();

    let current_badge = scripts_with_badges
        .iter()
        .find(|(id, _, _)| id == &props.current_script)
        .map(|(_, _, b)| b.clone())
        .unwrap_or_else(|| "ā".to_string());

    let current_name = props
        .scripts
        .iter()
        .find(|(id, _)| id == &props.current_script)
        .map(|(_, n)| n.clone())
        .unwrap_or_else(|| "Select Script".to_string());

    rsx! {
        div { class: "custom-dropdown-container",
            button {
                class: "dropdown-trigger",
                onclick: move |_| is_open.toggle(),
                div { class: "script-badge-mini", "{current_badge}" }
                span { "{current_name}" }
                svg {
                    class: if is_open() { "chevron open" } else { "chevron" },
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path { d: "M19 9l-7 7-7-7" }
                }
            }

            if is_open() {
                div { class: "dropdown-menu",
                    for (id, name, badge) in scripts_with_badges {
                        div {
                            class: if id == props.current_script { "dropdown-item active" } else { "dropdown-item" },
                            onclick: move |_| {
                                props.on_select.call(id.clone());
                                is_open.set(false);
                            },
                            div { class: "script-badge", "{badge}" }
                            span { class: "script-name", "{name}" }
                            if id == props.current_script {
                                svg {
                                    class: "check-icon",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    path { d: "M5 13l4 4L19 7" }
                                }
                            }
                        }
                    }
                }
                // Invisible overlay to close on click outside
                div {
                    class: "dropdown-overlay",
                    onclick: move |_| is_open.set(false)
                }
            }
        }
    }
}
