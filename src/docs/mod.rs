use std::collections::BTreeMap;

#[derive(Hash, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub enum DocSection {
    QuickStart,
    Content,
    Admin,
}

impl ToString for DocSection {
    fn to_string(&self) -> String {
        match self {
            DocSection::QuickStart => "Quickstart",
            DocSection::Content => "Content Items",
            DocSection::Admin => "Server Administration",
        }
        .to_string()
    }
}

#[derive(Clone)]
pub struct DocPage {
    pub section: DocSection,
    pub title: &'static str,
    pub href: &'static str,
    pub body: &'static str,
}

pub const DOC_PAGES: [DocPage; 15] = [
    DocPage {
        section: DocSection::QuickStart,
        title: "Get started",
        href: "/quickstart",
        body: include_str!("quickstart.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Overview",
        href: "/overview",
        body: include_str!("content-items.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Access & permissions",
        href: "/access",
        body: include_str!("content-access.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Invoking items",
        href: "/invocation",
        body: include_str!("invoking-items.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Environment variables",
        href: "/env-vars",
        body: include_str!("environment-variables.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Persistent storage",
        href: "/persistence",
        body: include_str!("persistence.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Using _ricochet.toml",
        href: "/ricochet-toml",
        body: include_str!("ricochet-toml.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Scheduling items",
        href: "/scheduling",
        body: include_str!("scheduling.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Scaling performance",
        href: "/scaling",
        body: include_str!("servable-settings.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Serverless R",
        href: "/serverless-r",
        body: include_str!("serverless-r.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Static HTML Sites",
        href: "/static",
        body: include_str!("static-settings.html"),
    },
    DocPage {
        section: DocSection::Admin,
        title: "Installing ricochet",
        href: "/install",
        body: include_str!("install.html"),
    },
    DocPage {
        section: DocSection::Admin,
        title: "Bootstrap dependencies",
        href: "/bootstrap",
        body: include_str!("bootstrap.html"),
    },
    DocPage {
        section: DocSection::Admin,
        title: "ricochet cli",
        href: "/ricochet-cli",
        body: include_str!("cli.html"),
    },
    DocPage {
        section: DocSection::Admin,
        title: "Environment restoration",
        href: "/env-restore",
        body: "",
    },
];

pub fn doc_sections() -> BTreeMap<DocSection, Vec<&'static DocPage>> {
    let mut map = BTreeMap::new();

    let mut quick = Vec::new();
    let mut content = Vec::new();
    let mut admin = Vec::new();

    for doc in DOC_PAGES.iter() {
        match &doc.section {
            DocSection::QuickStart => quick.push(doc),
            DocSection::Content => content.push(doc),
            DocSection::Admin => admin.push(doc),
        }
    }
    map.insert(DocSection::QuickStart, quick);
    map.insert(DocSection::Content, content);
    map.insert(DocSection::Admin, admin);
    map
}

pub struct DocNavItem {
    pub body: &'static str,
    pub prev_slug: Option<usize>,
    pub next_slug: Option<usize>,
}

impl Default for DocNavItem {
    fn default() -> Self {
        let body = r#"<div class="not-prose mx-auto flex h-full max-w-xl flex-col items-center justify-center text-center"><p class="text-sm font-semibold text-zinc-900 dark:text-white">404</p><h1 class="mt-2 text-2xl font-bold text-zinc-900 dark:text-white">Page not found</h1><p class="mt-2 text-base text-zinc-600 dark:text-zinc-400">These are not the docs you're looking for.</p><a class="inline-flex gap-0.5 justify-center overflow-hidden text-sm font-medium transition bg-zinc-900 py-1 px-3 text-white hover:bg-zinc-700 dark:bg-emerald-400/10 dark:text-emerald-400 dark:ring-1 dark:ring-inset dark:ring-emerald-400/20 dark:hover:bg-emerald-400/10 dark:hover:text-emerald-300 dark:hover:ring-emerald-300 mt-8" href="/">Back to docs<svg viewBox="0 0 20 20" fill="none" aria-hidden="true" class="mt-0.5 h-5 w-5 -mr-1"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" d="m11.5 6.5 3 3.5m0 0-3 3.5m3-3.5h-9"></path></svg></a></div>"#;
        Self {
            body,
            prev_slug: None,
            next_slug: None,
        }
    }
}

pub fn get_doc(slug: &str) -> Option<DocNavItem> {
    let idx = DOC_PAGES
        .iter()
        .enumerate()
        .position(|(_, d)| d.href == slug);

    match idx {
        Some(idx) => {
            let body = DOC_PAGES[idx].body;
            let next_slug = DOC_PAGES.get(idx + 1).map(|_| idx + 1);

            let prev_slug = if idx == 0 {
                None
            } else {
                DOC_PAGES.get(idx - 1).map(|_| idx - 1)
            };

            let res = DocNavItem {
                body,
                prev_slug,
                next_slug,
            };
            Some(res)
        }
        None => None,
    }
}
