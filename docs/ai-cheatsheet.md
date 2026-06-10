# Dioxus Kinetics — AI Agent Cheatsheet

A reference for AI agents and LLMs building SaaS frontends with this
workspace. Optimized for copy-pasteable patterns, exact import paths,
and gotchas that aren't obvious from API signatures.

---

## TL;DR

- **What it is**: A Rust UI library on top of Dioxus 0.7, designed for
  semantic glass-style SaaS interfaces. Ships 60+ components across
  Foundations, Actions, Inputs, Navigation, Layout, Surfaces, AI,
  Feedback, Data Workflows, Motion, Composition, Capture, and Scene —
  including a dedicated **AI-native surfaces** family (streaming text,
  status pills, citations, source cards/rails, a prompt composer, a
  docked assistant panel, and an agent timeline).
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
- `Heading` — semantic `h1..h6` chosen by a numeric `level`, with an
  optional `TextVariant` visual override (level and visual size are
  decoupled).
- `Text` — body / inline text on the `TextVariant` type scale;
  `as_element` picks the `p` / `span` / `div` tag.
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
- `Badge` — inline status pill; 6 tones (`BadgeTone`), single size.
- `Avatar` — circular image avatar with a derived-initials fallback;
  3 sizes (`AvatarSize`).
- `Popover` — anchored overlay primitive.

### AI-native
- `StreamingText` — incremental assistant output: a settled prefix, a
  freshly-faded tail token, and a blinking caret (`role="status"`,
  polite live region). `chunk_boundaries` are **byte** offsets.
- `AiStatus` — status pill driven by `AiStatusState`
  (idle / thinking / searching / generating → dots, done → check).
  One pill = one state; render several for a phase sequence.
- `CitationChip` — numbered inline source reference; a real `<a>` when
  given `href`, a non-navigating `<button>`-role chip when not.
- `SourceCard` / `SourceRail` — search-result source cards (favicon,
  title, domain, snippet) wrapped in an ARIA `list` rail.
- `PromptInput` — auto-growing chat composer; Enter submits,
  Shift+Enter inserts a newline, the send button flips to **Stop**
  while `streaming`. Fully controlled (`value` + `on_input`).
- `AssistantPanel` — **non-modal** docked side panel shell
  (`AssistantSide`) that hosts the surfaces above; dismiss via close
  button or Escape. No backdrop, no focus trap.
- `AgentTimeline` — vertical agent-run timeline from `Vec<AgentStep>`
  (`AgentStepState`: pending / active / done); the active step is
  `aria-current="step"`.
- `Waveform` / `AudioLevels` — audio level trace from plain `levels`
  props (0.0–1.0 per bar); `active` pulses the bars (stilled under
  reduced motion). Decorative unless given a `label`.
- `VoiceInput` — push-to-talk composer: mic toggle, live `Waveform`,
  `elapsed` readout, and a `VoiceInputState` lifecycle
  (idle / recording / processing / error). Fully controlled — the host
  owns the audio pipeline; errors announce via `role="alert"`.

### Charts
- `Sparkline` / `TrendLine` — compact axis-free trend line, 6 tones
  (`ChartTone`), optional `filled` area. Decorative without a `label`.
- `LineChart` / `TrendChart` — multi-series lines from
  `Vec<ChartSeries>`, nice-number grid, legend, optional area. The SVG
  is `role="img"`; the data is mirrored in a visually-hidden table.
- `BarChart` / `ComparisonChart` — grouped bars, zero-anchored domain,
  staggered rise-in.
- `DonutGauge` / `ProgressDial` — radial KPI gauge (`role="meter"`),
  center readout via `display_value` / `description`.
- All four animate via CSS by default; pass `progress: Some(t)` to pin
  the draw-in deterministically (Scene clocks, capture), or
  `animate: false` for no motion.

### Sortable surfaces
- `SortableList` / `ReorderList` — controlled reorderable list of
  `SortableItem`s; emits the full new id order via `on_reorder`. Drag
  or keyboard (Space grabs, arrows move, Escape restores).
- `KanbanBoard` / `WorkflowBoard` — multi-column board of
  `KanbanColumn`s; emits `KanbanMove` via `on_move` — apply it with
  `apply_kanban_move(&columns, &mv)`.

### Guidance
- `Tour` / `GuidedTour` — controlled step-by-step product tour:
  `Vec<TourStep>` (each optionally `.with_target(dom_id)` and
  `.with_placement(TourPlacement)`), `active` index + `on_change`,
  `on_dismiss` for Skip/Escape/scrim/Done. Focus is trapped in the
  callout and restored on dismiss.
