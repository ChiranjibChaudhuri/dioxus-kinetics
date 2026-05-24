# Dioxus Kinetics — AI Agent Cheatsheet

A reference for AI agents and LLMs building SaaS frontends with this
workspace. Optimized for copy-pasteable patterns, exact import paths,
and gotchas that aren't obvious from API signatures.

---

## TL;DR

- **What it is**: A Rust UI library on top of Dioxus 0.7, designed for
  semantic glass-style SaaS interfaces. Ships 45 components across
  Foundations, Actions, Inputs, Navigation, Layout, Surfaces, Feedback,
  Data Workflows, Motion, Composition, and Capture.
- **Single dependency**: `kinetics`. Pull
  `kinetics::prelude::*` and you have everything (components, tokens,
  motion primitives, icons, runtime hooks).
- **Two name surfaces** per component (both stable on the 0.1.x line):
  - Standard names: `Button`, `TextField`, `Dialog`, `Toast`, …
  - Functional / SaaS-role names: `ActionControl`, `TextEntry`,
    `ModalLayer`, `NoticeStack`, … (see `docs/component-naming.md`).
  - Pick one and stick to it within an app for readability. The
    standard names match React/Dioxus convention; the functional
    names match Material/Carbon-style design-system docs.
- **Glass rendering**: Most surfaces have two render paths — a wgpu
  engine (default, requires `liquid-glass` feature) and a CSS
  fallback. Auto tier detection picks per host; `force_css: true`
  overrides per-instance.

---

## Setup

```toml
# Cargo.toml
[dependencies]
dioxus = "0.7"
kinetics = { git = "https://github.com/ChiranjibChaudhuri/dioxus-kinetics" }
```

```rust
// main.rs (or app.rs)
use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
fn App() -> Element {
    // Inject the library stylesheet once at the app root. `library_css()`
    // returns base tokens + component CSS as a single string.
    rsx! {
        style { "{library_css()}" }
        // …your routes…
    }
}
```

Theme switching uses `data-ui-theme` and `data-ui-glass-policy`
attributes on a wrapping element:

```rust
div {
    "data-ui-theme": if dark { "dark" } else { "light" },
    "data-ui-glass-policy": if reduced_transparency { "solid" } else { "auto" },
    // children
}
```

---

## Component quick-reference

Standard name → functional alias (if any) → category. Every component
is in `kinetics::prelude::*`.

### Foundations
- `Surface` / `ContentPlane` — neutral content container.
- `GlassSurface` / `GlassLayer` — glass-styled container; auto picks
  wgpu LiquidSurface or CSS fallback based on device tier.
- `LiquidSurface` — direct wgpu glass canvas (no auto-fallback).

### Actions
- `Button` / `ActionControl` — primary/secondary/ghost/danger variants.
- `IconButton` — 3 tones × 3 sizes matrix.
- `CommandMenu` / `CommandFinder` — searchable command palette.
- `DropdownMenu` / `ActionMenu` — anchored `role="menu"` overlay.
- `Toolbar` / `ActionBar` — horizontal command group.

### Inputs
- `TextField` / `TextEntry` — text input with help/error/leading/trailing slots.
- `Checkbox` / `ChoiceMark` — boolean + indeterminate.
- `Switch` / `StateSwitch` — `role="switch"` toggle.
- `Slider` — native range with `aria-valuetext`.
- `SegmentedControl` — button-group radio (short choices).
- `RadioGroup` / `OptionGroup` — native `<input type=radio>` group
  with descriptions (long-form choices).
- `Select` — popover listbox.
- `Combobox` — typeahead-filtered Select.
- `DatePicker` — calendar-grid date picker.
- `DataTable` — `<table>` with sortable headers + stable row keys.

### Navigation
- `Breadcrumb` — wayfinding trail.
- `Pagination` — offset-style page jumper.
- `Stepper` — multi-step workflow indicator.
- `Tabs` / `ViewSwitcher` — tab list + panel.
- `Sidebar` / `NavigationRail` — sectioned vertical nav.

### Layout
- `Stack` — vertical stack with `gap` token.
- `Accordion` — disclosure pattern.

### Surfaces
- `MetricCard` / `MetricReadout` — KPI tile with tone + delta.
- `Popover` — anchored overlay primitive.

### Feedback
- `Alert` — page-level banner, 5 tones.
- `Progress` — determinate + indeterminate.
- `Skeleton` — loading placeholder.
- `EmptyState` / `BlankState` — empty-data CTA.
- `Dialog` / `ModalLayer` — modal panel.
- `Toast` / `NoticeStack` — transient notification.
- `Tooltip` / `ContextHint` — hover/focus tooltip.

