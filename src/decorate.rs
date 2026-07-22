use rand::seq::SliceRandom;
use rand::Rng;

/// Optional Monkeytype-style transforms applied to generated words: sprinkled
/// punctuation/capitalization and interspersed numbers. Both default off, in
/// which case [`WordMods::apply`] returns the words untouched.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WordMods {
    pub punctuation: bool,
    pub numbers: bool,
}

impl WordMods {
    pub fn any(&self) -> bool {
        self.punctuation || self.numbers
    }

    /// Decorate `words` per the enabled mods, returning new tokens the caller
    /// space-joins. With nothing enabled the input is returned unchanged.
    pub fn apply(&self, words: &[String]) -> Vec<String> {
        if !self.any() || words.is_empty() {
            return words.to_vec();
        }
        self.apply_with(words, &mut rand::thread_rng())
    }

    /// Core transform, generic over the rng so tests can seed it.
    fn apply_with<R: Rng>(&self, words: &[String], rng: &mut R) -> Vec<String> {
        const ENDERS: [char; 3] = ['.', '!', '?'];
        const MID: [char; 3] = [',', ';', ':'];
        const WRAPS: [(char, char); 3] = [('"', '"'), ('(', ')'), ('\'', '\'')];

        let mut out = Vec::with_capacity(words.len());
        // punctuation reads as sentences, so the first word is capitalized.
        let mut capitalize_next = self.punctuation;

        for w in words {
            // numbers occasionally stand in for a word; they never take
            // capitalization, so a pending capital carries to the next word.
            if self.numbers && rng.gen_bool(0.15) {
                out.push(rng.gen_range(0..10_000).to_string());
                continue;
            }

            let mut token = w.clone();

            if self.punctuation {
                if capitalize_next {
                    token = capitalize(&token);
                    capitalize_next = false;
                }
                if rng.gen_bool(0.03) {
                    let (l, r) = *WRAPS.choose(rng).unwrap();
                    token = format!("{l}{token}{r}");
                }
                if rng.gen_bool(0.15) {
                    let p = if rng.gen_bool(0.5) {
                        *ENDERS.choose(rng).unwrap()
                    } else {
                        *MID.choose(rng).unwrap()
                    };
                    token.push(p);
                    capitalize_next = ENDERS.contains(&p);
                }
            }

            out.push(token);
        }

        // close the passage on a full stop so it reads as finished.
        if self.punctuation {
            if let Some(last) = out.last_mut() {
                while last.ends_with(MID) {
                    last.pop();
                }
                if !last.ends_with(ENDERS) {
                    last.push('.');
                }
            }
        }

        out
    }
}

/// Uppercase the first character of `w`, leaving the rest untouched.
fn capitalize(w: &str) -> String {
    let mut chars = w.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn words(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn disabled_returns_input_unchanged() {
        let w = words(&["the", "quick", "brown"]);
        let mods = WordMods::default();
        assert!(!mods.any());
        assert_eq!(mods.apply(&w), w);
    }

    #[test]
    fn empty_input_is_safe() {
        let mods = WordMods {
            punctuation: true,
            numbers: true,
        };
        assert!(mods.apply(&[]).is_empty());
    }

    #[test]
    fn punctuation_capitalizes_and_closes() {
        let mods = WordMods {
            punctuation: true,
            numbers: false,
        };
        let w = words(&["alpha", "beta", "gamma", "delta", "epsilon"]);
        let mut rng = StdRng::seed_from_u64(7);
        let out = mods.apply_with(&w, &mut rng);
        // same token count (no numbers to replace anything)
        assert_eq!(out.len(), w.len());
        // first word capitalized
        assert!(out[0].starts_with('A'));
        // passage ends on a sentence-ender
        assert!(out.last().unwrap().ends_with(['.', '!', '?']));
        // some punctuation actually appears somewhere
        let joined = out.join(" ");
        assert!(joined.contains(['.', ',', '!', '?', ';', ':']));
    }

    #[test]
    fn numbers_inserts_digit_tokens() {
        let mods = WordMods {
            punctuation: false,
            numbers: true,
        };
        // many words so the 15% chance reliably fires at least once
        let w = words(&vec!["word"; 200]);
        let mut rng = StdRng::seed_from_u64(1);
        let out = mods.apply_with(&w, &mut rng);
        assert!(out.iter().any(|t| t.parse::<u32>().is_ok()));
        // untouched tokens are still the original word
        assert!(out.iter().any(|t| t == "word"));
    }

    #[test]
    fn capitalize_handles_unicode_and_empty() {
        assert_eq!(capitalize("épée"), "Épée");
        assert_eq!(capitalize(""), "");
    }
}
