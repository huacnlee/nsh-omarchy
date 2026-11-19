//! Display: surfaces, badges, keycaps, avatars, icons, type, Markdown, charts.

use nsh_core::element::IntoProp;
use nsh_core::prelude::*;

use crate::IconName;
use crate::ui::display::{chips, markdown as md, surfaces, typography};
use crate::ui::{charts, icon as glyph};

pub use crate::ui::charts::{
    AreaChart, AreaChartElement, BarChart, BarChartElement, ChartCurve, ChartPoint, LineChart,
    LineChartElement, PieChart, PieChartElement, SeriesData,
};
pub use crate::ui::display::chips::{
    Avatar, AvatarElement, Badge, BadgeElement, BadgeVariant, Keycap, KeycapElement, Shortcut,
    ShortcutElement,
};
pub use crate::ui::display::markdown::{Markdown, MarkdownElement};
pub use crate::ui::display::surfaces::{
    Card, CardElement, EmptyState, EmptyStateElement, Panel, PanelElement, PanelVariant,
    Separator, SeparatorElement,
};
pub use crate::ui::display::typography::{
    Body, BodyElement, Caption, CaptionElement, Code, CodeElement, Heading, HeadingElement, Title,
    TitleElement,
};
pub use crate::ui::icon::{Icon, IconElement};

// ---- Surfaces -------------------------------------------------------------

/// A raised surface; give it a `title`, a `description`, a `footer` and
/// its body as children: `card().title("Updates").child(..)`.
pub fn card() -> CardElement {
    surfaces::card()
}

/// A titled section on a square surface: `panel("Workspace").child(..)`,
/// `.outline()` for the border only.
pub fn panel<M>(title: impl IntoProp<SharedString, M>) -> PanelElement {
    surfaces::panel().title(title)
}

impl PanelElement {
    /// The raised surface color inside its border:
    /// `.variant(PanelVariant::Fill)`.
    pub fn fill(self) -> Self {
        self.variant(PanelVariant::Fill)
    }
    /// Only the border: `.variant(PanelVariant::Outline)`.
    pub fn outline(self) -> Self {
        self.variant(PanelVariant::Outline)
    }
}

/// A 1pt divider across its container; `.vertical()` runs it down a row.
pub fn separator() -> SeparatorElement {
    surfaces::separator()
}

impl SeparatorElement {
    /// Down a row, as tall as the row: `.layout(Axis::Vertical)`.
    pub fn vertical(self) -> Self {
        self.layout(crate::Axis::Vertical)
    }
}

/// What a view shows while it has nothing: `empty_state("No projects")
/// .description("Create one").icon(IconName::Folder).child(action)`.
pub fn empty_state<M>(title: impl IntoProp<SharedString, M>) -> EmptyStateElement {
    surfaces::empty_state().title(title)
}

// ---- Badge, Keycap, Shortcut, Avatar --------------------------------------

/// A short status or count: `badge("Online").success()`, `.outline()`.
pub fn badge<M>(label: impl IntoProp<SharedString, M>) -> BadgeElement {
    chips::badge().label(label)
}

impl BadgeElement {
    /// The accent: `.variant(BadgeVariant::Primary)`.
    pub fn primary(self) -> Self {
        self.variant(BadgeVariant::Primary)
    }
    /// Done, passing: `.variant(BadgeVariant::Success)`.
    pub fn success(self) -> Self {
        self.variant(BadgeVariant::Success)
    }
    /// Needs a look: `.variant(BadgeVariant::Warning)`.
    pub fn warning(self) -> Self {
        self.variant(BadgeVariant::Warning)
    }
    /// Failed: `.variant(BadgeVariant::Danger)`.
    pub fn danger(self) -> Self {
        self.variant(BadgeVariant::Danger)
    }
}

/// One key, as Omarchy writes it: `keycap("mod")`, `keycap("esc")`.
pub fn keycap<M>(key: impl IntoProp<SharedString, M>) -> KeycapElement {
    chips::keycap().key(key)
}

/// A chord, a keycap per key: `shortcut("mod-shift-p")`.
pub fn shortcut<M>(keys: impl IntoProp<SharedString, M>) -> ShortcutElement {
    chips::shortcut().keys(keys)
}

/// A square avatar with the person's initials, or `.src(..)`'s picture:
/// `avatar("Ada Lovelace")`.
pub fn avatar<M>(name: impl IntoProp<SharedString, M>) -> AvatarElement {
    chips::avatar().name(name)
}

/// `keys` (`"mod-s"`, any spelling) as this platform shows them: `cmd-s`
/// on macOS, `ctrl-s` elsewhere.
pub fn shortcut_label(keys: &str) -> SharedString {
    crate::components::shortcut::shortcut_label(keys)
}

// ---- Icon -----------------------------------------------------------------

/// A Lucide glyph in the inherited text color, decorative unless given a
/// `label`: `icon(IconName::Lock).label("Encrypted")`.
pub fn icon(name: IconName) -> IconElement {
    glyph::icon().name(name)
}

// ---- Type -----------------------------------------------------------------

/// A page's heading: 20pt, bright.
pub fn heading(content: impl IntoReactive<SharedString>) -> HeadingElement {
    typography::heading().child(text(content))
}

/// A section's title: 14pt, bright.
pub fn title(content: impl IntoReactive<SharedString>) -> TitleElement {
    typography::title().child(text(content))
}

/// Body text: 12pt.
pub fn body(content: impl IntoReactive<SharedString>) -> BodyElement {
    typography::body().child(text(content))
}

/// A caption or hint: 11pt, secondary.
pub fn caption(content: impl IntoReactive<SharedString>) -> CaptionElement {
    typography::caption().child(text(content))
}

/// Inline code: monospace on the inset surface.
pub fn code(content: impl IntoReactive<SharedString>) -> CodeElement {
    typography::code().child(text(content))
}

/// A Markdown document in Omarchy's type: `markdown(notes)
/// .on_link_press(move |url: &String| open(url))`.
pub fn markdown<M>(source: impl IntoProp<SharedString, M>) -> MarkdownElement {
    md::markdown().source(source)
}

// ---- Charts ---------------------------------------------------------------

/// A line per series: `line_chart(points).name("Desktop")`.
pub fn line_chart<M>(data: impl IntoProp<Vec<ChartPoint>, M>) -> LineChartElement {
    charts::line_chart().data(data)
}

/// A tinted area per series: `area_chart(points).stacked(true)`.
pub fn area_chart<M>(data: impl IntoProp<Vec<ChartPoint>, M>) -> AreaChartElement {
    charts::area_chart().data(data)
}

/// Square bars per label: `bar_chart(points).series(more)`.
pub fn bar_chart<M>(data: impl IntoProp<Vec<ChartPoint>, M>) -> BarChartElement {
    charts::bar_chart().data(data)
}

/// Slices of a whole: `pie_chart(points).donut(0.6)`.
pub fn pie_chart<M>(data: impl IntoProp<Vec<ChartPoint>, M>) -> PieChartElement {
    charts::pie_chart().data(data)
}
