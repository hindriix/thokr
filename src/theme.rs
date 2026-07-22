use directories::ProjectDirs;
use ratatui::style::Color;
use serde::Deserialize;

/// The colors that drive the whole tui. Every renderable element pulls from
/// here, so a preset or a user config file fully re-skins thokr.
///
/// The [`Theme::default`] values reproduce thokr's original hardcoded look
/// exactly (green / red / magenta on the terminal's own foreground), so an
/// unconfigured install is visually unchanged.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    /// correctly typed characters
    pub correct: Color,
    /// mistyped characters
    pub incorrect: Color,
    /// not-yet-typed characters (rendered dim)
    pub pending: Color,
    /// the wpm-over-time results graph
    pub graph: Color,
    /// the live countdown in timed mode
    pub timer: Color,
    /// results text and chart axes
    pub text: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            correct: Color::Green,
            incorrect: Color::Red,
            pending: Color::Reset,
            graph: Color::Magenta,
            timer: Color::Reset,
            text: Color::Reset,
        }
    }
}

/// Built-in, selectable color presets. `Default` is the classic thokr palette.
#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::ValueEnum, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum ThemePreset {
    Default,
    Matrix,
    Dracula,
    Ocean,
    Mono,
}

impl ThemePreset {
    /// The palette for this preset.
    pub fn theme(self) -> Theme {
        match self {
            ThemePreset::Default => Theme::default(),
            ThemePreset::Matrix => Theme {
                correct: Color::Green,
                incorrect: Color::LightRed,
                pending: Color::Rgb(0, 90, 0),
                graph: Color::LightGreen,
                timer: Color::Green,
                text: Color::Green,
            },
            ThemePreset::Dracula => Theme {
                correct: Color::Rgb(80, 250, 123),
                incorrect: Color::Rgb(255, 85, 85),
                pending: Color::Rgb(98, 114, 164),
                graph: Color::Rgb(189, 147, 249),
                timer: Color::Rgb(241, 250, 140),
                text: Color::Rgb(248, 248, 242),
            },
            ThemePreset::Ocean => Theme {
                correct: Color::Rgb(126, 214, 223),
                incorrect: Color::Rgb(255, 121, 121),
                pending: Color::Rgb(69, 105, 144),
                graph: Color::Rgb(116, 199, 236),
                timer: Color::Rgb(129, 236, 236),
                text: Color::Rgb(223, 233, 245),
            },
            ThemePreset::Mono => Theme {
                correct: Color::White,
                incorrect: Color::Rgb(160, 160, 160),
                pending: Color::DarkGray,
                graph: Color::White,
                timer: Color::Gray,
                text: Color::White,
            },
        }
    }
}

/// Optional user overrides, read from `theme.json` in thokr's config dir. Every
/// field is optional and layered over the selected preset, so a config can tweak
/// a single color or repaint everything. Unknown or unparseable values are
/// ignored in favor of the preset, so a typo never leaves the ui unreadable.
#[derive(Debug, Default, Deserialize)]
struct ThemeOverrides {
    #[serde(default)]
    preset: Option<String>,
    #[serde(default)]
    correct: Option<String>,
    #[serde(default)]
    incorrect: Option<String>,
    #[serde(default)]
    pending: Option<String>,
    #[serde(default)]
    graph: Option<String>,
    #[serde(default)]
    timer: Option<String>,
    #[serde(default)]
    text: Option<String>,
}

impl Theme {
    /// Resolve the active theme: start from `preset` (or a `preset` named in the
    /// config file), then layer any per-color overrides from `theme.json`.
    pub fn resolve(preset: ThemePreset) -> Self {
        let overrides = load_overrides().unwrap_or_default();

        // a preset named in the config file only applies when the user didn't
        // ask for one on the command line (Default is the un-asked-for value).
        let base = match (preset, overrides.preset.as_deref().and_then(parse_preset)) {
            (ThemePreset::Default, Some(cfg_preset)) => cfg_preset.theme(),
            _ => preset.theme(),
        };

        base.with_overrides(&overrides)
    }

