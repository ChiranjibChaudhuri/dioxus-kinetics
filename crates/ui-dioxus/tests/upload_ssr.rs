use dioxus::prelude::*;
use ui_dioxus::{format_bytes, Attachment, DropZone, FileChip, FileInput, FilePicker, UploadZone};

// Closures passed as props must be constructed inside a Dioxus runtime, so
// callback-bearing trees are rendered through a VirtualDom root component.
fn render_component(root: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(root);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

fn attachment_with_remove() -> Element {
    rsx! {
        Attachment {
            name: "report.pdf".to_string(),
            size_bytes: 2048,
            on_remove: move |_| {},
        }
    }
}

#[test]
fn format_bytes_is_human_readable() {
    assert_eq!(format_bytes(0), "0 B");
    assert_eq!(format_bytes(512), "512 B");
    assert_eq!(format_bytes(1024), "1.0 KB");
    assert_eq!(format_bytes(1500), "1.5 KB");
    assert_eq!(format_bytes(5_000_000), "4.8 MB");
    assert_eq!(format_bytes(1_073_741_824), "1.0 GB");
}

#[test]
fn file_input_renders_native_input_with_attrs() {
    let html = dioxus_ssr::render_element(rsx! {
        FileInput {
            id: "avatar",
            label: "Profile photo".to_string(),
            accept: "image/*".to_string(),
            multiple: true,
            help_text: "PNG or JPG".to_string(),
        }
    });
    assert!(html.contains(r#"type="file""#), "{html}");
    assert!(html.contains(r#"id="avatar""#), "{html}");
    assert!(html.contains(r#"accept="image/*""#), "{html}");
    assert!(html.contains("multiple"), "{html}");
    assert!(html.contains("ui-file-input"), "{html}");
    assert!(html.contains("Profile photo"), "{html}");
    assert!(html.contains("PNG or JPG"), "{html}");
}

#[test]
fn file_input_marks_required_aria() {
    let html = dioxus_ssr::render_element(rsx! {
        FileInput {
            id: "doc",
            label: "Document".to_string(),
            required: true,
        }
    });
    assert!(html.contains(r#"aria-required="true""#), "{html}");
}

#[test]
fn dropzone_renders_region_label_hint_and_hidden_input() {
    let html = dioxus_ssr::render_element(rsx! {
        DropZone {
            id: "drop",
            label: "Drop files here".to_string(),
            hint: "or click to browse".to_string(),
            accept: ".csv".to_string(),
            multiple: true,
        }
    });
    assert!(html.contains("ui-dropzone"), "{html}");
    assert!(html.contains(r#"type="file""#), "{html}");
    assert!(html.contains("Drop files here"), "{html}");
    assert!(html.contains("or click to browse"), "{html}");
    assert!(html.contains(r#"accept=".csv""#), "{html}");
}

#[test]
fn attachment_renders_name_size_and_remove_control() {
    let html = render_component(attachment_with_remove);
    assert!(html.contains("ui-attachment"), "{html}");
    assert!(html.contains("report.pdf"), "{html}");
    assert!(html.contains("2.0 KB"), "{html}");
    assert!(html.contains("ui-attachment-remove"), "{html}");
    assert!(html.contains(r#"type="button""#), "{html}");
}

#[test]
fn attachment_without_on_remove_has_no_remove_control() {
    let html = dioxus_ssr::render_element(rsx! {
        Attachment {
            name: "readme.txt".to_string(),
            size_bytes: 10,
        }
    });
    assert!(!html.contains("ui-attachment-remove"), "{html}");
}

#[test]
fn file_picker_alias_renders_identically() {
    let a = dioxus_ssr::render_element(rsx! { FilePicker {
        id: "x".to_string(), label: "L".to_string()
    }});
    let b = dioxus_ssr::render_element(rsx! { FileInput {
        id: "x".to_string(), label: "L".to_string()
    }});
    assert_eq!(a, b);
}

#[test]
fn upload_zone_alias_renders_identically() {
    let a = dioxus_ssr::render_element(rsx! { UploadZone {
        id: "d".to_string(), label: "L".to_string()
    }});
    let b = dioxus_ssr::render_element(rsx! { DropZone {
        id: "d".to_string(), label: "L".to_string()
    }});
    assert_eq!(a, b);
}

#[test]
fn file_chip_alias_renders_identically() {
    let a = dioxus_ssr::render_element(rsx! { FileChip {
        name: "a.txt".to_string(), size_bytes: 5
    }});
    let b = dioxus_ssr::render_element(rsx! { Attachment {
        name: "a.txt".to_string(), size_bytes: 5
    }});
    assert_eq!(a, b);
}
