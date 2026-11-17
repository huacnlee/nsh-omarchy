//! What TSX imports from `nsh-omarchy` in a Rust + TSX app: the [`Omarchy`]
//! theme module and the components only Rust draws well (charts,
//! [`Markdown`](crate::components::text_view::Markdown), [`Icon`]).
//!
//! Everyday controls (buttons, fields, dialogs) are the TSX library's, in
//! `tsx/`, over `nsh/base` and `themes/omarchy.json`; these exports sit beside
//! it and draw with the same tokens.

pub mod charts;
pub mod icon;
pub mod omarchy;

pub use charts::{AreaChart, BarChart, ChartCurve, ChartPoint, LineChart, PieChart, SeriesData};
pub use icon::Icon;
pub use omarchy::{BuiltInTheme, Omarchy, ThemeAppearance, ThemeInfo};