    fn with_overrides(mut self, o: &ThemeOverrides) -> Self {
        for (slot, raw) in [
            (&mut self.correct, &o.correct),
            (&mut self.incorrect, &o.incorrect),
            (&mut self.pending, &o.pending),
            (&mut self.graph, &o.graph),
            (&mut self.timer, &o.timer),
            (&mut self.text, &o.text),
        ] {
            if let Some(color) = raw.as_deref().and_then(parse_color) {
                *slot = color;
            }
        }
        self
    }
}

fn load_overrides() -> Option<ThemeOverrides> {
    let proj_dirs = ProjectDirs::from("", "", "thokr")?;
    let path = proj_dirs.config_dir().join("theme.json");
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn parse_preset(s: &str) -> Option<ThemePreset> {
    match s.trim().to_ascii_lowercase().as_str() {
        "default" => Some(ThemePreset::Default),
        "matrix" => Some(ThemePreset::Matrix),
        "dracula" => Some(ThemePreset::Dracula),
        "ocean" => Some(ThemePreset::Ocean),
        "mono" => Some(ThemePreset::Mono),
        _ => None,
    }
}

/// Parse a color from a `#rrggbb` hex string or a named terminal color.
fn parse_color(s: &str) -> Option<Color> {
    let t = s.trim();

    if let Some(hex) = t.strip_prefix('#') {
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            return Some(Color::Rgb(r, g, b));
        }
        return None;
    }

    Some(match t.to_ascii_lowercase().as_str() {
        "reset" | "default" => Color::Reset,
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" | "grey" => Color::Gray,
        "darkgray" | "darkgrey" => Color::DarkGray,
        "lightred" => Color::LightRed,
        "lightgreen" => Color::LightGreen,
        "lightyellow" => Color::LightYellow,
        "lightblue" => Color::LightBlue,
        "lightmagenta" => Color::LightMagenta,
        "lightcyan" => Color::LightCyan,
        "white" => Color::White,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_preset_matches_classic_palette() {
        assert_eq!(ThemePreset::Default.theme(), Theme::default());
        assert_eq!(Theme::default().correct, Color::Green);
        assert_eq!(Theme::default().incorrect, Color::Red);
        assert_eq!(Theme::default().graph, Color::Magenta);
    }

    #[test]
    fn parse_named_colors() {
        assert_eq!(parse_color("green"), Some(Color::Green));
        assert_eq!(parse_color("  LightBlue "), Some(Color::LightBlue));
        assert_eq!(parse_color("grey"), Some(Color::Gray));
        assert_eq!(parse_color("reset"), Some(Color::Reset));
    }

    #[test]
    fn parse_hex_colors() {
        assert_eq!(parse_color("#50fa7b"), Some(Color::Rgb(80, 250, 123)));
        assert_eq!(parse_color("#000000"), Some(Color::Rgb(0, 0, 0)));
    }

    #[test]
    fn parse_rejects_garbage() {
        assert_eq!(parse_color("nope"), None);
        assert_eq!(parse_color("#fff"), None);
        assert_eq!(parse_color("#gggggg"), None);
    }

    #[test]
    fn overrides_layer_over_preset() {
        let base = ThemePreset::Dracula.theme();
        let o = ThemeOverrides {
            correct: Some("#010203".to_string()),
            incorrect: Some("garbage".to_string()), // ignored, keeps preset
            ..Default::default()
        };
        let themed = base.with_overrides(&o);
        assert_eq!(themed.correct, Color::Rgb(1, 2, 3));
        assert_eq!(themed.incorrect, base.incorrect);
        assert_eq!(themed.graph, base.graph);
    }

    #[test]
    fn preset_names_round_trip() {
        for p in [
            ThemePreset::Default,
            ThemePreset::Matrix,
            ThemePreset::Dracula,
            ThemePreset::Ocean,
            ThemePreset::Mono,
        ] {
            assert_eq!(parse_preset(&p.to_string()), Some(p));
        }
    }
}