- `Spotlight` — the scrim-with-cutout primitive `Tour` uses; exported
  for custom guidance surfaces. Tracks `target_id` through resize and
  scroll; children render above the scrim.

### Feedback
- `Alert` — page-level banner, 5 tones.
- `Progress` — determinate + indeterminate.
- `Spinner` — indeterminate loading spinner (`role="status"` + label).
- `Skeleton` — loading placeholder.
- `EmptyState` / `BlankState` — empty-data CTA.
- `Dialog` / `ModalLayer` — modal panel.
- `Sheet` — modal side sheet / drawer (`SheetSide`) with focus trap +
  opener-focus restoration.
- `Toast` / `NoticeStack` — a single transient notification card.
- `Toaster` — fixed-position toast **stack** driven by
  `Vec<ToastEntry>` with per-entry auto-dismiss (hover pauses).
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

### AI assistant surfaces (AssistantPanel composing the AI family)

```rust
// A docked assistant panel composing the AI-native surfaces. The panel is
// NON-MODAL (no backdrop, no focus trap) — the rest of the page stays
// interactive. The panel is just a shell; YOU supply the surfaces as children.
#[component]
fn WorkspaceAssistant() -> Element {
    let mut open = use_signal(|| true);
    let mut prompt = use_signal(String::new);
    let streaming = use_signal(|| true);

    rsx! {
        AssistantPanel {
            open: *open.read(),                 // false → renders nothing
            side: AssistantSide::End,           // dock to inline-end (right in LTR)
            title: "Workspace assistant",       // also the aria-label
            on_dismiss: move |_| open.set(false), // fired by close button AND Escape

            // Phase pill — render ONE AiStatus for the current phase.
            AiStatus { state: AiStatusState::Generating, label: "Generating answer…".to_string() }

            // Streaming answer. `chunk_boundaries` are BYTE offsets; the
            // largest in-range one splits the settled prefix from the
            // highlighted tail token. `streaming` toggles the caret.
            StreamingText {
                text: "Revenue grew 18% QoQ, led by enterprise renewals".to_string(),
                streaming: *streaming.read(),
                chunk_boundaries: vec![38],
            }

            // Sources as an ARIA list of cards. A card with `href` is a link
            // (<a target="_blank">); without `href` it is a static <article>.
            SourceRail {
                SourceCard {
                    index: 1,
                    title: "Q3 revenue report",
                    domain: "docs.internal",
                    snippet: "Enterprise renewals drove the majority of QoQ growth.",
                    href: "https://docs.internal/q3",
                }
                SourceCard { index: 2, title: "Pipeline dashboard", domain: "app.internal" }
            }

            // Controlled composer. Enter submits; the send button flips to
            // Stop while `streaming`. You own `value` via `on_input`.
            PromptInput {
                value: prompt.read().clone(),
                streaming: *streaming.read(),
                placeholder: "Ask about this workspace…",
                on_input: move |next: String| prompt.set(next),
                on_submit: move |_text: String| prompt.set(String::new()), // clear yourself
                on_stop: move |_| { /* cancel the in-flight stream */ },
            }
        }
    }
}
```

### Agent run timeline

```rust
// `steps` is a Vec<AgentStep> DATA prop (not children). Build with
// AgentStep::new(label, state). The active step gets aria-current="step".
AgentTimeline {
    steps: vec![
        AgentStep::new("Parse the request", AgentStepState::Done),
        AgentStep::new("Search the knowledge base", AgentStepState::Done),
        AgentStep::new("Synthesise an answer", AgentStepState::Active),
        AgentStep::new("Cite sources", AgentStepState::Pending),
        AgentStep::new("Deliver response", AgentStepState::Pending),
    ],
}
```

### Inline citations (CitationChip in prose)

```rust
// CitationChip renders inline with no wrapper — place it directly in text.
// With `href` → <a target="_blank" rel="noopener noreferrer">. Without
// `href` → a non-navigating <button>-role chip (wire your own popover).
// `href` is an empty-String sentinel, NOT Option.
p {
    "Ownership prevents data races at compile time"
    CitationChip { index: 1, title: "The Rust Reference", href: "https://doc.rust-lang.org/reference/" }
    " and is enforced by the borrow checker"
    CitationChip { index: 2, title: "Rust Book · Ownership", href: "https://doc.rust-lang.org/book/" }
    "."
    // No href → button chip, e.g. to open a source-preview popover.
    CitationChip { index: 3, title: "Internal scheduler notes" }
}
```

