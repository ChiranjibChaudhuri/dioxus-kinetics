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

#[cfg(target_arch = "wasm32")]
mod imp {
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
}
