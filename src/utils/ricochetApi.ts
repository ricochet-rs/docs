export const API_ORIGIN = "https://try.ricochet.rs";
export const API_DOCS_URL = `${API_ORIGIN}/api/docs`;

let spec: Promise<Record<string, unknown>> | undefined;

// The API docs page embeds the OpenAPI spec in its `api-reference` script tag.
export function fetchOpenApiSpec(): Promise<Record<string, unknown>> {
  spec ??= fetch(API_DOCS_URL)
    .then((res) => {
      if (!res.ok) throw new Error(`${API_DOCS_URL} returned ${res.status}`);
      return res.text();
    })
    .then((html) => {
      const match = html.match(
        /<script[^>]*id="api-reference"[^>]*>\s*([\s\S]*?)\s*<\/script>/,
      );
      if (!match) throw new Error(`No OpenAPI spec found in ${API_DOCS_URL}`);
      return JSON.parse(match[1]);
    });
  return spec;
}
