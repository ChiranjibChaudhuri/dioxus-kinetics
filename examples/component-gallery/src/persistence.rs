pub const KEY_THEME: &str = "kx-gallery-theme";
pub const KEY_DENSITY: &str = "kx-gallery-density";
pub const KEY_MOTION: &str = "kx-gallery-motion";
pub const KEY_GLASS: &str = "kx-gallery-glass";

pub fn load(key: &str) -> Option<String> {
    imp::load(key)
}

pub fn save(key: &str, value: &str) {
    imp::save(key, value);
}

pub fn prefers_reduced_motion() -> bool {
    imp::prefers_reduced_motion()
}

/// Subscribe to OS-level `prefers-reduced-motion` changes. The callback fires
/// whenever the media-query state flips. Returns `true` if a listener was
/// registered successfully (so callers can skip a manual one-shot read).
pub fn subscribe_reduced_motion(on_change: impl FnMut(bool) + 'static) -> bool {
    imp::subscribe_reduced_motion(on_change)
}

#[cfg(target_arch = "wasm32")]
mod imp {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    pub fn load(key: &str) -> Option<String> {
        let window = web_sys::window()?;
        let storage = window.local_storage().ok()??;
        storage.get_item(key).ok().flatten()
    }

    pub fn save(key: &str, value: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(key, value);
            }
        }
    }

    pub fn prefers_reduced_motion() -> bool {
        let Some(window) = web_sys::window() else {
            return false;
        };
        match window.match_media("(prefers-reduced-motion: reduce)") {
            Ok(Some(mql)) => mql.matches(),
            _ => false,
        }
    }

    pub fn subscribe_reduced_motion(mut on_change: impl FnMut(bool) + 'static) -> bool {
        let Some(window) = web_sys::window() else {
            return false;
        };
        let mql = match window.match_media("(prefers-reduced-motion: reduce)") {
            Ok(Some(mql)) => mql,
            _ => return false,
        };
        let closure = Closure::wrap(Box::new(move |evt: web_sys::MediaQueryListEvent| {
            on_change(evt.matches());
        }) as Box<dyn FnMut(_)>);
        if mql
            .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
            .is_err()
        {
            return false;
        }
        // Listener lifetime is tied to the gallery's lifetime (the entire page);
        // leak the closure rather than tracking it through component teardown.
        closure.forget();
        true
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    pub fn load(_key: &str) -> Option<String> {
        None
    }
    pub fn save(_key: &str, _value: &str) {}
    pub fn prefers_reduced_motion() -> bool {
        false
    }
    pub fn subscribe_reduced_motion(_on_change: impl FnMut(bool) + 'static) -> bool {
        false
    }
}
