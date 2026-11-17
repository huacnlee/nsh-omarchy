//! Omarchy's `colors.toml`, in both of its formats.
//!
//! The ANSI format names the terminal colors (`color0` … `color15`) beside
//! `background`, `foreground`, `accent` and `selection_*`; the semantic format
//! names roles (`red`, `muted`, `lighter_background`, …). Keys of both may be
//! mixed. A palette missing a required role, or holding a color that is not
//! `#RRGGBB`, is an error as a whole: no role is borrowed from another theme.

use nsh_core::color::{Hsla, Rgba, rgb};

use super::ThemeLoadError;
use crate::theme::{Appearance, Theme};

impl Theme {
    /// Reads the text of an Omarchy `colors.toml` as the palette `name`.
    ///
    /// `background`, `foreground`, `accent` and the status colors (`red` or
    /// `color1`, `yellow` or `color3`, `green` or `color2`) are required.
    /// Surfaces, secondary text, selection and borders not given are mixed from
    /// this palette's own colors. `mode` (`dark` or `light`) sets the
    /// appearance; without it the lighter of background and foreground decides.
    pub fn from_colors_toml(text: &str, name: &str) -> Result<Theme, ThemeLoadError> {
        let values: toml::Table =
            toml::from_str(text).map_err(|error| ThemeLoadError(error.to_string()))?;
        let color = |keys: &[&str]| -> Result<Option<Hsla>, ThemeLoadError> {
            for key in keys {
                if let Some(value) = values.get(*key) {
                    let text = value
                        .as_str()
                        .ok_or_else(|| ThemeLoadError(format!("{key} must be a hex color")))?;
                    return parse_hex(text)
                        .map(Some)
                        .ok_or_else(|| ThemeLoadError(format!("{key} must use #RRGGBB")));
                }
            }
            Ok(None)
        };
        let required = |keys: &[&str]| {
            color(keys)?.ok_or_else(|| ThemeLoadError(format!("missing {}", keys[0])))
        };

        let background = required(&["background"])?;
        let foreground = required(&["foreground"])?;
        let accent = required(&["accent"])?;
        let appearance = match values.get("mode") {
            Some(toml::Value::String(mode)) if mode == "light" => Appearance::Light,
            Some(toml::Value::String(mode)) if mode == "dark" => Appearance::Dark,
            Some(_) => return Err(ThemeLoadError("mode must be dark or light".into())),
            None if background.luminance() > foreground.luminance() => Appearance::Light,
            None => Appearance::Dark,
        };
        // Neutral layers come from the user's colors, never from another theme.
        let surface =
            color(&["lighter_background"])?.unwrap_or_else(|| mix(background, foreground, 0.05));
        let inset =
            color(&["dark_background"])?.unwrap_or_else(|| mix(background, foreground, 0.08));
        let bright =
            color(&["bright_foreground", "selection_foreground", "cursor"])?.unwrap_or(foreground);
        let secondary = mix(background, foreground, 0.75);
        let danger = required(&["red", "color1"])?;
        let warning = required(&["yellow", "color3"])?;
        let success = required(&["green", "color2"])?;
        let chart = chart_palette(
            accent,
            &[
                color(&["magenta", "color5"])?,
                color(&["cyan", "color6"])?,
                Some(warning),
                Some(success),
                color(&["orange"])?,
                color(&["blue", "color4"])?,
                Some(danger),
            ],
            background,
        );
        let selection = color(&["selection", "selection_background"])?
            .unwrap_or_else(|| mix(background, accent, 0.2));
        let border =
            color(&["muted", "color8"])?.unwrap_or_else(|| mix(background, foreground, 0.25));
        let (black, white) = (rgb(0x000000).to_hsla(), rgb(0xffffff).to_hsla());
        let on_accent = if accent.contrast_ratio(black) > accent.contrast_ratio(white) {
            black
        } else {
            white
        };
        // Terminal colors only when the theme gives all sixteen.
        let mut ansi = [background; 16];
        let mut complete = true;
        for (index, slot) in ansi.iter_mut().enumerate() {
            match color(&[&format!("color{index}")])? {
                Some(value) => *slot = value,
                None => complete = false,
            }
        }

        Ok(Theme {
            name: name.to_owned().into(),
            appearance,
            background,
            surface,
            inset,
            foreground,
            secondary,
            bright,
            accent,
            on_accent,
            selection,
            border,
            danger,
            warning,
            success,
            chart,
            ansi: complete.then_some(ansi),
        })
    }
}

