//! Process-level locale normalization for text stack dependencies.

use sys_locale::get_locale;

/// Keep third-party text segmentation on a locale that is available in bundled ICU data.
pub fn normalize_process_locale() {
    let locale = get_locale().unwrap_or_else(|| "en-US".to_string());
    let normalized = locale.to_ascii_lowercase();
    let safe_locale = if normalized.starts_with("zh") {
        "zh-CN"
    } else {
        "en-US"
    };

    // SAFETY: Called from binary `main` before Bevy starts worker threads.
    unsafe {
        std::env::set_var("LANG", safe_locale);
        std::env::set_var("LANGUAGE", safe_locale);
        std::env::set_var("LC_ALL", safe_locale);
        std::env::set_var("LC_CTYPE", safe_locale);
    }
}
