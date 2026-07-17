//! File-upload surfaces: a native `FileInput`, a click-or-drop `DropZone`,
//! and a presentational `Attachment` chip. Files are surfaced through a
//! renderer-neutral [`AttachedFile`] so callers stay decoupled from Dioxus'
//! `FileData`. [`format_bytes`] is the shared human-readable size helper.

use dioxus::prelude::*;

/// A renderer-neutral snapshot of a selected file. Built from a Dioxus
/// `FileData` at change time; carried to consumer handlers and rendered by
/// [`Attachment`].
#[derive(Clone, Debug, PartialEq)]
pub struct AttachedFile {
    pub name: String,
    pub size_bytes: u64,
    pub content_type: String,
}

impl AttachedFile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size_bytes: 0,
            content_type: String::new(),
        }
    }

    pub fn from_file_data(file: &dioxus::html::FileData) -> Self {
        Self {
            name: file.name(),
            size_bytes: file.size(),
            content_type: file.content_type().unwrap_or_default(),
        }
    }
}

/// Format a byte count as a compact human-readable string (B / KB / MB / GB),
/// one decimal place past the unit boundary.
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

fn collect_files(evt: &FormEvent) -> Vec<AttachedFile> {
    evt.files()
        .iter()
        .map(AttachedFile::from_file_data)
        .collect()
}

/// Native `<input type="file">` with label, help text, and accept/multiple
/// pass-through. Hands selected files to `on_change` as [`AttachedFile`]s.
#[component]
pub fn FileInput(
    id: String,
    label: String,
    #[props(default)] accept: String,
    #[props(default)] help_text: String,
    #[props(default)] multiple: bool,
    #[props(default)] disabled: bool,
    #[props(default)] required: bool,
    on_change: Option<EventHandler<Vec<AttachedFile>>>,
) -> Element {
    let described_by = if help_text.is_empty() {
        String::new()
    } else {
        format!("{id}-help")
    };

    rsx! {
        div { class: "ui-file-input",
            label { class: "ui-file-input-label", r#for: "{id}", "{label}" }
            input {
                id: "{id}",
                class: "ui-file-input-control",
                r#type: "file",
                accept: "{accept}",
                multiple,
                disabled,
                "aria-required": if required { "true" } else { "false" },
                "aria-describedby": "{described_by}",
                onchange: move |evt: FormEvent| {
                    if let Some(handler) = &on_change {
                        handler.call(collect_files(&evt));
                    }
                },
            }
            if !help_text.is_empty() {
                p { id: "{id}-help", class: "ui-file-input-help", "{help_text}" }
            }
        }
    }
}

/// Click-or-drop upload region. A visually-hidden native `<input type=file>`
/// is the acquisition path (wrapped by a `<label>`, so clicks and keyboard
/// activation open the file dialog); drag over/leave toggles a `--dragover`
/// affordance. `on_change` fires with [`AttachedFile`]s.
#[component]
pub fn DropZone(
    id: String,
    label: String,
    #[props(default)] hint: String,
    #[props(default)] accept: String,
    #[props(default)] multiple: bool,
    #[props(default)] disabled: bool,
    on_change: Option<EventHandler<Vec<AttachedFile>>>,
) -> Element {
    let mut dragging = use_signal(|| false);
    let region_class = if *dragging.read() {
        "ui-dropzone-region ui-dropzone-region--dragover"
    } else {
        "ui-dropzone-region"
    };

    rsx! {
        div {
            class: "ui-dropzone",
            "data-disabled": if disabled { "true" } else { "false" },
            label {
                class: "{region_class}",
                r#for: "{id}",
                ondragover: move |_evt| {
                    dragging.set(true);
                },
                ondragleave: move |_evt| {
                    dragging.set(false);
                },
                ondrop: move |_evt| {
                    dragging.set(false);
                },
                span { class: "ui-dropzone-label", "{label}" }
                if !hint.is_empty() {
                    span { class: "ui-dropzone-hint", "{hint}" }
                }
            }
            input {
                id: "{id}",
                class: "ui-dropzone-input",
                r#type: "file",
                accept: "{accept}",
                multiple,
                disabled,
                onchange: move |evt: FormEvent| {
                    if let Some(handler) = &on_change {
                        handler.call(collect_files(&evt));
                    }
                },
            }
        }
    }
}

/// Presentational chip for one selected file — name, formatted size, and an
/// optional remove control. Pair with `FileInput`/`DropZone` selections.
#[component]
pub fn Attachment(
    name: String,
    #[props(default)] size_bytes: u64,
    on_remove: Option<EventHandler<()>>,
) -> Element {
    rsx! {
        span { class: "ui-attachment",
            span { class: "ui-attachment-name", "{name}" }
            span { class: "ui-attachment-size", "{format_bytes(size_bytes)}" }
            if on_remove.is_some() {
                button {
                    class: "ui-attachment-remove",
                    r#type: "button",
                    "aria-label": "Remove {name}",
                    onclick: move |_evt| {
                        if let Some(handler) = &on_remove {
                            handler.call(());
                        }
                    },
                    "×"
                }
            }
        }
    }
}
