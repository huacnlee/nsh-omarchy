//! Icons: a small set of Lucide glyphs (ISC license, <https://lucide.dev>)
//! drawn by NSH's `svg` element.
//!
//! Each glyph strokes with `currentColor`, which `svg` resolves to the
//! element's text color, so an icon inherits the color of the text around it
//! and follows the theme. Tint one with `.text_color(move || tokens::accent())`.
//!
//! Icons are decorative (hidden from screen readers) unless labeled with
//! [`labeled_icon`]: put the meaning in the text or the control beside them.

use nsh_core::element::{ImageSrc, Svg};
use nsh_core::prelude::*;

/// The default size of an icon: the line height of `TEXT_MD` text.
const ICON_SIZE: f32 = 16.0;

macro_rules! icons {
    ($( $variant:ident => $name:literal, $body:literal; )*) => {
        /// The name of an icon of the built-in set; in TSX its Lucide name
        /// (`"chevron-down"`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PropEnum)]
        pub enum IconName {
            $(
                #[doc = concat!("`", $name, "`")]
                #[prop(rename = $name)]
                $variant,
            )*
        }

        impl IconName {
            /// Every icon, in declaration order.
            pub const ALL: &'static [IconName] = &[$( IconName::$variant, )*];

            /// The Lucide name of the icon (`chevron-down`).
            pub fn name(self) -> &'static str {
                match self { $( IconName::$variant => $name, )* }
            }

            /// The SVG elements inside the 24x24 `<svg>` of the icon.
            fn body(self) -> &'static str {
                match self { $( IconName::$variant => $body, )* }
            }
        }
    };
}

icons! {
    Check => "check", r#"<path d="M20 6 9 17l-5-5"/>"#;
    X => "x", r#"<path d="M18 6 6 18"/><path d="m6 6 12 12"/>"#;
    ChevronDown => "chevron-down", r#"<path d="m6 9 6 6 6-6"/>"#;
    ChevronUp => "chevron-up", r#"<path d="m18 15-6-6-6 6"/>"#;
    ChevronLeft => "chevron-left", r#"<path d="m15 18-6-6 6-6"/>"#;
    ChevronRight => "chevron-right", r#"<path d="m9 18 6-6-6-6"/>"#;
    ArrowUp => "arrow-up", r#"<path d="m5 12 7-7 7 7"/><path d="M12 19V5"/>"#;
    ArrowDown => "arrow-down", r#"<path d="M12 5v14"/><path d="m19 12-7 7-7-7"/>"#;
    ArrowLeft => "arrow-left", r#"<path d="m12 19-7-7 7-7"/><path d="M19 12H5"/>"#;
    ArrowRight => "arrow-right", r#"<path d="M5 12h14"/><path d="m12 5 7 7-7 7"/>"#;
    Plus => "plus", r#"<path d="M5 12h14"/><path d="M12 5v14"/>"#;
    Minus => "minus", r#"<path d="M5 12h14"/>"#;
    Search => "search", r#"<circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>"#;
    Settings => "settings", r#"<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/>"#;
    Menu => "menu", r#"<path d="M4 12h16"/><path d="M4 6h16"/><path d="M4 18h16"/>"#;
    MoreHorizontal => "ellipsis", r#"<circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/>"#;
    MoreVertical => "ellipsis-vertical", r#"<circle cx="12" cy="12" r="1"/><circle cx="12" cy="5" r="1"/><circle cx="12" cy="19" r="1"/>"#;
    GripVertical => "grip-vertical", r#"<circle cx="9" cy="12" r="1"/><circle cx="9" cy="5" r="1"/><circle cx="9" cy="19" r="1"/><circle cx="15" cy="12" r="1"/><circle cx="15" cy="5" r="1"/><circle cx="15" cy="19" r="1"/>"#;
    Info => "info", r#"<circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/>"#;
    TriangleAlert => "triangle-alert", r#"<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="M12 17h.01"/>"#;
    CircleAlert => "circle-alert", r#"<circle cx="12" cy="12" r="10"/><path d="M12 8v4"/><path d="M12 16h.01"/>"#;
    CircleCheck => "circle-check", r#"<circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/>"#;
    CircleX => "circle-x", r#"<circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/>"#;
    Trash => "trash", r#"<path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/>"#;
    Copy => "copy", r#"<rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>"#;
    Pencil => "pencil", r#"<path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"/><path d="m15 5 4 4"/>"#;
    Calendar => "calendar", r#"<rect width="18" height="18" x="3" y="4" rx="2"/><path d="M16 2v4"/><path d="M8 2v4"/><path d="M3 10h18"/>"#;
    Folder => "folder", r#"<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>"#;
    File => "file", r#"<path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/>"#;
    Inbox => "inbox", r#"<path d="M22 12h-6l-2 3h-4l-2-3H2"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/>"#;
    User => "user", r#"<path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/>"#;
    Home => "house", r#"<path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><path d="M9 22V12h6v10"/>"#;
    Bell => "bell", r#"<path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9"/><path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"/>"#;
    Sun => "sun", r#"<circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/>"#;
    Moon => "moon", r#"<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/>"#;
    Eye => "eye", r#"<path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/>"#;
    Lock => "lock", r#"<rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/>"#;
    Link => "link", r#"<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>"#;
    ExternalLink => "external-link", r#"<path d="M15 3h6v6"/><path d="M10 14 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>"#;
    Download => "download", r#"<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m7 10 5 5 5-5"/><path d="M12 15V3"/>"#;
    Upload => "upload", r#"<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m17 8-5-5-5 5"/><path d="M12 3v12"/>"#;
    Filter => "funnel", r#"<path d="M22 3H2l8 9.46V19l4 2v-8.54L22 3z"/>"#;
    Terminal => "terminal", r#"<path d="m4 17 6-6-6-6"/><path d="M12 19h8"/>"#;
    Star => "star", r#"<path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>"#;
    Heart => "heart", r#"<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"/>"#;
    Loader => "loader-circle", r#"<path d="M21 12a9 9 0 1 1-6.219-8.56"/>"#;
}

impl IconName {
    /// The whole SVG document of the icon, stroked with `currentColor`.
    pub fn svg_markup(self) -> String {
        format!(
            concat!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" "#,
                r#"fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" "#,
                r#"stroke-linejoin="round">{}</svg>"#
            ),
            self.body()
        )
    }
}

/// A decorative icon, 16pt, in the inherited text color. Size it with
/// `.size(px(..))` and tint it with `.text_color(move || tokens::..())`.
pub fn icon(name: IconName) -> Svg {
    svg(ImageSrc::svg_markup(&name.svg_markup()))
        .size(px(ICON_SIZE))
        .flex_shrink_0()
        .accessibility_elements_hidden(true)
}

/// An icon that stands alone and means something: screen readers say
/// `label` for it, as an image.
pub fn labeled_icon(name: IconName, label: impl Into<SharedString>) -> Svg {
    svg(ImageSrc::svg_markup(&name.svg_markup()))
        .size(px(ICON_SIZE))
        .flex_shrink_0()
        .accessible(true)
        .role(AccessibilityRole::Image)
        .label(label.into())
}
