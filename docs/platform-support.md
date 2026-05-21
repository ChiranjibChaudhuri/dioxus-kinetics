# Platform Support

| Target | Status | Backend |
|---|---|---|
| Web | MVP | DOM style adapter |
| Desktop | MVP | WebView DOM style adapter |
| Mobile | MVP | WebView DOM style adapter |
| Native | MVP contract | Native capability adapter |

Native support begins with semantic parity, token rendering, glass fallback,
focus behavior, and basic motion planning. Native visual fidelity depends on
available Dioxus Native and Blitz renderer capabilities.

Timeline, composition, and capture are first-party Rust/Dioxus systems usable
through web, desktop, mobile WebView, and platform-native adapters.
