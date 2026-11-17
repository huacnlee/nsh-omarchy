//! Omarchy charts: [`LineChart`], [`AreaChart`], [`BarChart`] and
//! [`PieChart`], drawn with `nsh_base::plot`.
//!
//! The look: series in `tokens::chart(i)`, grid lines in the divider color,
//! axis labels in secondary `TEXT_XS`, square bars, square hover dots, and a
//! bordered square tooltip that reads out the row under the pointer (or the
//! keyboard: a chart is a focus stop whose Left / Right step the rows).
//! Marks draw in over `DURATION_SLOW` and the hover fades over
//! `DURATION_FAST`; the plot drops both while the user asks for reduced
//! motion. Every chart carries an accessibility summary (the app's words or
//! one made from the data) and reads the hovered row as its value.
//!
//! ```ignore
//! line_chart([("Mon", 12.0), ("Tue", 18.0), ("Wed", 15.0)])
//!     .name("Visitors")
//!     .series("Signups", [3.0, 5.0, 4.0])
//!     .accessibility_label("Visitors and signups this week")
//! ```

use std::f32::consts::TAU;
use std::rc::Rc;

use nsh_base::plot::{
    AreaY0, AxisLabelStyle, AxisPlacement, BandScale, Curve, PlotAccessibilityValue, PlotMotion,
    Scale, TickFormat, TooltipPlacement, arc, area, bar, grid, line, pie, plot, plot_axis,
};
use nsh_core::element::{RowAccessor, Rows, Style, StyleRefinement, Timing};
use nsh_core::prelude::*;

use crate::tokens;

/// The room left of the plot for the y axis' labels.
const Y_AXIS_ROOM: f32 = 36.0;
/// The room under the plot for the x axis' labels.
const X_AXIS_ROOM: f32 = 18.0;
/// The side of a legend swatch and of a hover dot.
const SWATCH: f32 = 8.0;
/// The most x axis labels a chart writes.
const MAX_X_TICKS: usize = 8;
/// The opacity of the marks the pointer is not on.
const INACTIVE: f32 = 0.45;

/// One named run of values, a value per label of its chart.
#[derive(Debug, Clone, PartialEq)]
pub struct ChartSeries {
    pub name: SharedString,
    pub values: Vec<f64>,
}

/// How a number is written in the tooltip, the summary and (when set) the
/// y axis.
type Format = Rc<dyn Fn(f64) -> SharedString>;

fn default_format(value: f64) -> SharedString {
    if !value.is_finite() {
        "–".into()
    } else if value.fract() == 0.0 {
        format!("{value:.0}").into()
    } else {
        format!("{value:.1}").into()
    }
}

/// What the cartesian charts share: the labels along x, the series and
/// the frame (grid, axes, size, hover).
#[derive(Clone)]
struct Cartesian {
    kind: &'static str,
    labels: Vec<SharedString>,
    series: Vec<ChartSeries>,
    /// Columns drawn but not read out (an area's stacked tops and bases).
    extra: Vec<(SharedString, Vec<f64>)>,
    label: Option<SharedString>,
    height: f32,
    grid: bool,
    x_axis: bool,
    y_axis: bool,
    legend: Option<bool>,
    interactive: bool,
    y_domain: Option<[f64; 2]>,
    format: Option<Format>,
}

impl Cartesian {
    fn new<L: Into<SharedString>>(
        kind: &'static str,
        data: impl IntoIterator<Item = (L, f64)>,
    ) -> Self {
        let (labels, values): (Vec<SharedString>, Vec<f64>) =
            data.into_iter().map(|(l, v)| (l.into(), v)).unzip();
        Self {
            kind,
            labels,
            series: vec![ChartSeries {
                name: "Value".into(),
                values,
            }],
            extra: Vec::new(),
            label: None,
            height: 200.0,
            grid: true,
            x_axis: true,
            y_axis: true,
            legend: None,
            interactive: true,
            y_domain: None,
            format: None,
        }
    }

    fn rows(&self) -> Rows {
        let series = self
            .series
            .iter()
            .enumerate()
            .map(|(i, s)| (key(i), s.values.clone()));
        Rows::from_columns(self.labels.len(), series.chain(self.extra.iter().cloned()))
    }

