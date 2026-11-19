//! The fields through the public API, headless: typing, stepping, codes,
//! choosing, filtering, picking days and colors, labels and errors. Today
//! is 2026-10-28 (the engine's manual clock).

use std::time::{Duration, SystemTime};

use nsh_core::platform::Os;
use nsh_core::prelude::*;
use nsh_core::time::Clock;
use nsh_engine::{EngineConfig, Root, Size};
use nsh_omarchy::{self as om, Date, DateRange, SelectItem};
use nsh_testing::Headless;

const WAIT: Duration = Duration::from_secs(5);

fn start(root: Root) -> Headless {
    let mut config = EngineConfig {
        viewport: Size::new(px(1200.0), px(800.0)),
        ..EngineConfig::new(root)
    };
    config.environment.os = Os::MacOs;
    // Noon UTC, 2026-10-28.
    config.clock = Clock::manual(SystemTime::UNIX_EPOCH + Duration::from_secs(1_793_188_800));
    let mut app = Headless::start(config).expect("the engine starts");
    app.first_render().unwrap();
    app
}

// ---- Input, textarea ----------------------------------------------------

#[component]
fn InputHarness() -> impl IntoElement {
    let name = signal(String::new());
    let submitted = signal(String::new());
    view()
        .p(px(20.0))
        .gap(px(8.0))
        .child(
            om::field("Hostname").description("Return submits.").child(
                om::input()
                    .value(move || name.get())
                    .placeholder("omarchy")
                    // Named by the field's label.
                    .on_change(move |text: &String| name.set(text.clone()))
                    .on_submit(move |text: &String| submitted.set(text.clone())),
            ),
        )
        // Left out, the value is the field's own.
        .child(om::input().accessibility_label("Scratch"))
        // The app's value wins over typing it does not write back.
        .child(om::input().value("fixed").accessibility_label("Fixed"))
        .child(om::input().masked(true).small().accessibility_label("Password"))
        .child(om::input().value("~/.config").disabled(true).accessibility_label("Config"))
        .child(
            om::textarea()
                .rows(3)
                .accessibility_label("Notes")
                .on_change(move |text: &String| name.set(format!("notes {}", text.lines().count()))),
        )
        .child(text(move || format!("name: {:?}", name.get())))
        .child(text(move || format!("submitted: {:?}", submitted.get())))
}

#[test]
fn an_input_types_submits_and_keeps_its_own_value() -> Result<(), String> {
    let mut app = start(Root::of::<InputHarness>());
    app.wait_for("text:Return submits.")?;
    app.type_text("label:Hostname", "arch")?;
    app.wait_for("text:name: \"arch\"")?;
    app.submit("label:Hostname")?;
    app.wait_for("text:submitted: \"arch\"")?;

    app.type_text("label:Scratch", "kept")?;
    assert_eq!(app.screen().input_text("label:Scratch")?.as_deref(), Some("kept"));

    assert!(!app.screen().enabled("label:Config")?);

    app.type_text("label:Notes", "one\ntwo")?;
    app.wait_for("text:name: \"notes 2\"")?;
    Ok(())
}

// ---- Number input -------------------------------------------------------

#[component]
fn NumberHarness() -> impl IntoElement {
    let gaps = signal(Some(4.0_f64));
    view()
        .p(px(20.0))
        .child(
            om::number_input()
                .value(move || gaps.get())
                .min(0.0)
                .max(20.0)
                .suffix("px")
                .accessibility_label("Window gaps")
                .on_change(move |v: &Option<f64>| gaps.set(*v)),
        )
        .child(om::number_input().step(0.5).accessibility_label("Own"))
        .child(text(move || match gaps.get() {
            Some(v) => format!("value: {v}"),
            None => "value: none".into(),
        }))
}

#[test]
fn a_number_input_steps_by_keys_and_buttons_within_its_bounds() -> Result<(), String> {
    let mut app = start(Root::of::<NumberHarness>());
    app.wait_for("text:value: 4")?;
    app.wait_for("text:px")?;
    app.type_text("label:Window gaps", "")?;
    app.press_key("up")?;
    app.wait_for("text:value: 5")?;
    app.press_key("pageup")?;
    app.press_key("pageup")?;
    app.wait_for("text:value: 20")?;
    app.press("label:Decrease >> nth=0")?;
    app.wait_for("text:value: 19")?;

    // Left out, the field keeps its own number.
    app.press("label:Increase >> nth=1")?;
    app.press("label:Increase >> nth=1")?;
    let own = app.screen().input_text("label:Own")?;
    // Shown to the step's precision.
    assert_eq!(own.as_deref(), Some("1.0"));
    Ok(())
}