### Motion
- `Presence`, `PresenceGate`, `PresenceCue` — enter/exit animation.
- `KineticBox`, `KineticText` — cue-driven motion containers.
- `Sequence`, `SequenceContext`, `Cue` — multi-stage motion.
- `TimelineScope` — timeline orchestrator.

### Composition
- `FrameStage`, `FrameClip`, `FrameLayer` — cinematic composition.
- `SharedElement`, `SharedLayout` — FLIP cross-tree transitions.

### Capture
- `CaptureStage` — viewport capture region for design exports.

---

## SaaS patterns (copy-paste ready)

### Login form

```rust
#[component]
fn LoginForm() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut remember = use_signal(|| true);
    let invalid = email.read().is_empty();

    rsx! {
        Surface {
            Stack { gap: "md".to_string(),
                TextField {
                    id: "login-email",
                    label: "Email",
                    value: email.read().clone(),
                    input_type: TextFieldType::Email,
                    autocomplete: "email".to_string(),
                    required: true,
                    invalid,
                    error_text: if invalid { "Email is required.".to_string() } else { String::new() },
                    oninput: move |evt: FormEvent| email.set(evt.value()),
                }
                TextField {
                    id: "login-password",
                    label: "Password",
                    value: password.read().clone(),
                    input_type: TextFieldType::Password,
                    autocomplete: "current-password".to_string(),
                    required: true,
                    oninput: move |evt: FormEvent| password.set(evt.value()),
                }
                Switch {
                    id: "login-remember",
                    label: "Remember me",
                    checked: *remember.read(),
                    onchange: move |next: bool| remember.set(next),
                }
                Button { variant: ButtonVariant::Primary, "Sign in" }
            }
        }
    }
}
```

### Data table with sort + pagination

```rust
#[component]
fn WorkspaceTable() -> Element {
    let mut sort_key = use_signal(|| "revenue".to_string());
    let mut sort_dir = use_signal(|| SortDirection::Descending);
    let mut page = use_signal(|| 1u32);

    let columns = vec![
        DataTableColumn::new("workspace", "Workspace"),
        DataTableColumn::new("revenue", "Revenue").sortable(),
        DataTableColumn::new("seats", "Seats").sortable(),
    ];
    // IMPORTANT: stable `id` per row so sort/reorder diffs correctly.
    let rows = vec![
        DataTableRow::new("acme",    vec!["Acme".into(),    "$12,400".into(), "48".into()]),
        DataTableRow::new("globex",  vec!["Globex".into(),  "$9,820".into(),  "32".into()]),
        DataTableRow::new("initech", vec!["Initech".into(), "$7,310".into(),  "21".into()]),
    ];

    rsx! {
        Stack { gap: "md".to_string(),
            DataTable {
                columns,
                rows,
                caption: "Top workspaces",
                sort_key: sort_key.read().clone(),
                sort_direction: *sort_dir.read(),
                on_sort: move |key: String| {
                    let same = *sort_key.read() == key;
                    sort_dir.set(if same {
                        match *sort_dir.read() {
                            SortDirection::Ascending => SortDirection::Descending,
                            _ => SortDirection::Ascending,
                        }
                    } else { SortDirection::Ascending });
                    sort_key.set(key);
                },
            }
            Pagination {
                page: *page.read(),
                total_pages: 12,
                on_select: move |p: u32| page.set(p),
            }
        }
    }
}
```

### Searchable filter (Combobox)

```rust
#[component]
fn TicketFinder() -> Element {
    let mut value = use_signal(String::new);
    let mut query = use_signal(String::new);
    let options = vec![
        ComboboxOption::new("ord-1204", "ORD-1204 — Acme"),
        ComboboxOption::new("ord-1203", "ORD-1203 — Globex"),
        ComboboxOption::new("ord-1130", "ORD-1130 — Initech"),
    ];
    rsx! {
        Combobox {
            id: "ticket-finder",
            label: "Find a ticket",
            value: value.read().clone(),
            query: query.read().clone(),
            options,
            on_query: move |q: String| query.set(q),
            on_select: move |v: String| value.set(v),
        }
    }
}
```

### Action menu (per-row kebab)