/// `#RRGGBB`, exactly: Omarchy's files use no other form.
fn parse_hex(text: &str) -> Option<Hsla> {
    let hex = text
        .strip_prefix('#')
        .filter(|hex| hex.len() == 6 && hex.bytes().all(|b| b.is_ascii_hexdigit()))?;
    u32::from_str_radix(hex, 16)
        .ok()
        .map(|value| rgb(value).to_hsla())
}

/// Series colors start from the accent and take the palette's hues in order,
/// skipping any too close to one already chosen to tell apart. A palette short
/// of distinct hues fills the rest with accent tints over the background.
fn chart_palette(accent: Hsla, candidates: &[Option<Hsla>], background: Hsla) -> [Hsla; 5] {
    let distinct = |a: Hsla, b: Hsla| {
        let (a, b) = (a.to_rgb(), b.to_rgb());
        let (r, g, bl) = (a.r - b.r, a.g - b.g, a.b - b.b);
        (r * r + g * g + bl * bl).sqrt() > 0.2
    };
    let mut chosen = vec![accent];
    for candidate in candidates.iter().flatten() {
        if chosen.len() == 5 {
            break;
        }
        if chosen.iter().all(|color| distinct(*color, *candidate)) {
            chosen.push(*candidate);
        }
    }
    let mut tint = 0.75;
    while chosen.len() < 5 {
        chosen.push(mix(background, accent, tint));
        tint -= 0.2;
    }
    [chosen[0], chosen[1], chosen[2], chosen[3], chosen[4]]
}