// ---- One-time code ------------------------------------------------------

#[component]
fn OtpHarness() -> impl IntoElement {
    let code = signal(String::new());
    let done = signal(String::new());
    view()
        .p(px(20.0))
        .child(
            om::otp_input(6)
                .groups(2)
                .value(move || code.get())
                .accessibility_label("Verification code")
                .on_change(move |v: &String| code.set(v.clone()))
                .on_complete(move |v: &String| done.set(v.clone())),
        )
        .child(om::otp_input(4).masked(true).invalid(true).accessibility_label("PIN"))
        .child(text(move || format!("code: {:?} done: {:?}", code.get(), done.get())))
}

#[test]
fn an_otp_input_takes_digits_and_completes() -> Result<(), String> {
    let mut app = start(Root::of::<OtpHarness>());
    app.type_text("label:Verification code", "12a3")?;
    app.wait_for("text:code: \"123\" done: \"\"")?;
    app.type_text("label:Verification code", "456")?;
    app.wait_for("text:code: \"123456\" done: \"123456\"")?;
    Ok(())
}

// ---- Select, combobox ---------------------------------------------------

fn layouts() -> Vec<SelectItem> {
    vec![
        SelectItem::new("dwindle", "Dwindle").group("Tiling"),
        SelectItem::new("master", "Master").group("Tiling"),
        SelectItem::new("floating", "Floating").group("Other").disabled(true),
        SelectItem::new("monocle", "Monocle").group("Other"),
    ]
}

#[component]
fn SelectHarness() -> impl IntoElement {
    let layout = signal(Some("dwindle".to_owned()));
    view()
        .p(px(20.0))
        .gap(px(8.0))
        .child(
            om::select(layouts())
                .value(move || layout.get())
                .accessibility_label("Layout")
                .on_change(move |v: &String| layout.set(Some(v.clone()))),
        )
        .child(
            om::select(layouts())
                .placeholder("Pick one")
                .searchable(true)
                .large()
                .accessibility_label("Own layout"),
        )
        .child(text(move || format!("value: {}", layout.get().unwrap_or_default())))
}

#[test]
fn a_select_chooses_by_press_and_by_keys() -> Result<(), String> {
    let mut app = start(Root::of::<SelectHarness>());
    app.wait_for("text:value: dwindle")?;
    app.wait_for("text:Dwindle")?;
    app.press("role:combobox[Layout]")?;
    app.wait_for("role:listbox")?;
    assert!(!app.screen().enabled("role:option[Floating]")?);
    app.press("role:option[Master]")?;
    app.wait_for("text:value: master")?;
    app.wait_for_gone("role:listbox", WAIT)?;

    // The list opens on the chosen row; Down skips the disabled Floating.
    app.press("role:combobox[Layout]")?;
    app.wait_for("role:listbox")?;
    app.press_key("down")?;
    app.press_key("enter")?;
    app.wait_for("text:value: monocle")?;
    Ok(())
}

#[test]
fn a_select_left_alone_keeps_its_choice() -> Result<(), String> {
    let mut app = start(Root::of::<SelectHarness>());
    app.wait_for("text:Pick one")?;
    app.press("role:combobox[Own layout]")?;
    app.wait_for("role:listbox")?;
    app.press("role:option[Monocle]")?;
    app.wait_for_gone("role:listbox", WAIT)?;
    app.wait_for("text:Monocle")?;
    assert!(!app.screen().shows("text:Pick one"));
    Ok(())
}

fn apps() -> Vec<SelectItem> {
    ["Alacritty", "Chromium", "Neovim", "Obsidian", "Spotify"]
        .into_iter()
        .map(|name| SelectItem::new(name.to_lowercase(), name))
        .collect()
}

