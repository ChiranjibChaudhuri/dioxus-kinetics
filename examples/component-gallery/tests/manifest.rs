//! Asserts that the TypeScript component manifest used by the Playwright
//! e2e harness lists every ComponentStatus::Ready entry from docs.rs.
//! Drift would let the Node project go stale without anyone noticing,
//! so we fail at `cargo test` time.

use component_gallery::{component_docs, ComponentStatus};

const MANIFEST_TS: &str = include_str!(
    "../e2e/tests/_lib/component-manifest.ts"
);

fn ts_manifest_names() -> Vec<String> {
    let mut names = Vec::new();
    for line in MANIFEST_TS.lines() {
        // We look for `{ name: "Foo", ...` shaped entries. The format is
        // single-line by convention in component-manifest.ts.
        let trimmed = line.trim();
        if !trimmed.starts_with("{ name:") {
            continue;
        }
        let Some(start) = trimmed.find("\"") else { continue };
        let rest = &trimmed[start + 1..];
        let Some(end) = rest.find("\"") else { continue };
        names.push(rest[..end].to_string());
    }
    names
}

#[test]
fn ts_manifest_includes_every_ready_component() {
    let ts_names = ts_manifest_names();
    assert!(
        !ts_names.is_empty(),
        "TS manifest parser found zero entries; format may have drifted"
    );

    // Count guard: if a Prettier reformat (or similar) wraps any entry across
    // multiple lines, the parser drops it silently. Require the parsed entry
    // count to match the count of `{ name:` occurrences a naive grep would
    // catch — if the two diverge, the format has drifted.
    let raw_entry_marker_count = MANIFEST_TS.matches("{ name:").count();
    assert_eq!(
        ts_names.len(),
        raw_entry_marker_count,
        "TS manifest parser parsed {} names but raw `{{ name:` markers occur {} times; \
         entries may have been reformatted across multiple lines",
        ts_names.len(),
        raw_entry_marker_count
    );

    for doc in component_docs().iter().filter(|d| d.status == ComponentStatus::Ready) {
        assert!(
            ts_names.iter().any(|n| n == doc.name),
            "Ready component {:?} is missing from e2e/tests/_lib/component-manifest.ts. \
             Add it before Playwright tests are run.",
            doc.name
        );
    }
}

#[test]
fn ts_manifest_does_not_list_unknown_components() {
    let doc_names: Vec<&str> = component_docs().iter().map(|d| d.name).collect();
    for name in ts_manifest_names() {
        assert!(
            doc_names.iter().any(|n| n == &name.as_str()),
            "TS manifest lists {:?} but no ComponentDoc has that name",
            name
        );
    }
}