/// `a` moved `amount` of the way to `b`, in sRGB.
pub(super) fn mix(a: Hsla, b: Hsla, amount: f32) -> Hsla {
    let (a, b) = (a.to_rgb(), b.to_rgb());
    Rgba {
        r: a.r + (b.r - a.r) * amount,
        g: a.g + (b.g - a.g) * amount,
        b: a.b + (b.b - a.b) * amount,
        a: 1.0,
    }
    .to_hsla()
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;

    fn c(hex: u32) -> Hsla {
        rgb(hex).to_hsla()
    }

    /// The ANSI format: terminal colors plus background, foreground and accent.
    pub(crate) const ANSI: &str = "\
accent = '#205ea6'
background = '#fffcf0'
foreground = '#100f0f'
selection_background = '#dad8ce'
selection_foreground = '#100f0f'
color0 = '#100f0f'
color1 = '#af3029'
color2 = '#526600'
color3 = '#855b00'
color4 = '#205ea6'
color5 = '#a02f6f'
color6 = '#24837b'
color7 = '#6f6e69'
color8 = '#b7b5ac'
color9 = '#d14d41'
color10 = '#879a39'
color11 = '#d0a215'
color12 = '#4385be'
color13 = '#ce5d97'
color14 = '#3aa99f'
color15 = '#cecdc3'
";

    const SEMANTIC: &str = "\
mode = 'dark'
background = '#1a1b26'
foreground = '#a9b1d6'
accent = '#7aa2f7'
red = '#f7768e'
green = '#9ece6a'
yellow = '#e0af68'
lighter_background = '#24283b'
dark_background = '#13141c'
selection = '#292e42'
muted = '#414868'
";

    #[test]
    fn ansi_palette_maps_terminal_colors_and_infers_light_mode() {
        let theme = Theme::from_colors_toml(ANSI, "Flexoki").unwrap();
        assert_eq!(theme.name.as_ref(), "Flexoki");
        assert_eq!(theme.appearance, Appearance::Light);
        assert_eq!(theme.accent, c(0x205ea6));
        assert_eq!(theme.danger, c(0xaf3029));
        assert_eq!(theme.success, c(0x526600));
        assert_eq!(theme.warning, c(0x855b00));
        assert_eq!(theme.selection, c(0xdad8ce));
        assert_eq!(theme.border, c(0xb7b5ac));
        let ansi = theme.ansi.expect("all sixteen colors");
        assert_eq!(ansi[0], c(0x100f0f));
        assert_eq!(ansi[15], c(0xcecdc3));
        // Derived surfaces stay light, and accent text reads.
        assert!(theme.surface.luminance() > 0.7);
        assert!(theme.accent.contrast_ratio(theme.on_accent) >= 4.5);
    }

    #[test]
    fn semantic_palette_preserves_explicit_roles() {
        let theme = Theme::from_colors_toml(SEMANTIC, "Custom").unwrap();
        assert_eq!(theme.appearance, Appearance::Dark);
        assert_eq!(theme.surface, c(0x24283b));
        assert_eq!(theme.inset, c(0x13141c));
        assert_eq!(theme.selection, c(0x292e42));
        assert_eq!(theme.border, c(0x414868));
        assert_eq!(theme.danger, c(0xf7768e));
        assert_eq!(theme.ansi, None);
    }

    #[test]
    fn mode_overrides_luminance() {
        let theme = Theme::from_colors_toml(&format!("mode = 'dark'\n{ANSI}"), "T").unwrap();
        assert_eq!(theme.appearance, Appearance::Dark);
    }

    #[test]
    fn chart_series_start_at_accent_and_skip_indistinct_hues() {
        let source = "background = '#000000'\nforeground = '#efefef'\naccent = '#8a9fbe'\nred = '#d35f5f'\ngreen = '#8a9a7b'\nyellow = '#ffc107'\nblue = '#8a9fbe'\ncyan = '#88aabb'\nmagenta = '#c1a1c1'\n";
        let theme = Theme::from_colors_toml(source, "Test").unwrap();
        assert_eq!(theme.chart[0], theme.accent);
        // Cyan and blue sit on the accent, so the next distinct hues follow.
        assert_eq!(theme.chart[1], c(0xc1a1c1));
        assert_eq!(theme.chart[2], theme.warning);
        assert_eq!(theme.chart[3], theme.success);
        assert_eq!(theme.chart[4], theme.danger);
    }

    #[test]
    fn short_palettes_fill_chart_series_from_the_accent() {
        let source = "background = '#fffcf0'\nforeground = '#100f0f'\naccent = '#205ea6'\ncolor1 = '#af3029'\ncolor2 = '#526600'\ncolor3 = '#855b00'\n";
        let theme = Theme::from_colors_toml(source, "Test").unwrap();
        assert_eq!(theme.chart[0], theme.accent);
        for (index, color) in theme.chart.iter().enumerate() {
            for other in &theme.chart[index + 1..] {
                assert_ne!(color, other);
            }
        }
    }

    #[test]
    fn invalid_palettes_are_rejected_without_partial_defaults() {
        for source in [
            "",
            "background = 42",
            "broken toml",
            "background = '#fffcf0'\nforeground = '#100f0f'\naccent = '#205ea6'\n",
            &ANSI.replace("#205ea6", "#nope!!"),
            &ANSI.replace("#205ea6", "#205ea"),
            &ANSI.replace("#af3029", "red"),
            &format!("{ANSI}mode = 'automatic'\n"),
        ] {
            assert!(
                Theme::from_colors_toml(source, "Bad").is_err(),
                "accepted {source:?}"
            );
        }
    }
}