```rust
#[component]
fn RowActions(row_id: String) -> Element {
    let items = vec![
        DropdownMenuItem::new("rename", "Rename"),
        DropdownMenuItem::new("duplicate", "Duplicate"),
        DropdownMenuItem::separator("div-1"),
        DropdownMenuItem::new("archive", "Archive").disabled(),
        DropdownMenuItem::new("delete", "Delete"),
    ];
    rsx! {
        DropdownMenu {
            id: format!("row-{row_id}-actions"),
            trigger: rsx! {
                IconButton {
                    label: "Row actions",
                    tone: IconButtonTone::Neutral,
                    size: IconButtonSize::Compact,
                    ChevronDown { size: 16 }
                }
            },
            items,
            on_select: move |action: String| {
                // dispatch by action id
                let _ = action;
            },
        }
    }
}
```

### Confirm-and-archive modal

```rust
#[component]
fn ArchiveDialog(open: Signal<bool>, workspace: String) -> Element {
    let mut open = open;
    rsx! {
        Dialog {
            open: *open.read(),
            title: "Archive workspace",
            description: format!("Move {workspace} out of active navigation."),
            body: "Team members can still request access later.",
            actions: vec![
                DialogAction::ghost("cancel", "Cancel"),
                DialogAction::primary("archive", "Archive"),
            ],
            on_dismiss: move |_| open.set(false),
            on_action: move |action: String| {
                if action == "archive" {
                    // call API
                }
                open.set(false);
            },
        }
    }
}
```

### Toast notification stack

```rust
#[component]
fn ExportFeedback() -> Element {
    let mut toasts: Signal<Vec<(u32, ToastTone, &'static str, &'static str)>> =
        use_signal(Vec::new);
    let mut next_id = use_signal(|| 0u32);

    let mut push = move |tone: ToastTone, title: &'static str, desc: &'static str| {
        let id = *next_id.read();
        next_id.set(id + 1);
        toasts.write().push((id, tone, title, desc));
    };

    rsx! {
        Stack { gap: "md".to_string(),
            Button {
                variant: ButtonVariant::Primary,
                onclick: move |_| push(ToastTone::Success, "Export ready", "Download started."),
                "Export"
            }
            for (id, tone, title, desc) in toasts.read().iter().cloned() {
                Toast { key: "{id}", tone, title, description: desc, dismiss_label: "Dismiss" }
            }
        }
    }
}
```

### Tabs + Sidebar shell

```rust
#[component]
fn AppShell() -> Element {
    let mut section = use_signal(|| "billing".to_string());
    let mut tab = use_signal(|| "overview".to_string());

    rsx! {
        div { style: "display: grid; grid-template-columns: 240px 1fr; gap: 16px;",
            Sidebar {
                selected: section.read().clone(),
                sections: vec![SidebarSection::new("Workspace", vec![
                    SidebarItem::new("home", "Home", "#home"),
                    SidebarItem::new("billing", "Billing", "#billing"),
                    SidebarItem::new("members", "Team", "#members"),
                ])],
                onnavigate: move |id: String| section.set(id),
            }
            Surface {
                Tabs {
                    selected: tab.read().clone(),
                    items: vec![
                        TabItem::new("overview", "Overview"),
                        TabItem::new("invoices", "Invoices"),
                        TabItem::new("usage", "Usage"),
                    ],
                    panels: vec![
                        TabPanel::new("overview", "Account summary"),
                        TabPanel::new("invoices", "Recent invoices"),
                        TabPanel::new("usage", "Quota: 92%"),
                    ],
                    onselect: move |t: String| tab.set(t),
                }
            }
        }
    }
}
```

### Empty + loading states

```rust
#[component]
fn ReportList(loading: bool, items: Vec<String>) -> Element {
    if loading {
        return rsx! {
            Stack { gap: "sm".to_string(),
                Skeleton { height: "20px".to_string(), width: "60%".to_string(), radius: "6px".to_string() }
                Skeleton { height: "12px".to_string(), width: "100%".to_string(), radius: "4px".to_string() }
                Skeleton { height: "12px".to_string(), width: "85%".to_string(), radius: "4px".to_string() }
            }
        };
    }
    if items.is_empty() {
        return rsx! {
            EmptyState {
                title: "No reports yet",
                description: "Create your first report to share metrics with the team.",
                action_label: "Create report",
            }
        };
    }
    rsx! {
        Stack { gap: "sm".to_string(),
            for item in items {
                p { "{item}" }
            }
        }
    }
}
```

### Multi-step workflow (Stepper + form)