    fn format(&self) -> Format {
        self.format
            .clone()
            .unwrap_or_else(|| Rc::new(default_format))
    }

    /// The finite values of every series.
    fn values(&self) -> impl Iterator<Item = f64> + '_ {
        self.series
            .iter()
            .flat_map(|s| s.values.iter().copied())
            .filter(|v| v.is_finite())
    }

    /// The words screen readers hear for the chart: the app's, or what it
    /// shows and its range.
    fn summary(&self) -> SharedString {
        if let Some(label) = &self.label {
            return label.clone();
        }
        let format = self.format();
        let names: Vec<&str> = self.series.iter().map(|s| &*s.name).collect();
        let (lo, hi) = self
            .values()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| {
                (lo.min(v), hi.max(v))
            });
        let range = if lo.is_finite() {
            format!(", from {} to {}", format(lo), format(hi))
        } else {
            String::new()
        };
        format!(
            "{} of {}, {} points{range}",
            self.kind,
            names.join(" and "),
            self.labels.len()
        )
        .into()
    }

    /// The row's words: its label and each series' value.
    fn row_text(&self, index: usize) -> String {
        let format = self.format();
        let label = self.labels.get(index).cloned().unwrap_or_default();
        let values: Vec<String> = self
            .series
            .iter()
            .map(|s| {
                let value = s.values.get(index).copied().unwrap_or(f64::NAN);
                format!("{} {}", s.name, format(value))
            })
            .collect();
        format!("{label}: {}", values.join(", "))
    }

    /// The plot, its grid and axes around `marks`, the tooltip, and the
    /// legend under it when there are several series. `band`: the x axis
    /// has a band per label (bars).
    fn frame(self, x_scale: Scale, band: bool, marks: Vec<AnyElement>) -> AnyElement {
        let hovered = signal(None::<u32>);
        let this = Rc::new(self);
        let len = this.labels.len();

        let y_scale = this.y_domain.map(|domain| Scale::Linear {
            domain: Some(domain),
            reverse: false,
        });
        let read = this.clone();
        let mut p = plot()
            .data(this.rows())
            .x_scale(Some(x_scale))
            .y_scale(y_scale)
            .interactive(this.interactive)
            .motion(Some(motion()))
            .appear_generation(Some(0_u32.into()))
            .accessibility_label(Some(this.summary()))
            .accessibility_value(move || {
                hovered.get().map(|i| PlotAccessibilityValue {
                    text: Some(read.row_text(i as usize).into()),
                    ..PlotAccessibilityValue::default()
                })
            })
            .h(px(this.height))
            .on_hover_change(move |index: &Option<u32>| hovered.set(*index));

        if this.interactive {
            p = p
                .cross_line_style(move |s| {
                    if band {
                        s.w_full().bg(move || tokens::hover_fill())
                    } else {
                        s.w(px(1.0)).bg(move || tokens::control_border())
                    }
                })
                .tooltip_placement(TooltipPlacement::Pointer)
                .tooltip_offset(Points(tokens::SPACE_2))
                .tooltip_content(tooltip(this.clone(), hovered));
        }

        if this.grid {
            p = p.child(
                grid()
                    .ticks(Some(4))
                    .stroke_color(move || tokens::divider())
                    .stroke_width(px(1.0)),
            );
        }
        p = p.children(marks);
        if this.x_axis && len > 0 {
            let labels = this.labels.clone();
            p = p.child(
                plot_axis()
                    .placement(AxisPlacement::Bottom)
                    .ticks(Some(len.min(MAX_X_TICKS) as u32))
                    .format(Some(TickFormat::from_fn(move |value, _| {
                        let i = value.round();
                        if i >= 0.0 {
                            labels.get(i as usize).cloned().unwrap_or_default()
                        } else {
                            "".into()
                        }
                    })))
                    .label_style(axis_label_style(false)),
            );
        }
        if this.y_axis {
            let mut axis = plot_axis()
                .placement(AxisPlacement::Left)
                .ticks(Some(4))
                .label_style(axis_label_style(true));
            if let Some(format) = this.format.clone() {
                axis = axis.format(Some(TickFormat::from_fn(move |value, _| format(value))));
            }
            p = p.child(axis);
        }

        let show_legend = this.legend.unwrap_or(this.series.len() > 1);
        let items = this
            .series
            .iter()
            .enumerate()
            .map(|(i, s)| (i, s.name.clone(), None))
            .collect();
        view()
            .gap(px(tokens::SPACE_2))
            .child(
                view()
                    .pl(px(if this.y_axis { Y_AXIS_ROOM } else { 0.0 }))
                    .pb(px(if this.x_axis { X_AXIS_ROOM } else { 0.0 }))
                    .pt(px(tokens::SPACE_1))
                    .pr(px(tokens::SPACE_1))
                    .child(p),
            )
            .child(show_legend.then(|| legend(items, false)))
            .into_element()
    }
}

