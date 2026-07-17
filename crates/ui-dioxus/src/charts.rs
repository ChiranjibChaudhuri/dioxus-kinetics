use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Shared chart vocabulary
// ---------------------------------------------------------------------------

/// Accent tone for single-series charts (`Sparkline`, `DonutGauge`).
/// Multi-series charts ignore the tone and cycle the `--ui-chart-1..6`
/// series ramp instead.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartTone {
    #[default]
    Primary,
    Success,
    Warning,
    Danger,
    Info,
    Neutral,
}

impl ChartTone {
    pub const fn class_suffix(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Info => "info",
            Self::Neutral => "neutral",
        }
    }
}

/// One named data series for `LineChart` / `BarChart`.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartSeries {
    pub name: String,
    pub points: Vec<f32>,
}

impl ChartSeries {
    pub fn new(name: impl Into<String>, points: Vec<f32>) -> Self {
        Self {
            name: name.into(),
            points,
        }
    }
}

/// Plot rectangle inside the SVG viewBox, in viewBox units.
#[derive(Clone, Copy, Debug, PartialEq)]
struct PlotRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl PlotRect {
    const fn right(self) -> f32 {
        self.x + self.width
    }

    const fn bottom(self) -> f32 {
        self.y + self.height
    }
}

// ---------------------------------------------------------------------------
// Pure geometry helpers (unit-tested)
// ---------------------------------------------------------------------------

/// Min/max over every finite point in every series. `None` when there is no
/// finite data at all, so callers can render an explicit empty state instead
/// of a degenerate axis.
fn series_bounds(series: &[ChartSeries]) -> Option<(f32, f32)> {
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    for s in series {
        for value in &s.points {
            if value.is_finite() {
                min = min.min(*value);
                max = max.max(*value);
            }
        }
    }
    if min.is_finite() && max.is_finite() {
        Some((min, max))
    } else {
        None
    }
}

/// "Nice number" rounding from Heckbert's axis-labelling algorithm: snaps a
/// raw range to 1/2/5×10ⁿ so tick values land on human-friendly numbers.
fn nice_num(range: f32, round: bool) -> f32 {
    if range <= 0.0 || !range.is_finite() {
        return 1.0;
    }
    let exp = range.log10().floor();
    let magnitude = 10f32.powf(exp);
    let fraction = range / magnitude;
    let nice_fraction = if round {
        if fraction < 1.5 {
            1.0
        } else if fraction < 3.0 {
            2.0
        } else if fraction < 7.0 {
            5.0
        } else {
            10.0
        }
    } else if fraction <= 1.0 {
        1.0
    } else if fraction <= 2.0 {
        2.0
    } else if fraction <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice_fraction * magnitude
}

/// Evenly spaced, nicely rounded tick values covering `[min, max]`. The
/// returned ticks may extend slightly past the data so the axis ends on a
/// round number; charts use `ticks.first()/last()` as the plot domain.
fn nice_ticks(min: f32, max: f32, target_count: usize) -> Vec<f32> {
    if !min.is_finite() || !max.is_finite() || target_count < 2 {
        return Vec::new();
    }
    // A flat series still needs a vertical domain to draw within; pad ±1
    // (or ±|value| for large magnitudes) around the constant value.
    let (min, max) = if (max - min).abs() < f32::EPSILON {
        let pad = min.abs().max(1.0);
        (min - pad, max + pad)
    } else {
        (min, max)
    };
    let range = nice_num(max - min, false);
    let step = nice_num(range / (target_count - 1) as f32, true);
    let lo = (min / step).floor() * step;
    let hi = (max / step).ceil() * step;

    let mut ticks = Vec::new();
    let mut value = lo;
    // The 0.5-step epsilon keeps `hi` itself included despite float drift.
    while value <= hi + step * 0.5 {
        // Snap near-zero float residue (e.g. -1.19e-7) to exactly 0.
        ticks.push(if value.abs() < step * 1e-4 {
            0.0
        } else {
            value
        });
        value += step;
    }
    ticks
}

/// Compact tick label: trims trailing `.0` and abbreviates thousands and
/// millions (`1500.0 → "1.5k"`).
fn format_tick(value: f32) -> String {
    fn trim(v: f32) -> String {
        let s = format!("{v:.1}");
        s.strip_suffix(".0").unwrap_or(&s).to_string()
    }
    let abs = value.abs();
    if abs >= 1_000_000.0 {
        format!("{}M", trim(value / 1_000_000.0))
    } else if abs >= 1_000.0 {
        format!("{}k", trim(value / 1_000.0))
    } else {
        trim(value)
    }
}

fn map_x(index: usize, count: usize, plot: PlotRect) -> f32 {
    if count <= 1 {
        return plot.x + plot.width / 2.0;
    }
    plot.x + plot.width * (index as f32) / ((count - 1) as f32)
}

fn map_y(value: f32, min: f32, max: f32, plot: PlotRect) -> f32 {
    let span = (max - min).max(1e-6);
    let t = ((value - min) / span).clamp(0.0, 1.0);
    // SVG y grows downward; the largest value maps to the plot top.
    plot.bottom() - t * plot.height
}

/// SVG path (`M … L …`) through the series points, skipping non-finite
/// values. `None` when fewer than two drawable points remain.
fn line_path(points: &[f32], min: f32, max: f32, plot: PlotRect) -> Option<String> {
    let mut d = String::new();
    let mut drawn = 0usize;
    for (index, value) in points.iter().enumerate() {
        if !value.is_finite() {
            continue;
        }
        let x = map_x(index, points.len(), plot);
        let y = map_y(*value, min, max, plot);
        if drawn == 0 {
            d.push_str(&format!("M{x:.2} {y:.2}"));
        } else {
            d.push_str(&format!(" L{x:.2} {y:.2}"));
        }
        drawn += 1;
    }
    (drawn >= 2).then_some(d)
}

/// Closed variant of [`line_path`] dropping to the plot floor, for area
/// fills under a line.
fn area_path(points: &[f32], min: f32, max: f32, plot: PlotRect) -> Option<String> {
    let open = line_path(points, min, max, plot)?;
    let first_x = points
        .iter()
        .position(|v| v.is_finite())
        .map(|i| map_x(i, points.len(), plot))?;
    let last_x = points
        .iter()
        .rposition(|v| v.is_finite())
        .map(|i| map_x(i, points.len(), plot))?;
    Some(format!(
        "{open} L{:.2} {:.2} L{:.2} {:.2} Z",
        last_x,
        plot.bottom(),
        first_x,
        plot.bottom()
    ))
}

