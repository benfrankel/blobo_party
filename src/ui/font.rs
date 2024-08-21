use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;
use bevy::utils::HashMap;
use lazy_regex::regex;

use crate::core::window::WindowRoot;
use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    load_internal_binary_asset!(
        app,
        FONT_HANDLE,
        "../../assets/font/pypx.ttf",
        |bytes: &[u8], _path: String| Font::try_from_bytes(bytes.to_vec()).unwrap()
    );
    load_internal_binary_asset!(
        app,
        BOLD_FONT_HANDLE,
        "../../assets/font/pypx-B.ttf",
        |bytes: &[u8], _path: String| Font::try_from_bytes(bytes.to_vec()).unwrap()
    );
    load_internal_binary_asset!(
        app,
        THICK_FONT_HANDLE,
        "../../assets/font/pypx-T.ttf",
        |bytes: &[u8], _path: String| Font::try_from_bytes(bytes.to_vec()).unwrap()
    );

    app.configure::<DynamicFontSize>();
}

pub const FONT_HANDLE: Handle<Font> =
    Handle::weak_from_u128(303551798864246209986336759745415587961);
pub const BOLD_FONT_HANDLE: Handle<Font> =
    Handle::weak_from_u128(317423448069604009516378143395193332978);
pub const THICK_FONT_HANDLE: Handle<Font> =
    Handle::weak_from_u128(93153499609634570285243616548722721367);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DynamicFontSize {
    pub size: Val,
    pub step: f32,
    pub minimum: f32,
}

impl Configure for DynamicFontSize {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_dynamic_font_size.in_set(UpdateSet::SyncLate));
    }
}

impl DynamicFontSize {
    pub fn new(size: Val) -> Self {
        Self {
            size,
            step: 0.0,
            minimum: 0.0,
        }
    }

    pub fn with_step(mut self, step: f32) -> Self {
        self.step = step;
        self.minimum = self.minimum.max(step);
        self
    }

    pub fn with_minimum(mut self, minimum: f32) -> Self {
        self.minimum = minimum;
        self
    }
}

fn apply_dynamic_font_size(
    window_root: Res<WindowRoot>,
    window_query: Query<&Window>,
    mut text_query: Query<(&DynamicFontSize, &Node, &mut Text)>,
) {
    let window = rq!(window_query.get(window_root.primary));
    let viewport_size = window.resolution.size();

    for (font_size, node, mut text) in &mut text_query {
        // Compute font size.
        let size = c!(font_size.size.resolve(node.size().x, viewport_size));

        // Round font size to nearest multiple of step.
        let resolved = if font_size.step > 0.0 {
            (size / font_size.step).floor() * font_size.step
        } else {
            size
        };
        // Clamp font size above minimum.
        let size = resolved.max(font_size.minimum);

        for section in &mut text.sections {
            section.style.font_size = size;
        }
    }
}

/// Parses a "rich text" string with tags `"[r]"`, `"[b]"`, and `"[t]"`.
pub fn parse_rich(text: impl AsRef<str>) -> Vec<TextSection> {
    let styles = HashMap::from([
        (
            "r",
            TextStyle {
                font: FONT_HANDLE,
                ..default()
            },
        ),
        (
            "b",
            TextStyle {
                font: BOLD_FONT_HANDLE,
                ..default()
            },
        ),
        (
            "t",
            TextStyle {
                font: THICK_FONT_HANDLE,
                ..default()
            },
        ),
    ]);

    parse_rich_custom(text, &styles, "r")
}

