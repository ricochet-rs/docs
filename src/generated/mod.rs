use crate::versioning::Version;

#[derive(Debug, Clone)]
pub struct DocPage {
    pub section: DocSection,
    pub title: &'static str,
    pub href: &'static str,
    pub body: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DocSection {
    QuickStart,
    Content,
    Admin,
}

impl std::fmt::Display for DocSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocSection::QuickStart => write!(f, "Quick Start"),
            DocSection::Content => write!(f, "Content"),
            DocSection::Admin => write!(f, "Admin"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DocNavItem {
    pub title: &'static str,
    pub body: &'static str,
    pub prev_slug: Option<usize>,
    pub next_slug: Option<usize>,
}

// Version-specific doc pages
pub const DOC_PAGES_V0_1: [DocPage; 16] = [
    DocPage {
        section: DocSection::QuickStart,
        title: "Overview",
        href: "/",
        body: include_str!("v0.1/home.html"),
    },
    DocPage {
        section: DocSection::QuickStart,
        title: "Get started",
        href: "/quickstart",
        body: include_str!("v0.1/quickstart.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Overview",
        href: "/overview",
        body: include_str!("v0.1/content-items.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Access & permissions",
        href: "/access",
        body: include_str!("v0.1/content-access.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Invoking items",
        href: "/invocation",
        body: include_str!("v0.1/invoking-items.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Environment variables",
        href: "/env-vars",
        body: include_str!("v0.1/environment-variables.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Persistent storage",
        href: "/persistence",
        body: include_str!("v0.1/persistence.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Using _ricochet.toml",
        href: "/ricochet-toml",
        body: include_str!("v0.1/ricochet-toml.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Scheduling items",
        href: "/scheduling",
        body: include_str!("v0.1/scheduling.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Scaling performance",
        href: "/scaling",
        body: include_str!("v0.1/servable-settings.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Serverless R",
        href: "/serverless-r",
        body: include_str!("v0.1/serverless-r.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Static HTML Sites",
        href: "/static",
        body: include_str!("v0.1/static-settings.html"),
    },
    DocPage {
        section: DocSection::Admin,
        title: "Installing ricochet",
        href: "/install",
        body: include_str!("v0.1/install.html"),
    },
    DocPage {
        section: DocSection::Admin,
        title: "Bootstrap dependencies",
        href: "/bootstrap",
        body: include_str!("v0.1/bootstrap.html"),
    },
    DocPage {
        section: DocSection::Admin,
        title: "ricochet cli",
        href: "/ricochet-cli",
        body: include_str!("v0.1/cli.html"),
    },
    DocPage {
        section: DocSection::Admin,
        title: "Environment restoration",
        href: "/env-restore",
        body: "",
    },
];

pub const DOC_PAGES_DEV: [DocPage; 16] = [
    DocPage {
        section: DocSection::QuickStart,
        title: "Overview",
        href: "/",
        body: include_str!("dev/home.html"),
    },
    DocPage {
        section: DocSection::QuickStart,
        title: "Get started",
        href: "/quickstart",
        body: include_str!("dev/quickstart.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Overview",
        href: "/overview",
        body: include_str!("dev/content-items.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Access & permissions",
        href: "/access",
        body: include_str!("dev/content-access.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Invoking items",
        href: "/invocation",
        body: include_str!("dev/invoking-items.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Environment variables",
        href: "/env-vars",
        body: include_str!("dev/environment-variables.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Persistent storage",
        href: "/persistence",
        body: include_str!("dev/persistence.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Using _ricochet.toml",
        href: "/ricochet-toml",
        body: include_str!("dev/ricochet-toml.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Scheduling items",
        href: "/scheduling",
        body: include_str!("dev/scheduling.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Scaling performance",
        href: "/scaling",
        body: include_str!("dev/servable-settings.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Serverless R",
        href: "/serverless-r",
        body: include_str!("dev/serverless-r.html"),
    },
    DocPage {
        section: DocSection::Content,
        title: "Static HTML Sites",
        href: "/static",
        body: include_str!("dev/static-settings.html"),
    },
    DocPage {
        section: DocSection::Admin,
        title: "Installing ricochet",
        href: "/install",
        body: include_str!("dev/install.html"),
    },
    DocPage {
        section: DocSection::Admin,
        title: "Bootstrap dependencies",
        href: "/bootstrap",
        body: include_str!("dev/bootstrap.html"),
    },
    DocPage {
        section: DocSection::Admin,
        title: "ricochet cli",
        href: "/ricochet-cli",
        body: include_str!("dev/cli.html"),
    },
    DocPage {
        section: DocSection::Admin,
        title: "Environment restoration",
        href: "/env-restore",
        body: "",
    },
];

// Default to latest version pages for backward compatibility
pub const DOC_PAGES: &[DocPage] = &DOC_PAGES_V0_1;

pub fn get_doc(path: &str) -> Option<DocNavItem> {
    // Find the doc by href
    let doc_index = DOC_PAGES.iter().position(|doc| doc.href == path)?;
    let doc = &DOC_PAGES[doc_index];
    
    Some(DocNavItem {
        title: doc.title,
        body: doc.body,
        prev_slug: if doc_index > 0 { Some(doc_index - 1) } else { None },
        next_slug: if doc_index < DOC_PAGES.len() - 1 { Some(doc_index + 1) } else { None },
    })
}

pub fn get_doc_for_version(path: &str, version: &Version) -> Option<DocNavItem> {
    let pages = match version.path {
        "v0.1" => &DOC_PAGES_V0_1,
        "dev" => &DOC_PAGES_DEV,
        _ => &DOC_PAGES_V0_1, // Default to latest stable
    };
    
    // Find the doc by href
    let doc_index = pages.iter().position(|doc| doc.href == path)?;
    let doc = &pages[doc_index];
    
    Some(DocNavItem {
        title: doc.title,
        body: doc.body,
        prev_slug: if doc_index > 0 { Some(doc_index - 1) } else { None },
        next_slug: if doc_index < pages.len() - 1 { Some(doc_index + 1) } else { None },
    })
}

pub fn doc_sections() -> std::collections::BTreeMap<DocSection, Vec<&'static DocPage>> {
    let mut map = std::collections::BTreeMap::new();
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