#[component]
fn ComboboxHarness() -> impl IntoElement {
    let pinned = signal(vec!["neovim".to_owned()]);
    view()
        .p(px(20.0))
        .child(
            om::combobox(apps())
                .value(move || pinned.get())
                .multiple(true)
                .placeholder("Search apps")
                .accessibility_label("Pinned apps")
                .on_change(move |v: &Vec<String>| pinned.set(v.clone())),
        )
        .child(text(move || format!("pinned: {}", pinned.get().join(", "))))
}

#[test]
fn a_combobox_filters_and_adds() -> Result<(), String> {
    let mut app = start(Root::of::<ComboboxHarness>());
    app.wait_for("text:pinned: neovim")?;
    app.type_text("label:Pinned apps", "spo")?;
    app.wait_for("role:option[Spotify]")?;
    assert!(!app.screen().matches("role:option[Chromium]")?);
    app.press("role:option[Spotify]")?;
    app.wait_for("text:pinned: neovim, spotify")?;
    Ok(())
}

// ---- Calendar, date picker ----------------------------------------------

#[component]
fn CalendarHarness() -> impl IntoElement {
    let picked = signal(None::<Date>);
    let span = signal(None::<DateRange>);
    view()
        .p(px(20.0))
        .gap(px(8.0))
        .child(
            om::calendar()
                .value(move || picked.get())
                .max(Date::new(2026, 11, 20))
                .on_change(move |d: &Date| picked.set(Some(*d))),
        )
        .child(text(move || {
            format!("picked: {}", picked.get().map(|d| d.to_string()).unwrap_or_default())
        }))
        .child(
            om::calendar()
                .range(true)
                .on_range_change(move |r: &DateRange| span.set(Some(*r))),
        )
        .child(text(move || match span.get() {
            Some(r) => format!("range: {} to {}", r.start, r.end),
            None => "range: none".into(),
        }))
}

#[test]
fn a_calendar_picks_a_day_and_a_range() -> Result<(), String> {
    let mut app = start(Root::of::<CalendarHarness>());
    app.wait_for("text:October >> nth=0")?;
    app.press("label:2026-10-14 >> nth=0")?;
    app.wait_for("text:picked: 2026-10-14")?;
    app.wait_until("the day selected", WAIT, |s| {
        s.selected("label:2026-10-14 >> nth=0") == Ok(true)
    })?;

    // After `max` nothing can be picked.
    app.press("label:Next month >> nth=0")?;
    app.wait_for("text:November >> nth=0")?;
    assert!(!app.screen().enabled("label:2026-11-24 >> nth=0")?);
    app.press("label:2026-11-03 >> nth=0")?;
    app.wait_for("text:picked: 2026-11-03")?;

    // A range: the first press starts it, the second ends it.
    app.press("label:2026-10-05")?;
    app.press("label:2026-10-09")?;
    app.wait_for("text:range: 2026-10-05 to 2026-10-09")?;
    Ok(())
}

#[component]
fn DatePickerHarness() -> impl IntoElement {
    let due = signal(None::<Date>);
    view()
        .p(px(20.0))
        .gap(px(8.0))
        .child(
            om::date_picker()
                .value(move || due.get())
                .accessibility_label("Due date")
                .on_change(move |d: &Date| due.set(Some(*d))),
        )
        .child(om::date_picker().value(Date::new(2026, 11, 19)).small().accessibility_label("Fixed"))
        .child(text(move || {
            format!("due: {}", due.get().map(|d| d.to_string()).unwrap_or_default())
        }))
}

#[test]
fn a_date_picker_opens_picks_and_closes() -> Result<(), String> {
    let mut app = start(Root::of::<DatePickerHarness>());
    app.wait_for("text:Choose a date")?;
    // The trigger formats its value.
    app.wait_for("text:Nov 19, 2026")?;
    assert!(!app.screen().shows("label:Next month"), "closed at first");
    app.press("role:combobox[Due date]")?;
    app.wait_for("label:2026-10-21")?;
    app.press("label:2026-10-21")?;
    app.wait_for("text:due: 2026-10-21")?;
    app.wait_for_gone("label:2026-10-21", WAIT)?;
    app.wait_for("text:Oct 21, 2026")?;
    app.press_key("enter")?;
    app.wait_for("label:Next month")?;
    app.press_key("escape")?;
    app.wait_for_gone("label:Next month", WAIT)?;
    Ok(())
}

// ---- Color picker -------------------------------------------------------