### Side sheet / drawer (Sheet)

```rust
// Modal: owns the backdrop, Escape-to-dismiss, the close button, and a
// focus trap keyed by `id`. Fully controlled — when `open` is false the
// component renders an empty tree, so hold the flag yourself.
#[component]
fn FiltersSheet() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { variant: ButtonVariant::Primary, onclick: move |_| open.set(true), "Edit filters" }
        Sheet {
            open: *open.read(),
            side: SheetSide::End,               // inline-end (right in LTR)
            title: "Edit filters",              // also the panel aria-label
            // id: "filters-sheet",             // set a unique id if >1 sheet can be open at once
            on_dismiss: move |_| open.set(false), // EventHandler<()> — no payload
            Stack { gap: "md".to_string(),
                TextField { id: "filter-name", label: "Name contains", value: String::new() }
                Button { variant: ButtonVariant::Primary, "Apply" }
            }
        }
    }
}
```

### Auto-dismissing toast stack (Toaster)

```rust
// Unlike manually stacking `Toast` cards (see "Toast notification stack"
// above), `Toaster` auto-dismisses each entry — hover pauses the countdown.
// It is RENDER-DRIVEN: you own the Vec<ToastEntry>. To add a toast, push to
// your signal; you MUST remove the id inside on_dismiss or it never clears.
#[component]
fn ExportToasts() -> Element {
    let mut entries = use_signal(|| vec![
        ToastEntry::new("export", "Report exported")
            .with_tone(ToastTone::Success)
            .with_description("The PDF is ready to download."),
    ]);
    let mut next = use_signal(|| 0u32);

    rsx! {
        Button {
            variant: ButtonVariant::Primary,
            onclick: move |_| {
                let id = *next.read();
                next.set(id + 1);
                entries.write().push(
                    ToastEntry::new(format!("sync-{id}"), "Sync started").with_tone(ToastTone::Info)
                );
            },
            "Start sync"
        }
        Toaster {
            items: entries.read().clone(),
            duration_ms: 5000,                  // Option<u32>; omit for the 5000ms default
            on_dismiss: move |id: String| { entries.write().retain(|e| e.id != id); },
        }
    }
}
```

### Typography & identity (Heading / Text / Badge / Avatar)

```rust
Stack { gap: "sm".to_string(),
    // `level` fixes the semantic h1..h6; `variant` overrides ONLY the size.
    Heading { level: 1, "Quarterly performance" }
    Heading { level: 2, variant: TextVariant::Display, "Display-sized, still an <h2>" }

    // `Text` never emits hN — `as_element` (a String) picks p / span / div.
    Text { variant: TextVariant::Body, "Default reading size for prose." }
    Text { variant: TextVariant::Caption, as_element: "span".to_string(), "Smallest legible annotation." }

    div { style: "display: flex; gap: 8px; align-items: center;",
        Avatar { name: "Ada Lovelace", size: AvatarSize::Sm } // no src → derived "AL" initials
        Badge { tone: BadgeTone::Success, "Active" }
        Badge { tone: BadgeTone::Danger, "Down" }
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

// Alert / Toast / Metric (Danger/Warning render role="alert"; others role="status")
AlertTone::{Neutral, Success, Warning, Danger, Info}
ToastTone::{Neutral, Success, Warning, Danger, Info}
MetricTone::{Neutral, Success, Warning, Danger, Info}

// Typography (Heading + Text share this scale; default = Body)
TextVariant::{Caption2, Caption, Footnote, Subhead, Callout, Body,
              Headline, Title3, Title2, Title1, LargeTitle, Display}
//   Heading.variant is Option<TextVariant> (default None → derived from
//   level: 1→Title1, 2→Title2, 3→Title3, 4..→Body).
//   Text.variant is a bare TextVariant (default Body). Don't copy the type.

// Badge / Avatar
BadgeTone::{Neutral, Primary, Success, Warning, Danger, Info} // default Neutral; Neutral has no modifier class
AvatarSize::{Sm, Md, Lg}                                       // default Md

// Forms
TextFieldType::{Text, Email, Password, Number, Search, Tel, Url}

// Tables
SortDirection::{None, Ascending, Descending}

// Popover/overlays
PopoverSide::{Top, Bottom, Start, End}

// Sheet (side sheet / drawer)
SheetSide::{Start, End}  // default End (inline-end / right in LTR)

// AI-native surfaces
AiStatusState::{Idle, Thinking, Searching, Generating, Done}  // default Idle; required by AiStatus
AgentStepState::{Pending, Active, Done}                       // default Pending; AgentStep::new(label, state)
AssistantSide::{End, Start}  // default End

// Dialog actions
DialogActionTone::{Primary, Ghost, Danger}
// Helper constructors: DialogAction::primary(id, label), .ghost(...), .danger(...)

// AI data builders
// AgentStep::new(label, AgentStepState)
// ToastEntry::new(id, title).with_tone(ToastTone).with_description(text)
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
KineticText {
    id: "headline",
    text: "Hello".to_string(),
    cue: "fade-in".to_string(),
    // Optional styling hook — appended to the built-in `ui-kinetic-text`
    // class. Use it when the scene's surrounding host needs to size the
    // text differently than the default body inheritance (e.g., a
    // display-tier hero title at 96 px).
    class: "scene-hero-title".to_string(),
}

// FLIP transitions across tree positions
SharedLayout {
    SharedElement { id: "card".to_string(), /* content */ }
    // …somewhere else in the same SharedLayout subtree…
}
```