/// Geometry for one bar in a grouped `BarChart`, in viewBox units.
#[derive(Clone, Copy, Debug, PartialEq)]
struct BarGeom {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

/// Which slot of the grouped layout a bar occupies: series N of M, at
/// category index N of M.
#[derive(Clone, Copy, Debug, PartialEq)]
struct BarSlot {
    series_index: usize,
    series_count: usize,
    point_index: usize,
    point_count: usize,
}

/// Grouped-bar layout: each category index owns an equal slice of the plot
/// width; bars for every series share the inner 72% of that slice. Bars are
/// measured from the domain floor (`min`), which `nice_ticks` extends to 0
/// for all-positive data.
fn bar_geometry(slot: BarSlot, value: f32, min: f32, max: f32, plot: PlotRect) -> Option<BarGeom> {
    if !value.is_finite() || slot.point_count == 0 || slot.series_count == 0 {
        return None;
    }
    let group_width = plot.width / slot.point_count as f32;
    let inner_width = group_width * 0.72;
    let bar_width = inner_width / slot.series_count as f32;
    let group_left =
        plot.x + group_width * slot.point_index as f32 + (group_width - inner_width) / 2.0;
    let x = group_left + bar_width * slot.series_index as f32;

    let baseline = map_y(min.max(0.0).min(max), min, max, plot);
    let top = map_y(value, min, max, plot);
    let (y, height) = if top <= baseline {
        (top, baseline - top)
    } else {
        (baseline, top - baseline)
    };
    Some(BarGeom {
        x,
        y,
        width: (bar_width - 1.0).max(1.0),
        height: height.max(0.5),
    })
}

/// Class for the Nth series in a multi-series chart; cycles the six-step
/// series color ramp defined in `ui-styles`.
fn series_class(index: usize) -> String {
    format!("ui-chart-series--{}", index % 6 + 1)
}

/// Visible fraction of a draw-in animation overridden by `progress`, clamped
/// and NaN-safe so capture callers can hand in raw clock samples.
fn clamp_progress(progress: f32) -> f32 {
    if progress.is_finite() {
        progress.clamp(0.0, 1.0)
    } else {
        1.0
    }
}

// ---------------------------------------------------------------------------
// Sparkline
// ---------------------------------------------------------------------------

/// A compact, axis-free trend line. Decorative (`aria-hidden`) when `label`
/// is empty — the surrounding readout is expected to carry the value — and
/// `role="img"` with the label as accessible name otherwise.
#[component]
pub fn Sparkline(
    points: Vec<f32>,
    #[props(default)] label: String,
    #[props(default)] tone: ChartTone,
    #[props(default)] filled: bool,
) -> Element {
    const PLOT: PlotRect = PlotRect {
        x: 1.0,
        y: 2.0,
        width: 98.0,
        height: 28.0,
    };
    let bounds = series_bounds(&[ChartSeries::new("", points.clone())]);
    let path = bounds.and_then(|(min, max)| line_path(&points, min, max, PLOT));
    let area = (filled && path.is_some())
        .then(|| bounds.and_then(|(min, max)| area_path(&points, min, max, PLOT)))
        .flatten();

    let class = format!("ui-sparkline ui-sparkline--{}", tone.class_suffix());
    let decorative = label.is_empty();

    rsx! {
        span {
            class: "{class}",
            role: if decorative { "presentation" } else { "img" },
            "aria-hidden": if decorative { "true" } else { "false" },
            "aria-label": if decorative { "" } else { "{label}" },
            svg {
                view_box: "0 0 100 32",
                preserve_aspect_ratio: "none",
                "aria-hidden": "true",
                if let Some(d) = area {
                    path { class: "ui-sparkline-area", d: "{d}" }
                }
                if let Some(d) = path {
                    path {
                        class: "ui-sparkline-line",
                        d: "{d}",
                        fill: "none",
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// LineChart
// ---------------------------------------------------------------------------

/// A multi-series line chart with nice-number gridlines, an optional legend,
/// and a spring-paced draw-in. The SVG is `role="img"` named by `label`; the
/// underlying numbers are mirrored in a visually-hidden table so screen
/// reader users get the data, not a picture.
///
/// Set `progress` (0.0–1.0) to pin the draw-in deterministically — e.g. from
/// a `Scene` clock sample — instead of letting CSS animate it; `animate:
/// false` renders the settled chart with no motion at all.
#[component]
pub fn LineChart(
    label: String,
    series: Vec<ChartSeries>,
    #[props(default)] x_labels: Vec<String>,
    #[props(default = true)] show_grid: bool,
    #[props(default = true)] show_legend: bool,
    #[props(default)] show_area: bool,
    #[props(default = true)] animate: bool,
    #[props(default)] progress: Option<f32>,
) -> Element {
    const PLOT: PlotRect = PlotRect {
        x: 38.0,
        y: 10.0,
        width: 274.0,
        height: 138.0,
    };
    let bounds = series_bounds(&series);
    let ticks = bounds
        .map(|(min, max)| nice_ticks(min, max, 5))
        .unwrap_or_default();
    let (domain_min, domain_max) = match (ticks.first(), ticks.last()) {
        (Some(first), Some(last)) => (*first, *last),
        _ => (0.0, 1.0),
    };

    let static_offset = progress.map(|p| 1.0 - clamp_progress(p));
    let animated = animate && static_offset.is_none();
    let root_class = format!(
        "ui-chart ui-chart--line{}",
        if animated { " ui-chart--animate" } else { "" }
    );
    let label_count = x_labels.len();

    rsx! {
        figure { class: "{root_class}",
            svg {
                class: "ui-chart-canvas",
                view_box: "0 0 320 180",
                role: "img",
                "aria-label": "{label}",
                if bounds.is_none() {
                    text {
                        class: "ui-chart-empty",
                        x: "160",
                        y: "94",
                        text_anchor: "middle",
                        "No data"
                    }
                }
                if show_grid {
                    g { class: "ui-chart-grid", "aria-hidden": "true",
                        for tick in ticks.iter().copied() {
                            line {
                                x1: "{PLOT.x}",
                                x2: "{PLOT.right()}",
                                y1: "{map_y(tick, domain_min, domain_max, PLOT)}",
                                y2: "{map_y(tick, domain_min, domain_max, PLOT)}",
                            }
                            text {
                                class: "ui-chart-tick",
                                x: "{PLOT.x - 6.0}",
                                y: "{map_y(tick, domain_min, domain_max, PLOT) + 3.0}",
                                text_anchor: "end",
                                "{format_tick(tick)}"
                            }
                        }
                    }
                }
                g { "aria-hidden": "true",
                    for (index, s) in series.iter().enumerate() {
                        if show_area {
                            if let Some(d) = area_path(&s.points, domain_min, domain_max, PLOT) {
                                path { class: "ui-chart-area {series_class(index)}", d: "{d}" }
                            }
                        }
                        if let Some(d) = line_path(&s.points, domain_min, domain_max, PLOT) {
                            path {
                                class: "ui-chart-line {series_class(index)}",
                                d: "{d}",
                                fill: "none",
                                "pathLength": "1",
                                style: match static_offset {
                                    Some(offset) => format!("stroke-dashoffset:{offset:.4}"),
                                    None => format!("animation-delay:{}ms", index * 140),
                                },
                            }
                        }
                    }
                }
                g { class: "ui-chart-x-labels", "aria-hidden": "true",
                    for (index, text_label) in x_labels.iter().enumerate() {
                        text {
                            class: "ui-chart-tick",
                            x: "{map_x(index, label_count, PLOT)}",
                            y: "{PLOT.bottom() + 16.0}",
                            text_anchor: "middle",
                            "{text_label}"
                        }
                    }
                }
            }
            if show_legend && series.len() > 1 {
                figcaption { class: "ui-chart-legend",
                    for (index, s) in series.iter().enumerate() {
                        span { class: "ui-chart-legend-item",
                            span {
                                class: "ui-chart-swatch {series_class(index)}",
                                "aria-hidden": "true",
                            }
                            "{s.name}"
                        }
                    }
                }
            }
            {chart_sr_table(&label, &series, &x_labels)}
        }
    }
}

// ---------------------------------------------------------------------------
// BarChart
// ---------------------------------------------------------------------------

/// A grouped bar chart with a staggered rise-in. Accessibility mirrors
/// `LineChart`: named SVG plus a visually-hidden data table. `progress` pins
/// the rise deterministically for capture; `animate: false` disables motion.
#[component]
pub fn BarChart(
    label: String,
    series: Vec<ChartSeries>,
    #[props(default)] x_labels: Vec<String>,
    #[props(default = true)] show_grid: bool,
    #[props(default = true)] show_legend: bool,
    #[props(default = true)] animate: bool,
    #[props(default)] progress: Option<f32>,
) -> Element {
    const PLOT: PlotRect = PlotRect {
        x: 38.0,
        y: 10.0,
        width: 274.0,
        height: 138.0,
    };
    let bounds = series_bounds(&series);
    // Anchor the domain at zero so bar lengths stay proportional to values.
    let ticks = bounds
        .map(|(min, max)| nice_ticks(min.min(0.0), max.max(0.0), 5))
        .unwrap_or_default();
    let (domain_min, domain_max) = match (ticks.first(), ticks.last()) {
        (Some(first), Some(last)) => (*first, *last),
        _ => (0.0, 1.0),
    };

    let static_scale = progress.map(clamp_progress);
    let animated = animate && static_scale.is_none();
    let root_class = format!(
        "ui-chart ui-chart--bar{}",
        if animated { " ui-chart--animate" } else { "" }
    );
    let point_count = series.iter().map(|s| s.points.len()).max().unwrap_or(0);
    let series_count = series.len();
    let label_count = x_labels.len();

    rsx! {
        figure { class: "{root_class}",
            svg {
                class: "ui-chart-canvas",
                view_box: "0 0 320 180",
                role: "img",
                "aria-label": "{label}",
                if bounds.is_none() {
                    text {
                        class: "ui-chart-empty",
                        x: "160",
                        y: "94",
                        text_anchor: "middle",
                        "No data"
                    }
                }
                if show_grid {
                    g { class: "ui-chart-grid", "aria-hidden": "true",
                        for tick in ticks.iter().copied() {
                            line {
                                x1: "{PLOT.x}",
                                x2: "{PLOT.right()}",
                                y1: "{map_y(tick, domain_min, domain_max, PLOT)}",
                                y2: "{map_y(tick, domain_min, domain_max, PLOT)}",
                            }
                            text {
                                class: "ui-chart-tick",
                                x: "{PLOT.x - 6.0}",
                                y: "{map_y(tick, domain_min, domain_max, PLOT) + 3.0}",
                                text_anchor: "end",
                                "{format_tick(tick)}"
                            }
                        }
                    }
                }
                g { "aria-hidden": "true",
                    for (series_index, s) in series.iter().enumerate() {
                        for (point_index, value) in s.points.iter().enumerate() {
                            if let Some(bar) = bar_geometry(
                                BarSlot {
                                    series_index,
                                    series_count,
                                    point_index,
                                    point_count,
                                },
                                *value,
                                domain_min,
                                domain_max,
                                PLOT,
                            ) {
                                rect {
                                    class: "ui-chart-bar {series_class(series_index)}",
                                    x: "{bar.x}",
                                    y: "{bar.y}",
                                    width: "{bar.width}",
                                    height: "{bar.height}",
                                    rx: "1.5",
                                    style: match static_scale {
                                        Some(scale) => format!("transform:scaleY({scale:.4})"),
                                        None => format!(
                                            "animation-delay:{}ms",
                                            point_index * 40 + series_index * 90
                                        ),
                                    },
                                }
                            }
                        }
                    }
                }
                g { class: "ui-chart-x-labels", "aria-hidden": "true",
                    for (index, text_label) in x_labels.iter().enumerate() {
                        text {
                            class: "ui-chart-tick",
                            x: "{bar_group_center(index, label_count.max(point_count), PLOT)}",
                            y: "{PLOT.bottom() + 16.0}",
                            text_anchor: "middle",
                            "{text_label}"
                        }
                    }
                }
            }
            if show_legend && series.len() > 1 {
                figcaption { class: "ui-chart-legend",
                    for (index, s) in series.iter().enumerate() {
                        span { class: "ui-chart-legend-item",
                            span {
                                class: "ui-chart-swatch {series_class(index)}",
                                "aria-hidden": "true",
                            }
                            "{s.name}"
                        }
                    }
                }
            }
            {chart_sr_table(&label, &series, &x_labels)}
        }
    }
}

/// Horizontal center of a bar group, used to place category labels.
fn bar_group_center(index: usize, count: usize, plot: PlotRect) -> f32 {
    if count == 0 {
        return plot.x + plot.width / 2.0;
    }
    let group_width = plot.width / count as f32;
    plot.x + group_width * (index as f32 + 0.5)
}

// ---------------------------------------------------------------------------
// DonutGauge
// ---------------------------------------------------------------------------

/// A radial gauge sweeping from 12 o'clock. Exposes `role="meter"` with the
/// percentage as `aria-valuenow` and `display_value` as the human-readable
/// `aria-valuetext`. `progress` pins the sweep for deterministic capture.
#[component]
pub fn DonutGauge(
    label: String,
    value: f32,
    #[props(default)] display_value: String,
    #[props(default)] description: String,
    #[props(default)] tone: ChartTone,
    #[props(default = true)] animate: bool,
    #[props(default)] progress: Option<f32>,
) -> Element {
    let value = clamp_progress(value);
    let percent = (value * 100.0).round() as u32;
    let shown_value = if display_value.is_empty() {
        format!("{percent}%")
    } else {
        display_value
    };

    let static_fraction = progress.map(|p| value * clamp_progress(p));
    let animated = animate && static_fraction.is_none();
    let root_class = format!(
        "ui-donut-gauge ui-donut-gauge--{}{}",
        tone.class_suffix(),
        if animated { " ui-chart--animate" } else { "" }
    );
    // pathLength="1" normalizes the circumference, so dasharray "v 1" shows
    // exactly the value fraction regardless of the actual radius.
    let arc_style = match static_fraction {
        Some(fraction) => format!("stroke-dasharray:{fraction:.4} 1"),
        None => format!("--ui-gauge-value:{value:.4}"),
    };

    rsx! {
        div {
            class: "{root_class}",
            role: "meter",
            "aria-label": "{label}",
            "aria-valuemin": "0",
            "aria-valuemax": "100",
            "aria-valuenow": "{percent}",
            "aria-valuetext": "{shown_value}",
            svg {
                class: "ui-donut-gauge-canvas",
                view_box: "0 0 120 120",
                "aria-hidden": "true",
                circle {
                    class: "ui-donut-gauge-track",
                    cx: "60",
                    cy: "60",
                    r: "52",
                    fill: "none",
                }
                circle {
                    class: "ui-donut-gauge-arc",
                    cx: "60",
                    cy: "60",
                    r: "52",
                    fill: "none",
                    transform: "rotate(-90 60 60)",
                    "pathLength": "1",
                    style: "{arc_style}",
                }
            }
            div { class: "ui-donut-gauge-readout", "aria-hidden": "true",
                strong { class: "ui-donut-gauge-value", "{shown_value}" }
                if !description.is_empty() {
                    span { class: "ui-donut-gauge-description", "{description}" }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Screen-reader data table shared by LineChart / BarChart
// ---------------------------------------------------------------------------

fn chart_sr_table(label: &str, series: &[ChartSeries], x_labels: &[String]) -> Element {
    if series.is_empty() {
        return rsx! {};
    }
    let point_count = series.iter().map(|s| s.points.len()).max().unwrap_or(0);
    let columns: Vec<String> = (0..point_count)
        .map(|i| {
            x_labels
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("Point {}", i + 1))
        })
        .collect();
    let rows: Vec<(String, Vec<String>)> = series
        .iter()
        .map(|s| {
            let cells = (0..point_count)
                .map(|i| {
                    s.points
                        .get(i)
                        .filter(|v| v.is_finite())
                        .map(|v| format_tick(*v))
                        .unwrap_or_else(|| "—".to_string())
                })
                .collect();
            (s.name.clone(), cells)
        })
        .collect();

    rsx! {
        table { class: "visually-hidden",
            caption { "{label}" }
            thead {
                tr {
                    th { scope: "col", "Series" }
                    for column in columns.iter() {
                        th { scope: "col", "{column}" }
                    }
                }
            }
            tbody {
                for (name, cells) in rows.iter() {
                    tr {
                        th { scope: "row", "{name}" }
                        for cell in cells.iter() {
                            td { "{cell}" }
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AreaChart — opinionated always-filled variant of LineChart
// ---------------------------------------------------------------------------

/// A multi-series area chart. Like `LineChart` but always fills the region
/// under each series (translucent) and draws the series line on top. Shares
/// the SR data-table mirror, nice-number grid, and `progress` override.
#[component]
pub fn AreaChart(
    label: String,
    series: Vec<ChartSeries>,
    #[props(default)] x_labels: Vec<String>,
    #[props(default = true)] show_grid: bool,
    #[props(default = true)] show_legend: bool,
    #[props(default = true)] animate: bool,
    #[props(default)] progress: Option<f32>,
) -> Element {
    const PLOT: PlotRect = PlotRect {
        x: 38.0,
        y: 10.0,
        width: 274.0,
        height: 138.0,
    };
    let bounds = series_bounds(&series);
    let ticks = bounds
        .map(|(min, max)| nice_ticks(min, max, 5))
        .unwrap_or_default();
    let (domain_min, domain_max) = match (ticks.first(), ticks.last()) {
        (Some(first), Some(last)) => (*first, *last),
        _ => (0.0, 1.0),
    };
    let static_offset = progress.map(|p| 1.0 - clamp_progress(p));
    let animated = animate && static_offset.is_none();
    let root_class = format!(
        "ui-chart ui-chart--area{}",
        if animated { " ui-chart--animate" } else { "" }
    );
    let label_count = x_labels.len();

    rsx! {
        figure { class: "{root_class}",
            svg {
                class: "ui-chart-canvas",
                view_box: "0 0 320 180",
                role: "img",
                "aria-label": "{label}",
                if bounds.is_none() {
                    text { class: "ui-chart-empty", x: "160", y: "94", text_anchor: "middle", "No data" }
                }
                if show_grid {
                    g { class: "ui-chart-grid", "aria-hidden": "true",
                        for tick in ticks.iter().copied() {
                            line {
                                x1: "{PLOT.x}",
                                x2: "{PLOT.right()}",
                                y1: "{map_y(tick, domain_min, domain_max, PLOT)}",
                                y2: "{map_y(tick, domain_min, domain_max, PLOT)}",
                            }
                            text {
                                class: "ui-chart-tick",
                                x: "{PLOT.x - 6.0}",
                                y: "{map_y(tick, domain_min, domain_max, PLOT) + 3.0}",
                                text_anchor: "end",
                                "{format_tick(tick)}"
                            }
                        }
                    }
                }
                g { "aria-hidden": "true",
                    for (index, s) in series.iter().enumerate() {
                        if let Some(d) = area_path(&s.points, domain_min, domain_max, PLOT) {
                            path { class: "ui-chart-area {series_class(index)}", d: "{d}" }
                        }
                        if let Some(d) = line_path(&s.points, domain_min, domain_max, PLOT) {
                            path {
                                class: "ui-chart-line {series_class(index)}",
                                d: "{d}",
                                fill: "none",
                                "pathLength": "1",
                                style: match static_offset {
                                    Some(offset) => format!("stroke-dashoffset:{offset:.4}"),
                                    None => format!("animation-delay:{}ms", index * 140),
                                },
                            }
                        }
                    }
                }
                g { class: "ui-chart-x-labels", "aria-hidden": "true",
                    for (index, text_label) in x_labels.iter().enumerate() {
                        text {
                            class: "ui-chart-tick",
                            x: "{map_x(index, label_count, PLOT)}",
                            y: "{PLOT.bottom() + 16.0}",
                            text_anchor: "middle",
                            "{text_label}"
                        }
                    }
                }
            }
            if show_legend && series.len() > 1 {
                figcaption { class: "ui-chart-legend",
                    for (index, s) in series.iter().enumerate() {
                        span { class: "ui-chart-legend-item",
                            span { class: "ui-chart-swatch {series_class(index)}", "aria-hidden": "true" }
                            "{s.name}"
                        }
                    }
                }
            }
            {chart_sr_table(&label, &series, &x_labels)}
        }
    }
}

// ---------------------------------------------------------------------------
// FunnelChart
// ---------------------------------------------------------------------------

/// One stage of a conversion funnel.
#[derive(Clone, Debug, PartialEq)]
pub struct FunnelStage {
    pub label: String,
    pub value: f32,
}

impl FunnelStage {
    pub fn new(label: impl Into<String>, value: f32) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

/// Largest finite value in a slice, or `None` when empty/all-non-finite.
fn finite_max(values: &[f32]) -> Option<f32> {
    values
        .iter()
        .copied()
        .filter(|v| v.is_finite())
        .reduce(f32::max)
}

/// Centered horizontal bar for a funnel stage occupying vertical band
/// `index` of `count`, width proportional to `value / max`.
fn funnel_bar(index: usize, count: usize, value: f32, max: f32, plot: PlotRect) -> BarGeom {
    let band = plot.height / count.max(1) as f32;
    let gap = band * 0.18;
    let height = (band - gap).max(1.0);
    let y = plot.y + band * index as f32 + gap / 2.0;
    let frac = if max > 0.0 {
        (value / max).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let width = (plot.width * frac).max(1.0);
    let x = plot.x + (plot.width - width) / 2.0;
    BarGeom {
        x,
        y,
        width,
        height,
    }
}

/// Precomputed draw data for one funnel stage (rsx renders structs, not
/// `let` statements).
struct FunnelDraw {
    series_index: usize,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    label: String,
    value: String,
}

/// A conversion funnel: stacked, centered stages narrowing with value. Each
/// stage carries its label and formatted value; the data is mirrored in a
/// visually-hidden table. `progress` (0..1) scales every stage's width for
/// deterministic capture.
#[component]
pub fn FunnelChart(
    label: String,
    stages: Vec<FunnelStage>,
    #[props(default = true)] animate: bool,
    #[props(default)] progress: Option<f32>,
) -> Element {
    const PLOT: PlotRect = PlotRect {
        x: 12.0,
        y: 12.0,
        width: 226.0,
        height: 156.0,
    };
    let max = finite_max(&stages.iter().map(|s| s.value).collect::<Vec<_>>()).unwrap_or(0.0);
    let scale = progress.map(clamp_progress);
    let animated = animate && scale.is_none();
    let root_class = format!(
        "ui-chart ui-chart--funnel{}",
        if animated { " ui-chart--animate" } else { "" }
    );
    let stage_count = stages.len();
    let draws: Vec<FunnelDraw> = stages
        .iter()
        .enumerate()
        .filter(|(_, stage)| stage.value.is_finite())
        .map(|(index, stage)| {
            let bar = funnel_bar(index, stage_count, stage.value, max, PLOT);
            let width = scale.map_or(bar.width, |s| (bar.width * s).max(1.0));
            FunnelDraw {
                series_index: index,
                x: bar.x,
                y: bar.y,
                width,
                height: bar.height,
                label: stage.label.clone(),
                value: format_tick(stage.value),
            }
        })
        .collect();

    rsx! {
        figure { class: "{root_class}",
            svg {
                class: "ui-chart-canvas",
                view_box: "0 0 250 180",
                role: "img",
                "aria-label": "{label}",
                if stages.is_empty() {
                    text { class: "ui-chart-empty", x: "125", y: "94", text_anchor: "middle", "No data" }
                }
                g { "aria-hidden": "true",
                    for d in draws.iter() {
                        rect {
                            class: "ui-chart-funnel-stage {series_class(d.series_index)}",
                            x: "{d.x}",
                            y: "{d.y}",
                            width: "{d.width}",
                            height: "{d.height}",
                            rx: "3.0",
                            style: if animated {
                                format!("animation-delay:{}ms", d.series_index * 60)
                            } else {
                                String::new()
                            },
                        }
                        text {
                            class: "ui-chart-funnel-label",
                            x: "{d.x + d.width / 2.0}",
                            y: "{d.y + d.height / 2.0 + 3.0}",
                            text_anchor: "middle",
                            "{d.label} — {d.value}"
                        }
                    }
                }
            }
            {funnel_sr(&label, &stages)}
        }
    }
}

fn funnel_sr(label: &str, stages: &[FunnelStage]) -> Element {
    if stages.is_empty() {
        return rsx! {};
    }
    rsx! {
        table { class: "visually-hidden",
            caption { "{label}" }
            tbody {
                for stage in stages.iter() {
                    tr {
                        th { scope: "row", "{stage.label}" }
                        td { "{format_tick(stage.value)}" }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// GaugeChart — semicircle gauge (distinct from the full-ring DonutGauge)
// ---------------------------------------------------------------------------

/// A semicircle (180°) gauge sweeping from 9 o'clock to 3 o'clock over the
/// top. Mirrors `DonutGauge`'s a11y contract (`role="meter"`, valuenow,
/// valuetext) but reads as a speedometer/progress arc rather than a ring.
#[component]
pub fn GaugeChart(
    label: String,
    value: f32,
    #[props(default)] display_value: String,
    #[props(default)] description: String,
    #[props(default)] tone: ChartTone,
    #[props(default = true)] animate: bool,
    #[props(default)] progress: Option<f32>,
) -> Element {
    let value = clamp_progress(value);
    let percent = (value * 100.0).round() as u32;
    let shown_value = if display_value.is_empty() {
        format!("{percent}%")
    } else {
        display_value
    };
    let static_fraction = progress.map(|p| value * clamp_progress(p));
    let animated = animate && static_fraction.is_none();
    let root_class = format!(
        "ui-gauge ui-gauge--{}{}",
        tone.class_suffix(),
        if animated { " ui-chart--animate" } else { "" }
    );
    let arc_style = match static_fraction {
        Some(fraction) => format!("stroke-dasharray:{fraction:.4} 1"),
        None => format!("--ui-gauge-value:{value:.4}"),
    };
    let arc_path = "M 20 100 A 80 80 0 0 1 180 100";

    rsx! {
        div {
            class: "{root_class}",
            role: "meter",
            "aria-label": "{label}",
            "aria-valuemin": "0",
            "aria-valuemax": "100",
            "aria-valuenow": "{percent}",
            "aria-valuetext": "{shown_value}",
            svg {
                class: "ui-gauge-canvas",
                view_box: "0 0 200 112",
                "aria-hidden": "true",
                path { class: "ui-gauge-track", d: "{arc_path}", fill: "none" }
                path {
                    class: "ui-gauge-arc",
                    d: "{arc_path}",
                    fill: "none",
                    "pathLength": "1",
                    style: "{arc_style}",
                }
            }
            div { class: "ui-gauge-readout", "aria-hidden": "true",
                strong { class: "ui-gauge-value", "{shown_value}" }
                if !description.is_empty() {
                    span { class: "ui-gauge-description", "{description}" }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Heatmap
// ---------------------------------------------------------------------------

/// One labelled row of heatmap cells.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatmapRow {
    pub label: String,
    pub cells: Vec<f32>,
}

impl HeatmapRow {
    pub fn new(label: impl Into<String>, cells: Vec<f32>) -> Self {
        Self {
            label: label.into(),
            cells,
        }
    }
}

/// Min/max over every finite cell across all rows. `None` when empty.
fn heatmap_range(rows: &[HeatmapRow]) -> Option<(f32, f32)> {
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    for row in rows {
        for value in &row.cells {
            if value.is_finite() {
                min = min.min(*value);
                max = max.max(*value);
            }
        }
    }
    if min.is_finite() && max.is_finite() {
        Some((min, max))
    } else {
        None
    }
}

/// Normalised intensity (0..1) of `value` against the heatmap's data range.
fn intensity(value: f32, min: f32, max: f32) -> f32 {
    let span = (max - min).max(1e-6);
    ((value - min) / span).clamp(0.0, 1.0)
}

/// One rendered heatmap cell (position + intensity), precomputed for rsx.
struct HeatCell {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    alpha: f32,
}

/// A grid of cells coloured by value intensity. Cell fill opacity encodes
/// intensity against the data range; row and column labels are rendered if
/// supplied, and the matrix is mirrored in a visually-hidden table.
#[component]
pub fn Heatmap(
    label: String,
    rows: Vec<HeatmapRow>,
    #[props(default)] column_labels: Vec<String>,
) -> Element {
    const PLOT: PlotRect = PlotRect {
        x: 56.0,
        y: 12.0,
        width: 232.0,
        height: 140.0,
    };
    let cols = rows.iter().map(|r| r.cells.len()).max().unwrap_or(0);
    let row_count = rows.len();
    let cell_w = if cols > 0 {
        PLOT.width / cols as f32
    } else {
        0.0
    };
    let cell_h = if row_count > 0 {
        PLOT.height / row_count as f32
    } else {
        0.0
    };
    let range = heatmap_range(&rows);
    let (min, max) = range.unwrap_or((0.0, 1.0));

    let cells: Vec<HeatCell> = rows
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.cells
                .iter()
                .enumerate()
                .filter_map(move |(col_index, value)| {
                    if value.is_finite() {
                        Some(HeatCell {
                            x: PLOT.x + cell_w * col_index as f32,
                            y: PLOT.y + cell_h * row_index as f32,
                            width: (cell_w - 1.0).max(1.0),
                            height: (cell_h - 1.0).max(1.0),
                            alpha: intensity(*value, min, max),
                        })
                    } else {
                        None
                    }
                })
        })
        .collect();

    rsx! {
        figure { class: "ui-chart ui-chart--heatmap",
            svg {
                class: "ui-chart-canvas",
                view_box: "0 0 300 180",
                role: "img",
                "aria-label": "{label}",
                if range.is_none() {
                    text { class: "ui-chart-empty", x: "150", y: "94", text_anchor: "middle", "No data" }
                }
                g { "aria-hidden": "true",
                    for cell in cells.iter() {
                        rect {
                            class: "ui-chart-heat-cell",
                            x: "{cell.x}",
                            y: "{cell.y}",
                            width: "{cell.width}",
                            height: "{cell.height}",
                            style: "fill-opacity:{cell.alpha:.3}",
                        }
                    }
                }
                g { class: "ui-chart-row-labels", "aria-hidden": "true",
                    for (index, row) in rows.iter().enumerate() {
                        text {
                            class: "ui-chart-tick",
                            x: "{PLOT.x - 6.0}",
                            y: "{PLOT.y + cell_h * (index as f32 + 0.5)}",
                            text_anchor: "end",
                            "{row.label}"
                        }
                    }
                }
                if !column_labels.is_empty() {
                    g { class: "ui-chart-col-labels", "aria-hidden": "true",
                        for (index, text_label) in column_labels.iter().enumerate() {
                            text {
                                class: "ui-chart-tick",
                                x: "{PLOT.x + cell_w * (index as f32 + 0.5)}",
                                y: "{PLOT.bottom() + 14.0}",
                                text_anchor: "middle",
                                "{text_label}"
                            }
                        }
                    }
                }
            }
            {heatmap_sr(&label, &rows, &column_labels)}
        }
    }
}

fn heatmap_sr(label: &str, rows: &[HeatmapRow], column_labels: &[String]) -> Element {
    if rows.is_empty() {
        return rsx! {};
    }
    let cols = rows.iter().map(|r| r.cells.len()).max().unwrap_or(0);
    let headers: Vec<String> = (0..cols)
        .map(|i| {
            column_labels
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("Col {}", i + 1))
        })
        .collect();
    rsx! {
        table { class: "visually-hidden",
            caption { "{label}" }
            thead {
                tr {
                    th { scope: "col", "" }
                    for header in headers.iter() {
                        th { scope: "col", "{header}" }
                    }
                }
            }
            tbody {
                for row in rows.iter() {
                    tr {
                        th { scope: "row", "{row.label}" }
                        for value in row.cells.iter() {
                            td { "{format_tick(*value)}" }
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Treemap
// ---------------------------------------------------------------------------

/// One labelled tile of a treemap, sized by `value`.
#[derive(Clone, Debug, PartialEq)]
pub struct TreemapItem {
    pub label: String,
    pub value: f32,
}

impl TreemapItem {
    pub fn new(label: impl Into<String>, value: f32) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

/// Simple slice-and-dice layout: each item gets a horizontal strip whose
/// height is proportional to its share of the total value. Items are laid out
/// in the order given (callers may pre-sort by value).
fn treemap_layout(items: &[TreemapItem], plot: PlotRect) -> Vec<BarGeom> {
    let total: f32 = items.iter().map(|i| i.value.max(0.0)).sum();
    let mut out = Vec::with_capacity(items.len());
    if total <= 0.0 {
        return out;
    }
    let gap = 1.5;
    let mut y = plot.y;
    for item in items {
        let strip = plot.height * (item.value.max(0.0) / total);
        let height = (strip - gap).max(1.0);
        out.push(BarGeom {
            x: plot.x,
            y,
            width: plot.width,
            height,
        });
        y += strip;
    }
    out
}

/// One rendered treemap tile (geometry + label + share), precomputed for rsx.
struct TileDraw {
    series_index: usize,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    label: String,
    share: String,
}

/// Proportional tiles sized by value. A simple horizontal slice-and-dice
/// layout (pre-sort items by value for the usual largest-on-top look). Each
/// tile carries its label and share of the total; the values mirror in a
/// visually-hidden list. `progress` (0..1) scales tile heights for capture.
#[component]
pub fn Treemap(
    label: String,
    items: Vec<TreemapItem>,
    #[props(default = true)] animate: bool,
    #[props(default)] progress: Option<f32>,
) -> Element {
    const PLOT: PlotRect = PlotRect {
        x: 10.0,
        y: 10.0,
        width: 230.0,
        height: 160.0,
    };
    let tiles = treemap_layout(&items, PLOT);
    let scale = progress.map(clamp_progress);
    let animated = animate && scale.is_none();
    let root_class = format!(
        "ui-chart ui-chart--treemap{}",
        if animated { " ui-chart--animate" } else { "" }
    );
    let total: f32 = items.iter().map(|i| i.value.max(0.0)).sum();

    let draws: Vec<TileDraw> = tiles
        .iter()
        .enumerate()
        .map(|(index, tile)| {
            let height = scale.map_or(tile.height, |s| (tile.height * s).max(1.0));
            let item = &items[index];
            let share = if total > 0.0 {
                format!("{}%", (item.value / total * 100.0).round() as u32)
            } else {
                String::new()
            };
            TileDraw {
                series_index: index,
                x: tile.x,
                y: tile.y,
                width: tile.width,
                height,
                label: item.label.clone(),
                share,
            }
        })
        .collect();

    rsx! {
        figure { class: "{root_class}",
            svg {
                class: "ui-chart-canvas",
                view_box: "0 0 250 180",
                role: "img",
                "aria-label": "{label}",
                if items.is_empty() {
                    text { class: "ui-chart-empty", x: "125", y: "94", text_anchor: "middle", "No data" }
                }
                g { "aria-hidden": "true",
                    for d in draws.iter() {
                        rect {
                            class: "ui-chart-tile {series_class(d.series_index)}",
                            x: "{d.x}",
                            y: "{d.y}",
                            width: "{d.width}",
                            height: "{d.height}",
                            rx: "2.0",
                            style: if animated {
                                format!("animation-delay:{}ms", d.series_index * 50)
                            } else {
                                String::new()
                            },
                        }
                        text {
                            class: "ui-chart-tile-label",
                            x: "{d.x + 6.0}",
                            y: "{d.y + 14.0}",
                            "{d.label} {d.share}"
                        }
                    }
                }
            }
            {treemap_sr(&label, &items)}
        }
    }
}

fn treemap_sr(label: &str, items: &[TreemapItem]) -> Element {
    if items.is_empty() {
        return rsx! {};
    }
    rsx! {
        table { class: "visually-hidden",
            caption { "{label}" }
            tbody {
                for item in items.iter() {
                    tr {
                        th { scope: "row", "{item.label}" }
                        td { "{format_tick(item.value)}" }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLOT: PlotRect = PlotRect {
        x: 10.0,
        y: 10.0,
        width: 100.0,
        height: 100.0,
    };

    /// One series, one category — the simplest grouped-bar slot.
    const SINGLE_SLOT: BarSlot = BarSlot {
        series_index: 0,
        series_count: 1,
        point_index: 0,
        point_count: 1,
    };

    #[test]
    fn series_bounds_skips_non_finite() {
        let series = [ChartSeries::new(
            "a",
            vec![1.0, f32::NAN, 5.0, f32::INFINITY, -2.0],
        )];
        assert_eq!(series_bounds(&series), Some((-2.0, 5.0)));
    }

    #[test]
    fn series_bounds_empty_is_none() {
        assert_eq!(series_bounds(&[]), None);
        assert_eq!(
            series_bounds(&[ChartSeries::new("a", vec![f32::NAN])]),
            None
        );
    }

    #[test]
    fn nice_ticks_cover_domain_with_round_steps() {
        let ticks = nice_ticks(3.0, 97.0, 5);
        assert_eq!(ticks, vec![0.0, 20.0, 40.0, 60.0, 80.0, 100.0]);
    }

    #[test]
    fn nice_ticks_flat_series_pads_domain() {
        let ticks = nice_ticks(5.0, 5.0, 5);
        assert!(ticks.len() >= 2);
        assert!(*ticks.first().unwrap() <= 4.0);
        assert!(*ticks.last().unwrap() >= 6.0);
    }

    #[test]
    fn format_tick_trims_and_abbreviates() {
        assert_eq!(format_tick(40.0), "40");
        assert_eq!(format_tick(2.5), "2.5");
        assert_eq!(format_tick(1_500.0), "1.5k");
        assert_eq!(format_tick(2_000_000.0), "2M");
        assert_eq!(format_tick(-1_500.0), "-1.5k");
    }

    #[test]
    fn map_y_inverts_axis() {
        // Highest value maps to the plot top, lowest to the bottom.
        assert_eq!(map_y(10.0, 0.0, 10.0, PLOT), PLOT.y);
        assert_eq!(map_y(0.0, 0.0, 10.0, PLOT), PLOT.bottom());
    }

    #[test]
    fn line_path_needs_two_finite_points() {
        assert!(line_path(&[1.0], 0.0, 1.0, PLOT).is_none());
        assert!(line_path(&[1.0, f32::NAN], 0.0, 1.0, PLOT).is_none());
        let d = line_path(&[0.0, 10.0], 0.0, 10.0, PLOT).unwrap();
        assert!(d.starts_with('M'));
        assert!(d.contains(" L"));
    }

    #[test]
    fn area_path_closes_to_floor() {
        let d = area_path(&[0.0, 10.0], 0.0, 10.0, PLOT).unwrap();
        assert!(d.ends_with('Z'));
    }

    #[test]
    fn bar_geometry_measures_from_zero_baseline() {
        let bar = bar_geometry(SINGLE_SLOT, 10.0, 0.0, 10.0, PLOT).unwrap();
        assert_eq!(bar.y, PLOT.y);
        assert_eq!(bar.height, PLOT.height);
    }

    #[test]
    fn bar_geometry_negative_value_hangs_below_baseline() {
        let bar = bar_geometry(SINGLE_SLOT, -5.0, -5.0, 5.0, PLOT).unwrap();
        let baseline = map_y(0.0, -5.0, 5.0, PLOT);
        assert_eq!(bar.y, baseline);
        assert!(bar.height > 0.0);
    }

    #[test]
    fn bar_geometry_skips_non_finite() {
        assert!(bar_geometry(SINGLE_SLOT, f32::NAN, 0.0, 1.0, PLOT).is_none());
    }

    #[test]
    fn series_class_cycles_six_colors() {
        assert_eq!(series_class(0), "ui-chart-series--1");
        assert_eq!(series_class(5), "ui-chart-series--6");
        assert_eq!(series_class(6), "ui-chart-series--1");
    }

    #[test]
    fn clamp_progress_handles_garbage() {
        assert_eq!(clamp_progress(0.5), 0.5);
        assert_eq!(clamp_progress(-1.0), 0.0);
        assert_eq!(clamp_progress(2.0), 1.0);
        assert_eq!(clamp_progress(f32::NAN), 1.0);
    }

    #[test]
    fn chart_tone_class_suffixes() {
        assert_eq!(ChartTone::Primary.class_suffix(), "primary");
        assert_eq!(ChartTone::Neutral.class_suffix(), "neutral");
        assert_eq!(ChartTone::default(), ChartTone::Primary);
    }

    #[test]
    fn finite_max_skips_non_finite() {
        assert_eq!(
            finite_max(&[1.0, f32::NAN, 3.0, f32::INFINITY, -2.0]),
            Some(3.0)
        );
        assert_eq!(finite_max(&[f32::NAN, f32::INFINITY]), None);
        assert_eq!(finite_max(&[]), None);
    }

    #[test]
    fn funnel_bar_is_centered_and_proportional() {
        // A single max-value stage spans the full plot width and is centered.
        let bar = funnel_bar(0, 1, 10.0, 10.0, PLOT);
        assert_eq!(bar.x, PLOT.x);
        assert!((bar.width - PLOT.width).abs() < 0.5);
        // Half-value stage is half width and centered within the plot.
        let half = funnel_bar(0, 1, 5.0, 10.0, PLOT);
        assert!((half.width - PLOT.width / 2.0).abs() < 0.5);
        assert!((half.x - (PLOT.x + PLOT.width / 4.0)).abs() < 0.5);
    }

    #[test]
    fn heatmap_range_and_intensity() {
        let rows = [HeatmapRow::new("a", vec![0.0, 5.0, 10.0])];
        assert_eq!(heatmap_range(&rows), Some((0.0, 10.0)));
        assert_eq!(intensity(0.0, 0.0, 10.0), 0.0);
        assert_eq!(intensity(10.0, 0.0, 10.0), 1.0);
        assert!((intensity(5.0, 0.0, 10.0) - 0.5).abs() < 1e-6);
        assert_eq!(heatmap_range(&[]), None);
    }

    #[test]
    fn treemap_layout_sums_to_plot_height() {
        let items = [TreemapItem::new("a", 1.0), TreemapItem::new("b", 3.0)];
        let tiles = treemap_layout(&items, PLOT);
        assert_eq!(tiles.len(), 2);
        // Larger item gets the taller strip.
        assert!(tiles[1].height > tiles[0].height);
        let total: f32 = tiles.iter().map(|t| t.height).sum();
        // Each tile loses up to `gap` (1.5) of height, so the strip heights
        // fall short of the plot by less than 2px per tile.
        assert!((PLOT.height - total) < (items.len() as f32 * 2.0));
    }

    #[test]
    fn treemap_layout_empty_when_no_value() {
        let tiles = treemap_layout(
            &[TreemapItem::new("a", 0.0), TreemapItem::new("b", 0.0)],
            PLOT,
        );
        assert!(tiles.is_empty());
    }
}