#[component]
fn ColorHarness() -> impl IntoElement {
    let picked = signal(om::Hsla::from(nsh_core::color::rgb(0x7aa2f7)));
    view()
        .p(px(20.0))
        .child(
            om::color_picker()
                .value(move || picked.get())
                .alpha(true)
                .swatches(vec![nsh_core::color::rgb(0xf7768e).into()])
                .accessibility_label("Accent")
                .on_change(move |c: &om::Hsla| picked.set(*c)),
        )
        .child(text(move || format!("color: {}", picked.get().to_hex())))
}

#[test]
fn a_color_picker_takes_a_typed_color() -> Result<(), String> {
    let mut app = start(Root::of::<ColorHarness>());
    app.wait_for("label:Hue")?;
    app.wait_for("label:Opacity")?;
    app.clear("label:Hex color")?;
    app.type_text("label:Hex color", "#ff0000")?;
    app.submit("label:Hex color")?;
    app.wait_for("text:color: #FF0000")?;
    Ok(())
}

// ---- Field --------------------------------------------------------------

#[component]
fn FieldHarness() -> impl IntoElement {
    let short = signal(true);
    view()
        .p(px(20.0))
        .child(
            om::field("Password")
                .required(true)
                .description("At least 8 characters.")
                .error(move || short.get().then(|| SharedString::from("Too short.")))
                .child(
                    om::input()
                        .masked(true)
                        .invalid(move || short.get())
                        .accessibility_label("Password")
                        .on_change(move |t: &String| short.set(t.chars().count() < 8)),
                ),
        )
}

#[test]
fn a_field_shows_its_error_in_place_of_its_description() -> Result<(), String> {
    let mut app = start(Root::of::<FieldHarness>());
    app.wait_for("text:Too short.")?;
    assert!(!app.screen().shows("text:At least 8 characters."));
    app.type_text("label:Password", "correct horse")?;
    app.wait_for("text:At least 8 characters.")?;
    assert!(!app.screen().shows("text:Too short."));
    Ok(())
}

// ---- Shared types -------------------------------------------------------

#[test]
fn a_date_is_a_day_written_in_iso() {
    let day = Date::new(2026, 11, 19);
    assert_eq!((day.year(), day.month(), day.day()), (2026, 11, 19));
    assert_eq!(day.to_string(), "2026-11-19");
    assert_eq!(day.label(), "Nov 19, 2026");
    assert_eq!("2026-11-19".parse::<Date>(), Ok(day));
    assert!("19/11/2026".parse::<Date>().is_err());
    assert_eq!(Date::from_ymd(2026, 2, 30), None);
    assert!(Date::new(2026, 11, 18) < day);
    let span = DateRange { start: Date::new(2026, 11, 19), end: Date::new(2026, 11, 22) };
    assert_eq!(span, DateRange::new(day, Date::new(2026, 11, 22)));
}

#[test]
fn select_items_build_up() {
    let item = SelectItem::new("dwindle", "Dwindle").group("Tiling").disabled(true);
    assert_eq!(item.value, "dwindle");
    assert_eq!(item.label, "Dwindle");
    assert_eq!(item.group.as_deref(), Some("Tiling"));
    assert!(item.disabled);
}

#[test]
fn the_fields_are_declared_for_tsx() {
    let ts = nsh_core::element::declarations(&nsh_omarchy::registry(), &[]);
    for name in [
        "Input", "Textarea", "NumberInput", "OtpInput", "Select", "Combobox", "Calendar",
        "DatePicker", "ColorPicker", "Field",
    ] {
        let line = format!("export declare const {name}: ((props: {name}Props) => JSX.Element);");
        assert!(ts.contains(&line), "missing {line:?}");
    }
    for line in [
        "  length: number;",
        "  items: (SelectItem)[];",
        "  label: string;",
        "  group?: string | null;",
        "  start: string;",
        "  end: string;",
        "  rangeValue?: DateRange | null;",
        "  onRangeChange?: (event: { value: DateRange }) => void;",
        "  onComplete?: (event: { value: string }) => void;",
        "  onSubmit?: (event: { value: string }) => void;",
        "  swatches?: (string)[];",
        "  value?: string | null;",
    ] {
        assert!(ts.contains(line), "missing {line:?} in\n{ts}");
    }
}