/// The column of series `index`.
fn key(index: usize) -> SharedString {
    format!("s{index}").into()
}

fn motion() -> PlotMotion {
    PlotMotion {
        enter: Some(Timing::new(tokens::DURATION_FAST, Easing::EaseOut)),
        exit: Some(Timing::new(tokens::DURATION_FAST, Easing::EaseIn)),
        appear: Some(Timing::new(tokens::DURATION_SLOW, Easing::EaseOut)),
        update: Some(Timing::new(tokens::DURATION_BASE, Easing::EaseInOut)),
        ..PlotMotion::default()
    }
}

/// Axis labels: secondary `TEXT_XS`, a little off the edge.
fn axis_label_style(left: bool) -> AxisLabelStyle {
    let style = StyleRefinement::default()
        .text_color(move || tokens::secondary())
        .text_size(px(tokens::TEXT_XS));
    let style = if left {
        style.pr(px(tokens::SPACE_1))
    } else {
        style.pt(px(tokens::SPACE_1))
    };
    AxisLabelStyle::from(Style::from(style))
}

/// A square point of series `index`; the hovered one ringed in the
/// background color so it stands off the line.
fn point_style(index: usize, hover: bool) -> impl FnOnce(StyleRefinement) -> StyleRefinement {
    move |s| {
        let side = if hover { SWATCH } else { SWATCH - 2.0 };
        let s = s.w(px(side)).h(px(side)).bg(move || tokens::chart(index));
        if hover {
            s.border_1().border_color(move || tokens::background())
        } else {
            s
        }
    }
}

/// A square swatch in series color `index`.
fn swatch(index: usize) -> impl IntoElement {
    view()
        .w(px(SWATCH))
        .h(px(SWATCH))
        .bg(move || tokens::chart(index))
}

/// The tooltip box: popover fill, a 1px border, square.
fn tooltip_box() -> View {
    view()
        .gap(px(tokens::SPACE_1))
        .p(px(tokens::SPACE_2))
        .bg(move || tokens::popover())
        .border_1()
        .border_color(move || tokens::border())
        .text_size(px(tokens::TEXT_SM))
}

/// The tooltip's readout of the hovered row: its label and each series'
/// value.
fn tooltip(chart: Rc<Cartesian>, hovered: Signal<Option<u32>>) -> impl IntoElement {
    let format = chart.format();
    let labels = chart.labels.clone();
    let rows: Vec<AnyElement> = chart
        .series
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let values = s.values.clone();
            let format = format.clone();
            view()
                .flex_row()
                .items_center()
                .gap(px(tokens::SPACE_2))
                .child(swatch(i))
                .child(
                    text(s.name.clone())
                        .flex_1()
                        .text_color(move || tokens::secondary()),
                )
                .child(
                    text(move || {
                        hovered
                            .get()
                            .and_then(|r| values.get(r as usize).copied())
                            .map(|v| format(v))
                            .unwrap_or_default()
                    })
                    .font_family(tokens::font_mono())
                    .text_color(move || tokens::foreground()),
                )
                .into_element()
        })
        .collect();
    tooltip_box()
        .min_w(px(120.0))
        .child(
            text(move || {
                hovered
                    .get()
                    .and_then(|i| labels.get(i as usize).cloned())
                    .unwrap_or_default()
            })
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(move || tokens::bright()),
        )
        .children(rows)
}

