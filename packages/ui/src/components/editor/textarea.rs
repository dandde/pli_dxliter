use dioxus::prelude::*;

#[component]
pub fn ReusableTextArea(
    value: String,
    placeholder: String,
    readonly: bool,
    oninput: Option<EventHandler<String>>,
) -> Element {
    rsx! {
        textarea {
            value: "{value}",
            placeholder: "{placeholder}",
            readonly: readonly,
            oninput: move |evt| {
                if let Some(handler) = oninput.as_ref() {
                    handler.call(evt.value());
                }
            }
        }
    }
}