```rust
#[component]
fn CheckoutFlow() -> Element {
    let mut step = use_signal(|| "plan".to_string());
    let steps = vec![
        StepperStep::new("plan", "Plan").with_description("Pick a tier"),
        StepperStep::new("payment", "Payment").with_description("Card details"),
        StepperStep::new("review", "Review").with_description("Confirm"),
    ];
    rsx! {
        Stack { gap: "lg".to_string(),
            Stepper { steps, current: step.read().clone(),
                on_select: move |s: String| step.set(s),
            }
            // render step body based on `*step.read()`
        }
    }
}
```

---

## Tokens reference

```rust
// Glass
GlassLevel::{Subtle, Floating, Overlay, Chrome}
GlassTone::{Neutral, Primary, Success, Warning, Danger, Info}
GlassDensity::{Compact, Comfortable, Spacious}

// Button
ButtonVariant::{Primary, Secondary, Ghost, Danger}

// IconButton
IconButtonTone::{Neutral, Primary, Danger}
IconButtonSize::{Compact, Default, Spacious}

// Alert / Toast / Metric
AlertTone::{Neutral, Success, Warning, Danger, Info}
ToastTone::{Neutral, Success, Warning, Danger, Info}
MetricTone::{Neutral, Success, Warning, Danger, Info}

// Forms
TextFieldType::{Text, Email, Password, Number, Search, Tel, Url}

// Tables
SortDirection::{None, Ascending, Descending}

// Popover/overlays
PopoverSide::{Top, Bottom, Start, End}

// Dialog actions
DialogActionTone::{Primary, Ghost, Danger}
// Helper constructors: DialogAction::primary(id, label), .ghost(...), .danger(...)
```

---

## Motion primitives

```rust
// Single-component enter/exit
Presence {
    present: true,                 // toggle to drive
    enter: Transition::Spring(Spring::snappy()),
    exit: Transition::Spring(Spring::soft()),
    cue: PresenceCue::Rise,        // Rise | Scale | Fade | None
    div { /* content */ }
}

// Coordinated multi-element entrance
Sequence {
    cues: Some(vec![
        Cue::new("title", 0.0, MotionCue::Opacity { from: 0.0, to: 1.0,
            transition: Transition::Tween { duration_ms: 220, ease: Ease::Standard } }),
        Cue::new("body", 120.0, MotionCue::Translate { axis: Axis::Y, from: 12.0, to: 0.0,
            transition: Transition::Tween { duration_ms: 200, ease: Ease::Standard } }),
    ]),
    clock: TimelineClock::Auto,    // or Manual { elapsed_ms } for scrubbing
    KineticBox { id: "title", h4 { "Welcome" } }
    KineticBox { id: "body", p { "Subtitle" } }
}

// Cue-driven primitives (cues are tokens defined in the host stylesheet)
KineticBox { id: "card", cue: "rise-in".to_string(), /* content */ }
KineticText { id: "headline", text: "Hello".to_string(), cue: "fade-in".to_string() }

// FLIP transitions across tree positions
SharedLayout {
    SharedElement { id: "card".to_string(), /* content */ }
    // …somewhere else in the same SharedLayout subtree…
}
```

---

## Runtime hooks (feature `runtime`)

```rust
use_reduced_motion()         // -> ReducedMotion
use_element_rect(node_ref)   // -> Rect of mounted element
use_element_computed_style(node_ref, prop) // -> String of computed CSS
use_animation_value(spec)    // -> f32 driven by Spring/Tween
use_presence_animation(state) // -> ResolvedMotionState for transitions
use_shared_element_registry() // -> SharedElementRegistry
use_timeline_sample(scope, t) // -> sampled state at time t
```

---

## Icons (feature `icons`)

```rust
Close { size: 16 }
Check { size: 20 }
ChevronDown { size: 14 }
ChevronRight { size: 14 }
Plus { size: 16 }
Minus { size: 16 }
Trash { size: 16 }
Search { size: 18 }
```

---

## Critical patterns AI should follow

1. **Stable row IDs in `DataTable`.** Always pass meaningful ids on
   `DataTableRow::new`. Index-based keys cause cell-level diffs to
   misfire when the user sorts the table.

2. **Controlled overlay state for previews.** `Select`, `DatePicker`,
   `Combobox`, `DropdownMenu` all accept `default_open: bool`. Use it
   for SSR snapshots, docs/showcases, or when you need the listbox
   visible at first paint. In normal app code leave it off; the trigger
   handles open/close.

3. **`force_css` for showcase grids.** Rendering ≥3 `GlassSurface`
   instances on one page risks exceeding webkit's per-page WebGL
   context cap. For pure design-token showcases (e.g. the gallery's
   3×3 level/tone grid), pass `force_css: true` so the CSS-only
   render path is used. Tone and level are still visually
   differentiated via the stylesheet.