/// Swatches and names (and a value each, when given): a wrapping row, or
/// a column.
fn legend(items: Vec<(usize, SharedString, Option<SharedString>)>, column: bool) -> AnyElement {
    let entries = items.into_iter().map(move |(i, name, value)| {
        view()
            .flex_row()
            .items_center()
            .gap(px(tokens::SPACE_2))
            .child(swatch(i))
            .child(
                text(name)
                    .min_w(px(if column { 88.0 } else { 0.0 }))
                    .text_size(px(tokens::TEXT_XS))
                    .text_color(move || tokens::secondary()),
            )
            .child(value.map(|v| {
                text(v)
                    .text_size(px(tokens::TEXT_XS))
                    .font_family(tokens::font_mono())
                    .text_color(move || tokens::foreground())
            }))
            .into_element()
    });
    let base = if column {
        view().gap(px(tokens::SPACE_1))
    } else {
        view().flex_row().flex_wrap().gap(px(tokens::SPACE_3))
    };
    base.children(entries.collect::<Vec<_>>()).into_element()
}

/// The builder methods the cartesian charts share (their first field is
/// the [`Cartesian`]).
macro_rules! cartesian_builders {
    () => {
        /// Names the first series (`"Value"` by default).
        pub fn name(mut self, name: impl Into<SharedString>) -> Self {
            if let Some(first) = self.0.series.first_mut() {
                first.name = name.into();
            }
            self
        }
        /// Adds a series: a value per label, in order.
        pub fn series(
            mut self,
            name: impl Into<SharedString>,
            values: impl IntoIterator<Item = f64>,
        ) -> Self {
            self.0.series.push(ChartSeries {
                name: name.into(),
                values: values.into_iter().collect(),
            });
            self
        }
        /// What screen readers say of the chart; by default a summary of
        /// its series and range.
        pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
            self.0.label = Some(label.into());
            self
        }
        /// The plot's height in points (200 by default).
        pub fn height(mut self, height: f32) -> Self {
            self.0.height = height;
            self
        }
        /// Horizontal grid lines (on by default).
        pub fn grid(mut self, grid: bool) -> Self {
            self.0.grid = grid;
            self
        }
        /// The labels along the bottom (on by default).
        pub fn x_axis(mut self, x_axis: bool) -> Self {
            self.0.x_axis = x_axis;
            self
        }
        /// The values along the left (on by default).
        pub fn y_axis(mut self, y_axis: bool) -> Self {
            self.0.y_axis = y_axis;
            self
        }
        /// The legend under the plot (shown with two series or more).
        pub fn legend(mut self, legend: bool) -> Self {
            self.0.legend = Some(legend);
            self
        }
        /// Follows the pointer and the keys with a tooltip (on by
        /// default); off draws the marks only, with no focus stop.
        pub fn interactive(mut self, interactive: bool) -> Self {
            self.0.interactive = interactive;
            self
        }
        /// The values at the bottom and the top of the y axis.
        pub fn y_domain(mut self, min: f64, max: f64) -> Self {
            self.0.y_domain = Some([min, max]);
            self
        }
        /// How values are written (the tooltip, the summary, the y axis).
        pub fn format(mut self, format: impl Fn(f64) -> SharedString + 'static) -> Self {
            self.0.format = Some(Rc::new(format));
            self
        }
    };
}

/// The curve builders of the line and area charts (their second field is
/// the [`Curve`]).
macro_rules! curve_builders {
    () => {
        /// Straight segments between points (the default).
        pub fn linear(mut self) -> Self {
            self.1 = Curve::Linear;
            self
        }
        /// A smooth curve through the points.
        pub fn natural(mut self) -> Self {
            self.1 = Curve::Natural;
            self
        }
        /// Steps: each value held until the next point.
        pub fn step_after(mut self) -> Self {
            self.1 = Curve::StepAfter;
            self
        }
    };
}

