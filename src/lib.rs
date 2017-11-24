#[macro_use]
extern crate lazy_static;

mod data;

/// Convert latex expressions to their unicode representation.
///
/// Symbols, super- and subscripts and fonts are supported.
pub fn convert(s: &str) -> String {
    let s = convert_latex_symbols(s);
    apply_all_modifiers(&s)
}

/// Replace each "\alpha" and similar latex symbols with their unicode
/// representation.
pub fn convert_latex_symbols(s: &str) -> String {
    let mut result = s.to_string();
    for (&code, &val) in data::LATEX_SYMBOLS.iter() {
        result = result.replace(code, val);
    }
    result
}

/// Replace super- and subcripts and fonts with their unicode representation.
fn apply_all_modifiers(s: &str) -> String {
    let s = apply_modifier(s, "^", &data::SUPERSCRIPTS);
    let s = apply_modifier(&s, "_", &data::SUBSCRIPTS);
    let s = apply_modifier(&s, "\\bb", &data::TEXTBB);
    let s = apply_modifier(&s, "\\bf", &data::TEXTBF);
    let s = apply_modifier(&s, "\\it", &data::TEXTIT);
    let s = apply_modifier(&s, "\\cal", &data::TEXTCAL);
    let s = apply_modifier(&s, "\\frak", &data::TEXTFRAK);
    let s = apply_modifier(&s, "\\mono", &data::TEXTMONO);
    s
}

type Map = std::collections::HashMap<char, char>;

/// Apply a latex modifier using a given map.
///
/// Make sure to process "^" modifiers first.
fn apply_modifier(s: &str, modifier: &str, map: &Map) -> String {
    let s = s.clone().replace(modifier, "^");

    enum Mode { Normal, Modified, Long }

    let mut result = String::with_capacity(s.len());
    let mut mode = Mode::Normal;
    for ch in s.chars() {
        match mode {
            Mode::Normal if ch == '^' => {
                mode = Mode::Modified;
            },
            Mode::Modified if ch == '{' => {
                mode = Mode::Long;
            },
            Mode::Modified => {
                result.push(*map.get(&ch).unwrap_or(&ch));
                mode = Mode::Normal;
            },
            Mode::Long if ch == '}' => {
                mode = Mode::Normal;
            },
            Mode::Normal => {
                result.push(ch);
            },
            _ => {
                result.push(*map.get(&ch).unwrap_or(&ch));
            },
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_symbol() {
        assert_eq!(convert(r"\alpha"), "α");
    }
}
