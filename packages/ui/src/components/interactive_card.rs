use dioxus::prelude::*;
use std::time::Duration;
use vidyut_lipi::Scheme;

#[component]
pub fn InteractiveTransliterationCard() -> Element {
    let text = "namo tassa bhagavato arahato sammāsambuddhassa";
    let mut current_script = use_signal(|| "Devanagari".to_string());
    let mut transliterated_text = use_signal(|| "".to_string());

    use_future(move || async move {
        let scripts = vec![
            ("Devanagari", "Devanagari"),
            ("Assamese", "Assamese"),
            ("Bengali", "Bengali"),
            ("Bhaiksuki", "Bhaiksuki"),
            ("Brahmi", "Brahmi"),
            ("Burmese", "Burmese"),
            ("Gujarati", "Gujarati"),
            ("Kannada", "Kannada"),
            ("Khmer", "Khmer"),
            ("Malayalam", "Malayalam"),
            ("Odia", "Odia"),
            ("Sinhala", "Sinhala"),
            ("Tamil", "Tamil"),
            ("Telugu", "Telugu"),
            ("Thai", "Thai"),
            ("Tibetan", "Tibetan"),
            ("HarvardKyoto", "Harvard-Kyoto"),
            ("Iast", "IAST"),
            ("Iso15919", "ISO 15919"),
            ("Itrans", "ITRANS"),
            ("Slp1", "SLP1"),
            ("Velthuis", "Velthuis"),
            ("Wx", "WX"),
        ];

        // A very simple LCG for random-ish selection
        let mut seed = 12345u32;

        loop {
            // LCG step
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let idx = (seed as usize) % scripts.len();
            let (script_id, script_name) = scripts[idx];

            let from = Scheme::HarvardKyoto;
            let target = script_id.parse::<Scheme>().unwrap_or(Scheme::Devanagari);

            let result = crate::transliterate(text, from, target, script_id);

            current_script.set(script_name.to_string());
            transliterated_text.set(result);

            // Wait for 3 seconds before next cycle
            // Using a standard async sleep that works across platforms in Dioxus
            #[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
            tokio::time::sleep(Duration::from_secs(3)).await;
            #[cfg(any(target_arch = "wasm32", not(feature = "desktop")))]
            async_std::task::sleep(Duration::from_secs(3)).await;
        }
    });

    rsx! {
        div { class: "interactive-card-outer",
            div { class: "interactive-card",
                div { class: "card-header",
                    span { class: "script-label", "{current_script}" }
                    div { class: "progress-bar-mini",
                        div { class: "progress-fill-mini" }
                    }
                }
                div { class: "card-content",
                    p { class: "transliterated-display", "{transliterated_text}" }
                }
            }
        }
    }
}
