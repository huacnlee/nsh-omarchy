//! What TSX imports from `nsh-omarchy` in a Rust + TSX app: the [`Omarchy`]
//! theme module and the components only Rust draws well (charts,
//! [`Markdown`](crate::components::text_view::Markdown), [`Icon`]).
//!
//! Everyday controls (buttons, fields, dialogs) are the TSX library's, in
//! `tsx/`, over `nsh/base` and `themes/omarchy.json`; these exports sit beside
//! it and draw with the same tokens.

pub mod button;
pub mod charts;
pub mod choices;
pub mod display;
pub mod fields;
pub mod icon;
pub mod menu;
pub mod omarchy;
pub mod overlays;

pub use button::{Button, ButtonEvent, IconButton};
pub use charts::{AreaChart, BarChart, ChartCurve, ChartPoint, LineChart, PieChart, SeriesData};
pub use choices::{Checkbox, Progress, ProgressTone, Radio, RadioGroup, Slider, Switch};
pub use display::{
    Avatar, AvatarSizeProp, Badge, BadgeTone, Body, ButtonGroup, ButtonGroupEvent, Caption, Card,
    Code, EmptyState, GroupItem, Heading, Keycap, Orientation, Panel, PanelVariant, Separator,
    Shortcut, Title, Toggle, ToggleEvent, ToggleGroup, ToggleGroupEvent,
};
pub use fields::{
    Combobox, ComboboxEvent, Field, Input, InputApi, InputEvent, NumberInput, NumberInputEvent,
    Select, SelectEvent, SelectItem, Textarea,
};
pub use icon::Icon;
pub use menu::{ContextMenu, DropdownMenu, MenuItem, MenuLabel, MenuSeparator};
pub use overlays::{
    AlertDialog, AlertTone, ConfirmOptions, Dialog, DialogRole, HoverCard, Popover, Sheet,
    ToastAction, ToastContent, ToastKind, Toaster, Toasts, Tooltip,
};
pub use omarchy::{BuiltInTheme, Omarchy, ThemeAppearance, ThemeInfo};
