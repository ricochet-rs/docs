use leptos::prelude::*;

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = localStorage)]
    fn getItem(key: &str) -> Option<String>;

    #[wasm_bindgen(js_namespace = localStorage)]
    fn setItem(key: &str, value: &str);
}

#[derive(Clone, Debug, PartialEq)]
pub struct Version {
    pub label: &'static str,
    pub path: &'static str,
    pub is_latest: bool,
}

pub const VERSIONS: [Version; 2] = [
    Version {
        label: "v0.1",
        path: "v0.1",
        is_latest: false,
    },
    Version {
        label: "dev",
        path: "dev",
        is_latest: false,
    },
];

pub fn get_current_version() -> &'static Version {
    VERSIONS
        .iter()
        .find(|v| v.is_latest)
        .unwrap_or(&VERSIONS[0])
}

pub fn get_version_by_path(path: &str) -> Option<&'static Version> {
    VERSIONS.iter().find(|v| v.path == path)
}

#[derive(Clone, Debug)]
pub struct VersionContext {
    pub current: Signal<Version>,
    pub set_version: WriteSignal<Version>,
}

pub fn provide_version_context() -> VersionContext {
    // Helper function to get from localStorage (client-side only)
    let get_stored_version = || -> Option<String> {
        #[cfg(feature = "hydrate")]
        {
            getItem("ricochet_docs_version")
        }
        #[cfg(not(feature = "hydrate"))]
        {
            None
        }
    };

    // Helper function to set to localStorage (client-side only)
    let set_stored_version = |version: &str| {
        #[cfg(feature = "hydrate")]
        {
            setItem("ricochet_docs_version", version);
        }
        #[cfg(not(feature = "hydrate"))]
        {
            let _ = version; // Suppress unused warning
        }
    };

    // Start with localStorage or default version
    let initial_version = if let Some(stored) = get_stored_version() {
        if let Some(version) = get_version_by_path(&stored) {
            version.clone()
        } else {
            get_current_version().clone()
        }
    } else {
        get_current_version().clone()
    };

    let (current, set_version) = signal(initial_version);

    // Update localStorage whenever version changes (client-side only)
    Effect::new(move || {
        let version = current.get();
        set_stored_version(version.path);
    });

    let context = VersionContext {
        current: current.into(),
        set_version,
    };
    provide_context(context.clone());
    context
}

pub fn use_version_context() -> Option<VersionContext> {
    use_context::<VersionContext>()
}