4. **Stable IDs for all overlays.** `Dialog`, `Popover`,
   `CommandMenu`, `Combobox`, `DropdownMenu` etc. all take an `id`
   that wires up `aria-controls` and `aria-labelledby`. Use stable,
   unique ids per instance.

5. **`Tooltip` requires `visible: bool`.** It's controlled — you own
   the hover/focus state. Pair with `onmouseenter` / `onfocusin` on
   a wrapping element. (Native HTML `title` attribute is not used.)

6. **Toast lifecycle is host-driven.** `Toast` renders one
   notification. The stack + auto-dismiss is your responsibility:
   push into a `Signal<Vec<…>>`, set up a `spawn(async move { sleep;
   remove })`. See the gallery's `feedback.rs::ToastPreviewBody` for
   the canonical pattern.

7. **`Combobox` query is fully controlled.** `query` and `value` are
   independent props. The host listens on `on_query` to filter and
   `on_select` to commit. The pure helper `filter_options(&opts,
   &query)` is exposed for custom matching strategies.

8. **`RadioGroup` requires a shared `name`** — that's how the browser
   enforces native mutual exclusion. Different `RadioGroup` instances
   on the same page must use different `name` values.

9. **`SegmentedControl` is for short equal-weight choices**;
   `RadioGroup` is for long-form options with descriptive copy.
   Don't conflate them.

---

## Anti-patterns to avoid

- **Don't render >3 `GlassSurface` instances on a single page** unless
  they all pass `force_css: true`. WebGL contexts get dropped on
  webkit past the per-page cap and tiles render dark.
- **Don't put any `LiquidSurface` inside a hidden tab/panel** that
  might mount/unmount on click. WebGL contexts are expensive to
  init; prefer `GlassSurface` (auto-fallback) inside tabs.
- **Don't bypass `kinetics::prelude` to import individual sub-crates**
  directly (e.g. `ui_dioxus::Button`). The prelude is the stable
  surface; sub-crate paths can be reorganized between minor versions.
- **Don't use `format!("{:?}", value)`** for accessible labels. The
  components already wire `aria-label`/`aria-labelledby` from
  documented props — feed them human-readable strings.
- **Don't disable `liquid-glass` feature in production** unless you've
  measured your tier mix. The CSS fallback works but loses the
  vibrant glass effect on capable hardware.

---

## Component naming policy

When in doubt about which name to use, follow these rules:
1. **Code that ships to other Dioxus/React developers**: standard
   names (`Button`, `TextField`, `Dialog`).
2. **Product docs and design specs**: functional names
   (`ActionControl`, `TextEntry`, `ModalLayer`) read more naturally.
3. **Within one app**: pick one set and never mix. The
   `crates/kinetics/tests/prelude.rs` test pins both surfaces so
   neither can drift away from the other.

---

## Feature flags

```toml
[dependencies.kinetics]
git = "https://github.com/ChiranjibChaudhuri/dioxus-kinetics"
default-features = false
features = [
    "web",           # Dioxus web target (default)
    "desktop",       # Dioxus desktop
    "mobile",        # Dioxus mobile
    "native",        # Native glass via ui-native
    "timeline",      # Timeline runtime + helpers
    "runtime",       # use_* hooks for measurement/animation
    "composition",   # FrameStage + Composition
    "capture",       # CaptureStage descriptor
    "icons",         # Icon set
    "liquid-glass",  # wgpu glass engine (recommended)
]
```

---

## Where to look in the source

- **Component implementations**: `crates/ui-dioxus/src/`
- **Foundational types**: `crates/ui-core/src/lib.rs`
- **Glass material types**: `crates/ui-glass/src/lib.rs`
- **Motion primitives**: `crates/ui-motion/`, `crates/ui-timeline/`
- **Runtime hooks**: `crates/ui-runtime/src/`
- **Stylesheet**: `crates/ui-styles/src/lib.rs` (`COMPONENT_CSS`)
- **Prelude truth**: `crates/kinetics/src/lib.rs::prelude`
- **Gallery preview patterns**: `examples/component-gallery/src/previews/`

---

## Useful links inside this repo

- `docs/component-naming.md` — full standard ↔ functional table
- `docs/glass-materials.md` — material recipes + tier ladder
- `docs/platform-support.md` — target matrix
- `examples/component-gallery/` — every component rendered in 4
  variants, served via `dx serve` or built via `dx build --release`
