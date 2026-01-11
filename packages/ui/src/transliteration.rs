use lazy_regex::regex;
use vidyut_lipi::{Lipika, Scheme};

pub fn is_latin(scheme: &str) -> bool {
    match scheme {
        "HarvardKyoto" | "Iast" | "Iso15919" | "Itrans" | "Slp1" | "Velthuis" | "Wx" | "Normal" => {
            true
        }
        _ => false,
    }
}

pub fn transliterate(text: &str, from: Scheme, to: Scheme, to_name: &str) -> String {
    let mut lipika = Lipika::new();

    // 1. Case Normalization if target is not Latin
    let input = if !is_latin(to_name) {
        text.to_lowercase()
    } else {
        text.to_string()
    };

    let mut result = lipika.transliterate(&input, from, to);

    if from == Scheme::Iso15919 {
        result = result.replace("ṃ", "ṁ").replace("l̤", "ḷ");
    } else if from == Scheme::Iast {
        result = result.replace("ṁ", "ṃ").replace("ḷ", "l̤");
    }

    // 2. Myanmar Post-Processing
    if to_name == "Burmese" {
        // result = result.replace("ṃ", "ṁ");
        result = post_process_myanmar(&result);
    }

    result
}

fn post_process_myanmar(text: &str) -> String {
    let replacements = [
        (regex!(r"[,;]"), "၊"),
        (regex!(r"[\u{2026}\u{0964}\u{0965}]+"), "။"),
        (regex!(r"ဉ\u{1039}ဉ"), "ည"),
        (regex!(r"သ\u{1039}သ"), "ဿ"),
        (regex!(r"င္([က-ဠ])"), "င\u{103A}္$1"),
        (regex!(r"္ယ"), "ျ"),
        (regex!(r"္ရ"), "ြ"),
        (regex!(r"္ဝ"), "ွ"),
        (regex!(r"္ဟ"), "ှ"),
        (regex!(r"([ခဂငဒပဝ]ေ?)\u{102c}"), "$1\u{102b}"),
        (regex!(r"(က္ခ|န္ဒ|ပ္ပ|မ္ပ)(ေ?)\u{102b}"), "$1$2\u{102c}"),
        (regex!(r"(ဒ္ဓ|ဒွ)(ေ?)\u{102c}"), "$1$2\u{102b}"),
    ];

    let mut result = text.to_string();
    for (pattern, replace_with) in replacements {
        result = pattern.replace_all(&result, replace_with).to_string();
    }
    result
}