### Scene player (`Scene` / `Clip`)

```rust
// Autoplay film, with on-screen transport
Scene {
    id: "product-intro",
    width: 1920, height: 1080,
    duration_ms: 10_000.0,
    autoplay: Some(true),
    controls: Some(true),
    Clip { start_ms: 0.0, duration_ms: 2_400.0,
        KineticText { id: "title", text: "Hello.".to_string(), cue: "rise-in" }
    }
    Clip { start_ms: 800.0, duration_ms: 2_400.0, fill: ClipFill::HoldEnd,
        KineticText { id: "body", text: "Subtitle.".to_string(), cue: "fade-in" }
    }
}

// Frozen still at a specific frame — no autoplay, no chrome.
// Useful for marketing-page heroes that want a curated frame instead
// of racing autoplay against first paint.
Scene {
    id: "hero-still",
    width: 1920, height: 1080,
    duration_ms: 10_000.0,
    autoplay: Some(false),
    controls: Some(false),
    initial_elapsed_ms: Some(2_200.0),  // seek once at mount
    // …clips…
}
```

`ClipFill::None` (the default) hides the clip outside its window via
`visibility: hidden` — the element still reserves layout space.
Host pages that grid-stack multiple clips and only want active ones in
flow can collapse hidden clips with their own CSS:

```css
.your-host .ui-scene-clip[data-clip-active="false"] { display: none; }
```

The scene stage's backdrop is overridable via the
`--ui-scene-stage-bg` CSS variable; set it to `transparent` when the
host already provides the backdrop (e.g., an ambient mesh).

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

// Added for the AI-native surfaces
Sparkle { size: 16 }   // assistant / AI affordance
Send { size: 16 }      // prompt send
Stop { size: 16 }      // stop streaming
Quote { size: 16 }     // citation
Globe { size: 16 }     // web source
Copy { size: 16 }      // copy message
Link { size: 16 }      // source link
```

(The AI components inline their own glyphs — e.g. `PromptInput` draws the
send arrow and stop square directly — so importing these icons is only
needed when you build your own AI affordances.)

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

10. **`StreamingText` `chunk_boundaries` are BYTE offsets**, not char
    indices. To highlight the last word as the streaming tail, pass
    `text.rfind(' ').map(|i| i + 1)`. An empty/zero boundary makes the
    *whole* string the highlighted tail (intended for a brand-new
    stream).

11. **AI "no link" cases are empty strings, not `Option`.**
    `CitationChip` and `SourceCard` take `href` (and `favicon`/`snippet`)
    as `String`; an empty `href` silently switches `CitationChip` from
    `<a>` to a non-navigating `<button>` and `SourceCard` from `<a>` to a
    static `<article>`. There's no `onclick` — wire the button variant's
    behavior yourself.

12. **`PromptInput` is fully controlled.** Wire `on_input` to a signal
    and feed `value` back, or the textarea won't update. `on_submit`
    hands you the current value — clear it yourself
    (`value.set(String::new())`); the component doesn't.

13. **`AssistantPanel` is NON-modal; `Sheet`/`Dialog` are modal.** The
    assistant panel renders no backdrop and does not trap focus (the
    page stays usable). `Sheet` and `Dialog` trap focus, dim the page,
    and restore focus to the opener on close. Give each simultaneously
    open `Sheet`/`Dialog` a unique `id` so their focus traps don't
    collide.

14. **`Toaster` is render-driven — you own the list.** It does not keep
    a queue. Hold a `Signal<Vec<ToastEntry>>`, pass
    `items: entries.read().clone()`, push to add, and **remove the id in
    `on_dismiss`** (`entries.write().retain(|e| e.id != id)`) or the
    toast never clears. Auto-dismiss pauses while the pointer hovers.

15. **`Heading` vs `Text`: level ≠ size.** `Heading.level` fixes the
    semantic `h1..h6` tag; `Heading.variant` (an `Option<TextVariant>`)
    overrides only the visual size. `Text` never emits an `hN` tag — use
    it for prose, `Heading` for the document outline. Deep headings
    (level 4+) default to `Body` size unless you pass an explicit
    `variant`.

16. **`Avatar` chooses image vs initials by `src.is_empty()`** — there
    is no load-error fallback, so a broken `src` shows a broken image.
    `name` is required and doubles as the `<img alt>` / initials
    `aria-label`; never leave it blank.

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
    "liquid-glass",  # wgpu glass engine via ui-glass-dioxus + ui-glass-engine (default, recommended)
    "blocks",        # ui-blocks Scene catalog: LowerThird/Caption/WipeTransition/MetricCounter/SocialOverlay (default)
]
```

