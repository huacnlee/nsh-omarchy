//! Omarchy's presentation language on NSH's `nsh_base` behavior.
//!
//! Call [`init`] in the root component's body, then build screens from the
//! constructors in [`components`]. Every color comes from [`tokens`], which
//! follow the active [`Theme`].

pub mod components;
pub mod gallery;
pub mod system_theme;
pub mod theme;
pub mod tokens;
pub mod tsx;

pub use components::*;
pub use system_theme::follow_system;
pub use theme::{ActiveTheme, Appearance, Theme, active_theme};

use nsh_core::prelude::*;

/// Applies the Omarchy system theme (Tokyo Night when unavailable) and
/// follows its changes. Call it once, in the root component's body.
pub fn init() {
    system_theme::follow_system();
}

/// What a TSX app imports from `nsh-omarchy` (installed with
/// `nsh add nsh-omarchy`): the `Omarchy` theme module and the components.
#[native_module("nsh-omarchy")]
fn native(registry: &mut Registry) {
    registry
        .module::<tsx::Omarchy>()
        .register::<tsx::LineChart>()
        .register::<tsx::AreaChart>()
        .register::<tsx::BarChart>()
        .register::<tsx::PieChart>()
        .register::<components::text_view::Markdown>()
        .register::<tsx::Icon>()
        .register::<tsx::Button>()
        .register::<tsx::IconButton>()
        .register::<tsx::DropdownMenu>()
        .register::<tsx::ContextMenu>()
        .register::<tsx::MenuItem>()
        .register::<tsx::MenuSeparator>()
        .register::<tsx::MenuLabel>()
        .register::<tsx::Input>()
        .register::<tsx::Textarea>()
        .register::<tsx::NumberInput>()
        .register::<tsx::Select>()
        .register::<tsx::Combobox>()
        .register::<tsx::Field>()
        .register::<tsx::Toggle>()
        .register::<tsx::ToggleGroup>()
        .register::<tsx::ButtonGroup>()
        .register::<tsx::Card>()
        .register::<tsx::Panel>()
        .register::<tsx::Separator>()
        .register::<tsx::Badge>()
        .register::<tsx::Keycap>()
        .register::<tsx::Shortcut>()
        .register::<tsx::EmptyState>()
        .register::<tsx::Avatar>()
        .register::<tsx::Heading>()
        .register::<tsx::Title>()
        .register::<tsx::Body>()
        .register::<tsx::Caption>()
        .register::<tsx::Code>()
        .register::<tsx::Checkbox>()
        .register::<tsx::Switch>()
        .register::<tsx::RadioGroup>()
        .register::<tsx::Radio>()
        .register::<tsx::Slider>()
        .register::<tsx::Progress>()
        .module::<tsx::Toasts>()
        .register::<tsx::Dialog>()
        .register::<tsx::AlertDialog>()
        .register::<tsx::Sheet>()
        .register::<tsx::Popover>()
        .register::<tsx::Tooltip>()
        .register::<tsx::HoverCard>()
        .register::<tsx::Toaster>();
}