/// The domain from zero (or the lowest value below it) to the highest of
/// `values`: what bars and areas stand on.
fn from_zero(values: impl Iterator<Item = f64>) -> [f64; 2] {
    values
        .filter(|v| v.is_finite())
        .fold([0.0, 0.0], |[lo, hi], v| [lo.min(v), hi.max(v)])
}

// ---- Line -----------------------------------------------------------------

/// A line per series over labels along x. Build with [`line_chart`].
#[derive(Clone)]
pub struct LineChart(Cartesian, Curve, bool);

/// A line chart of `(label, value)` points; add series with
/// [`LineChart::series`].
pub fn line_chart<L: Into<SharedString>>(data: impl IntoIterator<Item = (L, f64)>) -> LineChart {
    LineChart(Cartesian::new("Line chart", data), Curve::Linear, false)
}

impl LineChart {
    cartesian_builders!();
    curve_builders!();

    /// A square dot on every point.
    pub fn dots(mut self, dots: bool) -> Self {
        self.2 = dots;
        self
    }
}

impl IntoElement for LineChart {
    fn into_element(self) -> AnyElement {
        let LineChart(chart, curve, dots) = self;
        let marks = (0..chart.series.len())
            .map(|i| {
                line()
                    .y(Some(RowAccessor::key(key(i))))
                    .curve(curve)
                    .dot(dots)
                    .dot_style(point_style(i, false))
                    .hover_dot_style(point_style(i, true))
                    .stroke_color(move || tokens::chart(i))
                    .stroke_width(px(2.0))
                    .into_element()
            })
            .collect();
        chart.frame(Scale::Point { reverse: false }, false, marks)
    }
}

// ---- Area -----------------------------------------------------------------

/// A tinted area per series under a line of its color. Build with
/// [`area_chart`].
#[derive(Clone)]
pub struct AreaChart(Cartesian, Curve, bool);

/// An area chart of `(label, value)` points; add series with
/// [`AreaChart::series`].
pub fn area_chart<L: Into<SharedString>>(data: impl IntoIterator<Item = (L, f64)>) -> AreaChart {
    AreaChart(Cartesian::new("Area chart", data), Curve::Linear, false)
}

impl AreaChart {
    cartesian_builders!();
    curve_builders!();

    /// Stacks the series, each on the ones before it.
    pub fn stacked(mut self, stacked: bool) -> Self {
        self.2 = stacked;
        self
    }
}

impl IntoElement for AreaChart {
    fn into_element(self) -> AnyElement {
        let AreaChart(mut chart, curve, stacked) = self;
        let len = chart.labels.len();
        let top = |i: usize| -> SharedString { format!("t{i}").into() };
        let base = |i: usize| -> SharedString { format!("b{i}").into() };
        // Each series drawn from its base (zero, or when stacked the top
        // of the one before) to its top; the tooltip reads raw values.
        let mut under = vec![0.0; len];
        for (i, s) in chart.series.iter().enumerate() {
            let tops: Vec<f64> = (0..len)
                .map(|r| under[r] + s.values.get(r).copied().unwrap_or(f64::NAN))
                .collect();
            chart.extra.push((base(i), under.clone()));
            chart.extra.push((top(i), tops.clone()));
            if stacked {
                under = tops
                    .iter()
                    .zip(&under)
                    .map(|(t, u)| if t.is_finite() { *t } else { *u })
                    .collect();
            }
        }
        if chart.y_domain.is_none() {
            let drawn = chart.extra.iter().flat_map(|(_, v)| v.iter().copied());
            chart.y_domain = Some(from_zero(drawn));
        }
        let marks = (0..chart.series.len())
            .flat_map(|i| {
                let fill = area()
                    .y1(Some(RowAccessor::key(top(i))))
                    .y0(Some(AreaY0::Row(RowAccessor::key(base(i)))))
                    .curve(curve)
                    .hover_dot_style(point_style(i, true))
                    .fill_color(move || tokens::chart(i).opacity(0.18))
                    .stroke_none()
                    .into_element();
                let edge = line()
                    .y(Some(RowAccessor::key(top(i))))
                    .curve(curve)
                    .hover_dot_style(|s| s.w(px(0.0)).h(px(0.0)))
                    .stroke_color(move || tokens::chart(i))
                    .stroke_width(px(1.5))
                    .into_element();
                [fill, edge]
            })
            .collect();
        chart.frame(Scale::Point { reverse: false }, false, marks)
    }
}

