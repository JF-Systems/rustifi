//! Shared HTTP transport used by the UniFi Network and Protect clients.

use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::error::Result;
use reqwest::Client;

/// Build a reqwest client with the crate's standard settings.
///
/// # Security Warning
/// Passing `accept_invalid_certs = true` disables TLS certificate validation.
/// Only use this for local consoles with self-signed certificates on trusted
/// networks.
pub(crate) fn build_http_client(accept_invalid_certs: bool, cookies: bool) -> Result<Client> {
    let mut builder = Client::builder()
        .user_agent("rustifi/1.0")
        .connect_timeout(std::time::Duration::from_secs(30))
        .timeout(std::time::Duration::from_secs(60));

    if cookies {
        let cookie_store = std::sync::Arc::new(reqwest::cookie::Jar::default());
        builder = builder.cookie_store(true).cookie_provider(cookie_store);
    }

    if accept_invalid_certs {
        builder = builder
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true);
    }

    Ok(builder.build()?)
}

/// Execute an [`Endpoint`] request against `{url_prefix}/{path}` and
/// deserialize the JSON response.
///
/// `url_prefix` is the caller-computed base (e.g. `https://host/api/v1` or the
/// remote connector form) without a trailing slash.
pub(crate) async fn execute_endpoint<E>(
    http: &Client,
    url_prefix: &str,
    api_key: Option<&str>,
    endpoint: &E,
) -> Result<E::Response>
where
    E: Endpoint,
    E::Response: for<'a> serde::Deserialize<'a>,
{
    let url = format!("{}/{}", url_prefix, endpoint.build_path());

    let mut headers = reqwest::header::HeaderMap::new();
    if let Some(api_key) = api_key {
        headers.insert("X-API-Key", api_key.parse()?);
    }

    let mut request = http.request(E::METHOD.into(), &url).headers(headers);

    // Append query parameters using reqwest's query() for proper URL encoding
    let params = endpoint.query_params();
    if !params.is_empty() {
        request = request.query(&params);
    }

    // Add JSON body for POST/PUT/PATCH methods
    if let Some(body) = endpoint.request_body()? {
        request = request.json(&body);
    }

    let response = request.send().await?;

    if !response.status().is_success() {
        // Preserve the error body — controllers return JSON describing the
        // actual failure reason, which error_for_status() would discard.
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(crate::error::Error::Api { status, body });
    }

    let body = response.text().await?;
    // Some endpoints (e.g. DELETE) return an empty body; deserialize it as null
    // so responses typed as Option<_> or () succeed.
    let effective_body = if body.trim().is_empty() {
        "null"
    } else {
        &body
    };
    let response_data = serde_json::from_str::<E::Response>(effective_body)
        .map_err(|e| crate::error::Error::Parse(format!("{}\nResponse body: {}", e, body)))?;
    Ok(response_data)
}

impl From<HttpMethod> for reqwest::Method {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::Get => reqwest::Method::GET,
            HttpMethod::Post => reqwest::Method::POST,
            HttpMethod::Put => reqwest::Method::PUT,
            HttpMethod::Patch => reqwest::Method::PATCH,
            HttpMethod::Delete => reqwest::Method::DELETE,
        }
    }
}
