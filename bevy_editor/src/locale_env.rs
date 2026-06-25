//! Process-level locale normalization for text stack dependencies.

use sys_locale::get_locale;

/// UI language supported by the editor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SupportedLocale {
    /// Simplified Chinese.
    ZhCn,
    /// English fallback.
    EnUs,
}

impl SupportedLocale {
    /// Return a BCP-47 locale identifier supported by the bundled text stack data.
    pub const fn as_bcp47(self) -> &'static str {
        match self {
            Self::ZhCn => "zh-CN",
            Self::EnUs => "en-US",
        }
    }

    /// Detect the requested UI language, limited to Chinese and English.
    pub fn detect() -> Self {
        let locale = get_locale().unwrap_or_else(|| "en-US".to_string());
        let normalized = locale.to_ascii_lowercase();
        if normalized.starts_with("zh") {
            Self::ZhCn
        } else {
            Self::EnUs
        }
    }
}

/// Keep third-party text segmentation on a locale that is available in bundled ICU data.
pub fn normalize_process_locale() {
    let safe_locale = SupportedLocale::detect().as_bcp47();

    // SAFETY: Called from binary `main` before Bevy starts worker threads.
    unsafe {
        std::env::set_var("LANG", safe_locale);
        std::env::set_var("LANGUAGE", safe_locale);
        std::env::set_var("LC_ALL", safe_locale);
        std::env::set_var("LC_CTYPE", safe_locale);
    }
}
