use crate::api::ApiEndpoint;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
fn read_toml(slug: String) -> Result<ApiEndpoint, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(slug)?;
    let mut res = toml_edit::de::from_str::<ApiEndpoint>(&contents)?;
    res.description = md_to_html(&res.description);
    res.response = md_to_html(&res.response);
    res.examples.r = md_to_html(&res.examples.r);
    res.examples.curl = md_to_html(&res.examples.curl);
    if let Some(ref mut params) = res.body_params {
        for param in params {
            param.desc = md_to_html(&param.desc);
        }
    }

    if let Some(ref mut params) = res.path_params {
        for param in params {
            param.desc = md_to_html(&param.desc);
        }
    }

    Ok(res)
}

#[server]
pub async fn read_api_ref(slug: String) -> Result<ApiEndpoint, ServerFnError> {
    read_toml(slug).map_err(|e| {
        leptos::logging::log!("{e:?}");
        ServerFnError::new(e)
    })
}

#[cfg(feature = "ssr")]
pub fn md_to_html(content: &str) -> String {
    let parser = pulldown_cmark::Parser::new(content);
    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

#[server]
pub async fn read_md(content: String) -> Result<String, ServerFnError> {
    Ok(md_to_html(&content))
}
