//! Generates / verifies the spring settling reference JSON consumed by the
//! Playwright motion suite. The TS side cannot recompute spring physics in a
//! browser without drifting from ui-motion, so we serialise the authoritative
//! settling times here and assert parity in CI.
//!
//! Update mode: KINETICS_UPDATE_SPRING_REFERENCE=1 cargo test -p component-gallery --test spring_reference
//! Verify mode (default): cargo test -p component-gallery --test spring_reference

use serde_json::json;
use std::path::PathBuf;
use ui_motion::Spring;

const REFERENCE_PATH_REL: &str = "e2e/tests/_lib/spring-reference.json";
const SETTLE_TOLERANCE: f32 = 0.005;

fn reference_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(REFERENCE_PATH_REL)
}

fn expected_value() -> serde_json::Value {
    let snappy = Spring::snappy();
    json!({
        "tolerance": SETTLE_TOLERANCE,
        "presets": {
            "snappy": {
                "stiffness": snappy.stiffness,
                "damping": snappy.damping,
                "mass": snappy.mass,
                "settling_ms": snappy.settling_duration_ms(SETTLE_TOLERANCE),
            }
        }
    })
}

#[test]
fn spring_reference_json_is_in_sync() {
    let expected = expected_value();
    let path = reference_path();

    if std::env::var("KINETICS_UPDATE_SPRING_REFERENCE").is_ok() {
        let serialized = serde_json::to_string_pretty(&expected).expect("serialize");
        std::fs::write(&path, format!("{serialized}\n")).expect("write reference");
        return;
    }

    let actual = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "spring-reference.json missing at {}: {err}.\n\
             Regenerate with KINETICS_UPDATE_SPRING_REFERENCE=1 cargo test -p component-gallery --test spring_reference",
            path.display()
        )
    });

    let actual_value: serde_json::Value =
        serde_json::from_str(&actual).expect("spring-reference.json must be valid JSON");

    assert_eq!(
        actual_value, expected,
        "spring-reference.json is out of sync with ui-motion. \
         Regenerate with KINETICS_UPDATE_SPRING_REFERENCE=1 cargo test -p component-gallery --test spring_reference"
    );
}