// ---- Bar ------------------------------------------------------------------

/// Square bars per label, side by side per series or stacked. Build with
/// [`bar_chart`].
#[derive(Clone)]
pub struct BarChart(Cartesian, bool);

/// A bar chart of `(label, value)` bars; add series with
/// [`BarChart::series`].
pub fn bar_chart<L: Into<SharedString>>(data: impl IntoIterator<Item = (L, f64)>) -> BarChart {
    BarChart(Cartesian::new("Bar chart", data), false)
}

impl BarChart {
    cartesian_builders!();

    /// Stacks the series in one bar per label.
    pub fn stacked(mut self, stacked: bool) -> Self {
        self.1 = stacked;
        self
    }
}

impl IntoElement for BarChart {
    fn into_element(self) -> AnyElement {
        let BarChart(mut chart, stacked) = self;
        if chart.y_domain.is_none() {
            let len = chart.labels.len();
            chart.y_domain = Some(if stacked {
                let sums = (0..len).map(|r| {
                    chart
                        .series
                        .iter()
                        .filter_map(|s| s.values.get(r).copied())
                        .filter(|v| v.is_finite())
                        .sum()
                });
                from_zero(sums)
            } else {
                from_zero(chart.values())
            });
        }
        let mut band = BandScale::default();
        band.padding_inner = Percentage(0.3);
        band.padding_outer = Percentage(0.15);
        band.group_padding = Percentage(0.1);
        let marks = (0..chart.series.len())
            .map(|i| {
                bar()
                    .value(Some(RowAccessor::key(key(i))))
                    .stack(stacked.then(|| SharedString::from("stack")))
                    .stagger(Percentage(0.3))
                    .inactive_opacity(Some(Percentage(INACTIVE)))
                    .fill_color(move || tokens::chart(i))
                    .into_element()
            })
            .collect();
        chart.frame(Scale::Band(band), true, marks)
    }
}

// ---- Pie ------------------------------------------------------------------

/// Slices of a whole, with a legend of shares. Build with [`pie_chart`].
#[derive(Clone)]
pub struct PieChart {
    slices: Vec<(SharedString, f64)>,
    label: Option<SharedString>,
    size: f32,
    inner: f32,
    legend: bool,
    interactive: bool,
    format: Option<Format>,
}

/// A pie chart of `(label, value)` slices.
pub fn pie_chart<L: Into<SharedString>>(data: impl IntoIterator<Item = (L, f64)>) -> PieChart {
    PieChart {
        slices: data.into_iter().map(|(l, v)| (l.into(), v)).collect(),
        label: None,
        size: 180.0,
        inner: 0.0,
        legend: true,
        interactive: true,
        format: None,
    }
}

impl PieChart {
    /// What screen readers say of the chart; by default its slices and
    /// their shares.
    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// The pie's diameter in points (180 by default).
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    /// A donut: the hole's radius as a share of the pie's (0 to 0.95).
    pub fn donut(mut self, inner: f32) -> Self {
        self.inner = inner.clamp(0.0, 0.95);
        self
    }
    /// The legend of names and shares beside the pie (on by default).
    pub fn legend(mut self, legend: bool) -> Self {
        self.legend = legend;
        self
    }
    /// Follows the pointer and the keys with a tooltip (on by default).
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }
    /// How values are written (the tooltip and the summary).
    pub fn format(mut self, format: impl Fn(f64) -> SharedString + 'static) -> Self {
        self.format = Some(Rc::new(format));
        self
    }

    /// A slice's value as drawn: positive, or nothing.
    fn value(&self, index: usize) -> f64 {
        match self.slices.get(index) {
            Some((_, v)) if v.is_finite() && *v > 0.0 => *v,
            _ => 0.0,
        }
    }

    fn share(&self, index: usize) -> f64 {
        let total: f64 = (0..self.slices.len()).map(|i| self.value(i)).sum();
        if total > 0.0 {
            self.value(index) / total
        } else {
            0.0
        }
    }

    fn summary(&self) -> SharedString {
        if let Some(label) = &self.label {
            return label.clone();
        }
        let parts: Vec<String> = (0..self.slices.len())
            .map(|i| format!("{} {:.0}%", self.slices[i].0, self.share(i) * 100.0))
            .collect();
        format!("Pie chart: {}", parts.join(", ")).into()
    }

    fn slice_text(&self, index: usize) -> String {
        let format = self
            .format
            .clone()
            .unwrap_or_else(|| Rc::new(default_format));
        let (name, value) = &self.slices[index];
        format!(
            "{name}: {} ({:.0}%)",
            format(*value),
            self.share(index) * 100.0
        )
    }
}

