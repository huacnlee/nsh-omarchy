//! Square avatars on `nsh_base`'s `Avatar`: a picture, or the person's
//! initials while it loads, when it fails, and when there is none.

use nsh_base::AvatarElement;
use nsh_core::element::ImageSrc;
use nsh_core::prelude::*;

use crate::tokens;

/// The sizes an [`avatar`] comes in, matched to the control heights.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AvatarSize {
    /// 24pt, beside small controls and list rows.
    Small,
    /// 32pt.
    #[default]
    Medium,
    /// 48pt, in headers and profile cards.
    Large,
}

impl AvatarSize {
    fn side(self) -> f32 {
        match self {
            AvatarSize::Small => tokens::CONTROL_SM,
            AvatarSize::Medium => tokens::CONTROL_LG,
            AvatarSize::Large => tokens::CONTROL_LG * 1.5,
        }
    }

    fn text(self) -> f32 {
        match self {
            AvatarSize::Small => tokens::TEXT_XS,
            AvatarSize::Medium => tokens::TEXT_SM,
            AvatarSize::Large => tokens::TEXT_LG,
        }
    }
}

/// A square avatar for `name`: its initials on a faint fill inside a control
/// border ("Ada Lovelace" shows "AL"). Screen readers say the name. Give it
/// a picture with [`avatar_with_image`] or `.src(..)`.
pub fn avatar(name: impl Into<SharedString>, size: AvatarSize) -> AvatarElement {
    let side = size.side();
    let text_size = size.text();
    nsh_base::avatar()
        .name(Some(name.into()))
        .size(px(side))
        .flex_shrink_0()
        .overflow_hidden()
        .border_1()
        .border_color(move || tokens::control_border())
        .bg(move || tokens::hover_fill())
        .fallback_style(move |s| {
            s.flex_row()
                .items_center()
                .justify_center()
                .text_size(px(text_size))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || tokens::foreground())
        })
        .image_style(|s| s.object_fit(ResizeMode::Cover))
}

/// An [`avatar`] showing `src` once it loads, and the initials until then
/// or when it fails.
pub fn avatar_with_image(
    name: impl Into<SharedString>,
    src: impl Into<ImageSrc>,
    size: AvatarSize,
) -> AvatarElement {
    avatar(name, size).src(Some(src.into()))
}