`liquid-glass` and `blocks` are **on by default**; drop them with
`default-features = false` only if you've measured you don't need the
wgpu glass path or the cinematic Scene blocks. The AI-native surfaces,
typography, feedback, and overlay primitives are unconditional (no
feature flag).

---

## Where to look in the source

- **Component implementations**: `crates/ui-dioxus/src/`
- **AI-native surfaces**: `crates/ui-dioxus/src/ai/` (`streaming_text`,
  `status`, `citation`, `source_card`, `prompt_input`,
  `assistant_panel`, `agent_timeline`)
- **Typography / display primitives**: `crates/ui-dioxus/src/typography.rs`
  (`Heading`, `Text`), `crates/ui-dioxus/src/display.rs` (`Badge`,
  `Avatar`, `Spinner`, `Alert`, `Progress`, `Skeleton`)
- **Overlays** (`Sheet`, `Toaster`, focus trap):
  `crates/ui-dioxus/src/overlays/`
- **Foundational types**: `crates/ui-core/src/lib.rs`
- **Glass material types**: `crates/ui-glass/src/lib.rs`
- **Motion primitives**: `crates/ui-motion/`, `crates/ui-timeline/`
- **Runtime hooks**: `crates/ui-runtime/src/`
- **Stylesheet**: `crates/ui-styles/src/lib.rs` (`COMPONENT_CSS`,
  `SCENE_PLAYER_CSS`, `KINETIC_CUES_CSS`)
- **Prelude truth**: `crates/kinetics/src/lib.rs::prelude`
- **Gallery preview patterns**: `examples/component-gallery/src/previews/`
- **Flagship marketing-page patterns**: `examples/flagship/src/` —
  five full-bleed sections (Hero / Story / Features / Metrics / CTA)
  composed entirely from existing scenes and components. Useful as
  a template for product-launch surfaces and as a worked example of
  hosting `Scene` outside the gallery's documentation chrome.

---

## Reading OS preferences

The gallery's `persistence` module exposes one-shot read helpers that
work on both wasm (via `match_media`) and native (return `false`):

```rust
use component_gallery::persistence::{
    prefers_reduced_motion,
    prefers_color_scheme_dark,
};

let reduced = prefers_reduced_motion();
let dark = prefers_color_scheme_dark();
```

Apply them to the shell via `data-*` attributes — the library's CSS
keys off the same attributes (`[data-ui-motion="reduced"]`,
`[data-ui-theme="dark"]`):

```rust
main {
    "data-ui-motion": if reduced { "reduced" } else { "normal" },
    "data-ui-theme": if dark { "dark" } else { "light" },
    // children
}
```

`subscribe_reduced_motion(callback)` is available for live updates;
the marketing-page flagship deliberately reads once at mount and
accepts that mid-session OS toggles require a refresh.

---

## Useful links inside this repo

- `docs/component-naming.md` — full standard ↔ functional table
- `docs/glass-materials.md` — material recipes + tier ladder
- `docs/platform-support.md` — target matrix
- `examples/component-gallery/` — every component rendered in 4
  variants, served via `dx serve` or built via `dx build --release`
- `examples/flagship/` — full-bleed marketing page; canonical
  reference for hosting scenes outside docs chrome and for the
  binding "Hero-3-seconds" visual check
  (`examples/flagship/docs/hero-screenshot.png`)
