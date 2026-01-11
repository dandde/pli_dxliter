use crate::components::editor::script_dropdown::ScriptDropdown;
use crate::components::editor::textarea::ReusableTextArea;
use dioxus::prelude::*;
use vidyut_lipi::{detect, Scheme};

#[component]
pub fn TransliterationApp() -> Element {
    let mut input_text = use_signal(|| String::new());
    let mut output_text = use_signal(|| String::new());
    let mut from_script = use_signal(|| "HarvardKyoto".to_string());
    let mut to_script = use_signal(|| "Devanagari".to_string());
    let mut detected_script = use_signal(|| "(none)".to_string());

    // Comprehensive scripts list from vidyut-lipi official example
    let scripts = vec![
        ("Devanagari", "Devanagari"),
        ("Assamese", "Assamese"),
        ("Balinese", "Balinese"),
        ("Bengali", "Bengali"),
        ("Bhaiksuki", "Bhaiksuki"),
        ("Brahmi", "Brahmi"),
        ("Burmese", "Burmese"),
        ("Cham", "Cham"),
        ("Dogra", "Dogra"),
        ("Grantha", "Grantha"),
        ("Gujarati", "Gujarati"),
        ("GunjalaGondi", "Gunjala Gondi"),
        ("Gurmukhi", "Gurmukhi"),
        ("Javanese", "Javanese"),
        ("Kaithi", "Kaithi"),
        ("Kannada", "Kannada"),
        ("Khmer", "Khmer"),
        ("Khudawadi", "Khudawadi"),
        ("Limbu", "Limbu"),
        ("Malayalam", "Malayalam"),
        ("MasaramGondi", "Masaram Gondi"),
        ("MeeteiMayek", "Meetei Mayek"),
        ("Modi", "Modi"),
        ("Nandinagari", "Nandinagari"),
        ("Newa", "Newa (Nepal Bhasa)"),
        ("Odia", "Odia"),
        ("OlChiki", "Ol Chiki"),
        ("Saurashtra", "Saurashtra"),
        ("Sharada", "Sharada"),
        ("Siddham", "Siddham"),
        ("Sinhala", "Sinhala"),
        ("TaiTham", "Tai Tham"),
        ("Takri", "Takri"),
        ("Tamil", "Tamil"),
        ("Telugu", "Telugu"),
        ("Thai", "Thai"),
        ("Tibetan", "Tibetan"),
        ("Tirhuta", "Tirhuta"),
        ("ZanabazarSquare", "Zanabazar Square"),
        ("BarahaSouth", "Baraha (Southern)"),
        ("HarvardKyoto", "Harvard-Kyoto"),
        ("Iast", "IAST"),
        ("Iso15919", "ISO 15919"),
        ("Itrans", "ITRANS"),
        ("Slp1", "SLP1"),
        ("Velthuis", "Velthuis"),
        ("Wx", "WX"),
    ];

    let scripts_data: Vec<(String, String)> = scripts
        .into_iter()
        .map(|(id, name)| (id.to_string(), name.to_string()))
        .collect();

    fn to_scheme(s: &str) -> Scheme {
        match s {
            "Devanagari" => Scheme::Devanagari,
            "Assamese" => Scheme::Assamese,
            "Balinese" => Scheme::Balinese,
            "Bengali" => Scheme::Bengali,
            "Bhaiksuki" => Scheme::Bhaiksuki,
            "Brahmi" => Scheme::Brahmi,
            "Burmese" => Scheme::Burmese,
            "Cham" => Scheme::Cham,
            "Dogra" => Scheme::Dogra,
            "Grantha" => Scheme::Grantha,
            "Gujarati" => Scheme::Gujarati,
            "GunjalaGondi" => Scheme::GunjalaGondi,
            "Gurmukhi" => Scheme::Gurmukhi,
            "Javanese" => Scheme::Javanese,
            "Kaithi" => Scheme::Kaithi,
            "Kannada" => Scheme::Kannada,
            "Khmer" => Scheme::Khmer,
            "Khudawadi" => Scheme::Khudawadi,
            "Limbu" => Scheme::Limbu,
            "Malayalam" => Scheme::Malayalam,
            "MasaramGondi" => Scheme::MasaramGondi,
            "MeeteiMayek" => Scheme::MeeteiMayek,
            "Modi" => Scheme::Modi,
            "Nandinagari" => Scheme::Nandinagari,
            "Newa" => Scheme::Newa,
            "Odia" => Scheme::Odia,
            "OlChiki" => Scheme::OlChiki,
            "Saurashtra" => Scheme::Saurashtra,
            "Sharada" => Scheme::Sharada,
            "Siddham" => Scheme::Siddham,
            "Sinhala" => Scheme::Sinhala,
            "TaiTham" => Scheme::TaiTham,
            "Takri" => Scheme::Takri,
            "Tamil" => Scheme::Tamil,
            "Telugu" => Scheme::Telugu,
            "Thai" => Scheme::Thai,
            "Tibetan" => Scheme::Tibetan,
            "Tirhuta" => Scheme::Tirhuta,
            "ZanabazarSquare" => Scheme::ZanabazarSquare,
            "BarahaSouth" => Scheme::BarahaSouth,
            "HarvardKyoto" => Scheme::HarvardKyoto,
            "Iast" => Scheme::Iast,
            "Iso15919" => Scheme::Iso15919,
            "Itrans" => Scheme::Itrans,
            "Slp1" => Scheme::Slp1,
            "Velthuis" => Scheme::Velthuis,
            "Wx" => Scheme::Wx,
            _ => Scheme::Devanagari,
        }
    }

    use_effect(move || {
        let text = input_text.read();
        if text.is_empty() {
            detected_script.set("(none)".to_string());
            output_text.set("".to_string());
            return;
        }

        // Script Detection
        if let Some(scheme) = detect(&*text) {
            let scheme_name = format!("{:?}", scheme);
            detected_script.set(scheme_name);
        } else {
            detected_script.set("Unknown".to_string());
        }

        // Transliteration
        let result = crate::transliterate(
            &*text,
            to_scheme(&*from_script.read()),
            to_scheme(&*to_script.read()),
            &*to_script.read(),
        );
        output_text.set(result);
    });

    rsx! {
        div { class: "container editor-container",
            h1 { "Transliteration Editor" }
            p { class: "subtitle", "High-performance real-time transliteration for Sanskrit and Pali." }

            div { class: "converter-grid",
                div { class: "input-block",
                    div { class: "label-row",
                        label { "Source" }
                        div { class: "script-controls",
                            span { class: "script-tag", "{from_script}" }
                            button {
                                class: "btn-detect",
                                onclick: move |_| {
                                    let detected = detected_script.read().clone();
                                    if detected != "(none)" && detected != "Unknown" {
                                        from_script.set(detected);
                                    }
                                },
                                "Use: {detected_script}"
                            }
                        }
                    }
                    div { class: "selector-row",
                        ScriptDropdown {
                            label: "Source",
                            current_script: from_script.read().clone(),
                            scripts: scripts_data.clone(),
                            on_select: move |id| from_script.set(id),
                        }
                    }
                    ReusableTextArea {
                        value: input_text.read().to_string(),
                        placeholder: "Type here...",
                        readonly: false,
                        oninput: move |v| input_text.set(v)
                    }
                }

                div { class: "input-block",
                    div { class: "label-row",
                        label { "Target" }
                        span { class: "script-tag", "{to_script}" }
                    }
                    div { class: "selector-row",
                        ScriptDropdown {
                            label: "Target",
                            current_script: to_script.read().clone(),
                            scripts: scripts_data.clone(),
                            on_select: move |id| to_script.set(id),
                        }
                    }
                    ReusableTextArea {
                        value: output_text.read().to_string(),
                        placeholder: "Transliterated text will appear here...",
                        readonly: true,
                    }
                }
            }

            div { class: "action-row",
                div { class: "action-left",
                    button {
                        class: "btn-secondary btn-compact btn-with-icon",
                        onclick: move |_| {
                            let old_from = from_script.read().clone();
                            let old_to = to_script.read().clone();
                            let old_output = output_text.read().clone();

                            from_script.set(old_to);
                            to_script.set(old_from);
                            input_text.set(old_output);
                        },
                        svg {
                            class: "icon-swap icon-horizontal",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "M8 7h12m0 0l-4-4m4 4l-4 4M16 17H4m0 0l4-4m-4 4l4 4" }
                        }
                        svg {
                            class: "icon-swap icon-vertical",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" }
                        }
                        span { "Swap Scripts" }
                    }
                }
                div { class: "action-right",
                     button {
                        class: "btn-primary btn-compact btn-with-icon",
                        onclick: move |_| {
                            // Copy to clipboard logic
                        },
                        svg {
                            class: "btn-icon",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3" }
                        }
                        span { "Copy Output" }
                    }
                }
            }
        }
    }
}
