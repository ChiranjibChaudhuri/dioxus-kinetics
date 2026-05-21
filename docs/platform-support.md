# Platform Support

| Target | Status | Backend |
|---|---|---|
| Web | MVP | DOM style adapter |
| Desktop | MVP | WebView DOM style adapter |
| Mobile | MVP | WebView DOM style adapter |
| Native | MVP contract | Native capability adapter |

Native support begins with semantic parity, token rendering, glass fallback,
focus behavior, motion snapshots, composition metadata, and capture manifests.

Timeline, composition, and capture are native Rust/Dioxus systems. They do not
depend on third-party animation, video, or capture runtimes.
