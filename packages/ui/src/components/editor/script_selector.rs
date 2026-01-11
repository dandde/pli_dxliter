use dioxus::prelude::*;

#[component]
pub fn ScriptSelector(
    label: String,
    value: String,
    onchange: EventHandler<String>,
    scripts: Vec<(String, String)>,
) -> Element {
    rsx! {
        div { class: "input-block",
            div { class: "label-row",
                label { "{label}" }
                span { class: "script-tag", "{value}" }
            }
            select {
                value: "{value}",
                onchange: move |evt| onchange.call(evt.value()),
                for (id, name) in scripts {
                    option { value: "{id}", "{name}" }
                }
            }
        }
    }
}