/// Parses a "rich text" string.
///
/// Format:
/// - The text style will be set to `styles[start_tag]` initially.
/// - `"[tag]"` will set the text style to `styles["tag"]` for the following text.
/// - If `styles["tag"]` is not found, `"[tag]"` will be interpreted as literal text.
/// - Tags cannot be escaped. To allow literal `"[tag]"`, don't use `"tag"` as a key.
pub fn parse_rich_custom(
    text: impl AsRef<str>,
    styles: &HashMap<&str, TextStyle>,
    start_tag: &str,
) -> Vec<TextSection> {
    let text = text.as_ref();
    let mut sections = vec![];

    let mut lo = 0;
    let mut style = &styles[start_tag];
    let mut section = TextSection::new("", style.clone());

    let mut push_str = |s: &str, style: &TextStyle| {
        if s.is_empty() {
            return;
        }

        // If the new text uses a different style, create a new section for it.
        if section.style.font != style.font
            || section.style.font_size != style.font_size
            || section.style.color != style.color
        {
            let mut old_section = TextSection::new("", style.clone());
            std::mem::swap(&mut old_section, &mut section);
            if !old_section.value.is_empty() {
                sections.push(old_section);
            }
        }
        section.value.push_str(s);
    };

    for tag in regex!(r"\[((?:\w|\d|-)+)\]").captures_iter(text) {
        // Skip invalid tags to include them as literal text instead.
        let next_style = c!(styles.get(&tag[1]));

        let delim = tag.get(0).unwrap();
        push_str(&text[lo..delim.start()], style);
        lo = delim.end();
        style = next_style;
    }
    push_str(&text[lo..text.len()], style);
    if !section.value.is_empty() {
        sections.push(section);
    }

    sections
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_styles() -> HashMap<&'static str, TextStyle> {
        let r = TextStyle {
            font: FONT_HANDLE,
            ..default()
        };
        let b = TextStyle {
            font: BOLD_FONT_HANDLE,
            ..default()
        };
        HashMap::from([("regular", r.clone()), ("bold", b.clone())])
    }

    #[test]
    #[should_panic]
    fn test_invalid_start_tag() {
        let _ = parse_rich_custom("hello", &get_styles(), "invalid");
    }

    #[test]
    fn test_text() {
        let styles = get_styles();
        let r = &styles["regular"].clone();
        let b = &styles["bold"].clone();
        for (case, want) in [
            ("", vec![]),
            ("[bold]", vec![]),
            ("[bold", vec![TextSection::new("[bold", r.clone())]),
            ("bold]", vec![TextSection::new("bold]", r.clone())]),
            ("[[bold]", vec![TextSection::new("[", r.clone())]),
            ("[bold]]", vec![TextSection::new("]", b.clone())]),
            (
                "[[bold]]",
                vec![
                    TextSection::new("[", r.clone()),
                    TextSection::new("]", b.clone()),
                ],
            ),
            ("[invalid]", vec![TextSection::new("[invalid]", r.clone())]),
            ("[][][]", vec![TextSection::new("[][][]", r.clone())]),
            ("hello [bold]", vec![TextSection::new("hello ", r.clone())]),
            ("[bold] hello", vec![TextSection::new(" hello", b.clone())]),
            (
                "[bold][bold] hello",
                vec![TextSection::new(" hello", b.clone())],
            ),
            (
                "hello [bold] world",
                vec![
                    TextSection::new("hello ", r.clone()),
                    TextSection::new(" world", b.clone()),
                ],
            ),
            (
                "hello [invalid] world",
                vec![TextSection::new("hello [invalid] world", r.clone())],
            ),
            (
                "hello [] world",
                vec![TextSection::new("hello [] world", r.clone())],
            ),
            (
                "hello [[bold]] world",
                vec![
                    TextSection::new("hello [", r.clone()),
                    TextSection::new("] world", b.clone()),
                ],
            ),
            (
                "hello \\[bold] world",
                vec![
                    TextSection::new("hello \\", r.clone()),
                    TextSection::new(" world", b.clone()),
                ],
            ),
            (
                "hello [regular] world",
                vec![TextSection::new("hello  world", r.clone())],
            ),
            (
                "hello [regular] w[regular][regular]orld",
                vec![TextSection::new("hello  world", r.clone())],
            ),
            (
                "hello [regular][bold] world",
                vec![
                    TextSection::new("hello ", r.clone()),
                    TextSection::new(" world", b.clone()),
                ],
            ),
            (
                "hello [bold][regular] world",
                vec![TextSection::new("hello  world", r.clone())],
            ),
        ] {
            let got = parse_rich_custom(case, &styles, "regular");
            assert_eq!(got.len(), want.len());
            for (got, want) in got.iter().zip(&want) {
                assert_eq!(got.value, want.value);
                assert_eq!(got.style.font, want.style.font);
                assert_eq!(got.style.font_size, want.style.font_size);
                assert_eq!(got.style.color, want.style.color);
            }
        }
    }
}
