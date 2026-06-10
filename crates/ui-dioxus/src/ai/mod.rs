//! AI-native surfaces and primitives.
//!
//! These components target conversational / agentic UIs (streaming
//! assistant output, source citations, prompt composers, agent run
//! timelines). Each component is self-contained — it emits plain
//! elements, inline `<svg>`, and the Wave-1 `.ui-*` classes; it never
//! calls sibling components. Motion is delegated to CSS, which gates on
//! `prefers-reduced-motion`.

mod agent_timeline;
mod assistant_panel;
mod citation;
mod prompt_input;
mod source_card;
mod status;
mod streaming_text;
mod voice;

pub use agent_timeline::*;
pub use assistant_panel::*;
pub use citation::*;
pub use prompt_input::*;
pub use source_card::*;
pub use status::*;
pub use streaming_text::*;
pub use voice::*;