impl IntoElement for PieChart {
    fn into_element(self) -> AnyElement {
        let hovered = signal(None::<u32>);
        let this = Rc::new(self);
        let len = this.slices.len();
        let rows = Rows::from_columns(
            len,
            [("value".into(), (0..len).map(|i| this.value(i)).collect())],
        );
        let inner = relative(this.inner);

        // Each slice an arc in its own series color (so it follows the
        // theme), the pie over them taking the pointer and the keys.
        let mut start = 0.0_f32;
        let mut arcs: Vec<AnyElement> = Vec::with_capacity(len);
        for i in 0..len {
            let end = start + this.share(i) as f32 * TAU;
            arcs.push(
                arc()
                    .start_angle(Radians(start))
                    .end_angle(Radians(end))
                    .inner_radius(inner)
                    .fill_color(move || tokens::chart(i))
                    .stroke_color(move || tokens::background())
                    .stroke_width(px(1.0))
                    .opacity(move || match hovered.get() {
                        Some(h) if h as usize != i => INACTIVE,
                        _ => 1.0,
                    })
                    .transition_opacity()
                    .duration(tokens::DURATION_FAST)
                    .into_element(),
            );
            start = end;
        }

        let read = this.clone();
        let mut p = plot()
            .data(rows)
            .interactive(this.interactive)
            .motion(Some(motion()))
            .accessibility_label(Some(this.summary()))
            .accessibility_value(move || {
                hovered.get().map(|i| PlotAccessibilityValue {
                    text: Some(read.slice_text(i as usize).into()),
                    ..PlotAccessibilityValue::default()
                })
            })
            .w(px(this.size))
            .h(px(this.size))
            .on_hover_change(move |index: &Option<u32>| hovered.set(*index))
            .children(arcs)
            .child(
                pie()
                    .value(Some(RowAccessor::key("value")))
                    .inner_radius(inner)
                    .fill_color(move || AdaptiveColor::TRANSPARENT)
                    .stroke_none(),
            );
        if this.interactive {
            let tip = this.clone();
            p = p
                .tooltip_placement(TooltipPlacement::Pointer)
                .tooltip_offset(Points(tokens::SPACE_2))
                .tooltip_content(
                    tooltip_box()
                        .flex_row()
                        .items_center()
                        .gap(px(tokens::SPACE_2))
                        .child(
                            view()
                                .w(px(SWATCH))
                                .h(px(SWATCH))
                                .bg(move || tokens::chart(hovered.get().unwrap_or(0) as usize)),
                        )
                        .child(
                            text(move || {
                                hovered
                                    .get()
                                    .map(|i| tip.slice_text(i as usize))
                                    .unwrap_or_default()
                            })
                            .text_color(move || tokens::foreground()),
                        ),
                );
        }

        let items = (0..len)
            .map(|i| {
                let share = SharedString::from(format!("{:.0}%", this.share(i) * 100.0));
                (i, this.slices[i].0.clone(), Some(share))
            })
            .collect();
        view()
            .flex_row()
            .items_center()
            .gap(px(tokens::SPACE_6))
            .child(p)
            .child(this.legend.then(|| legend(items, true)))
            .into_element()
    }
}
