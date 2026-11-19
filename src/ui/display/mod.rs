//! Display: the surfaces (`Card`, `Panel`, `Separator`, `EmptyState`), the
//! chips (`Badge`, `Keycap`, `Shortcut`, `Avatar`), the type ramp
//! (`Heading` … `Code`) and `Markdown`, drawn with [`crate::components`]
//! and the Omarchy tokens. The icon and the charts live in
//! [`super::icon`] and [`super::charts`].
//!
//! Square, flat, one border; no shadows, no gradients. Every root is a
//! plain view or text (an app may put an avatar or a badge in a menu's or
//! popover's trigger slot, which reaches a component one level deep).

pub mod chips;
pub mod markdown;
pub mod surfaces;
pub mod typography;

use nsh_core::prelude::*;

/// Registers this area's components for TSX (with the icon and the charts).
pub(crate) fn register(registry: &mut Registry) {
    registry
        .register::<super::icon::Icon>()
        .register::<super::charts::LineChart>()
        .register::<super::charts::AreaChart>()
        .register::<super::charts::BarChart>()
        .register::<super::charts::PieChart>()
        .register::<markdown::Markdown>()
        .register::<surfaces::Card>()
        .register::<surfaces::Panel>()
        .register::<surfaces::Separator>()
        .register::<surfaces::EmptyState>()
        .register::<chips::Badge>()
        .register::<chips::Keycap>()
        .register::<chips::Shortcut>()
        .register::<chips::Avatar>()
        .register::<typography::Heading>()
        .register::<typography::Title>()
        .register::<typography::Body>()
        .register::<typography::Caption>()
        .register::<typography::Code>();
}
