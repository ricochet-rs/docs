use leptos::prelude::*;

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
        is_latest: true,
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
    let (current, set_version) = signal(get_current_version().clone());
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
