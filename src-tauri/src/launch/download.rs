use std::path::Path;

pub fn download_url(arg: &str) -> Option<String> {
    let (scheme, rest) = arg.split_once("://")?;
    let scheme = match scheme.to_ascii_lowercase().as_str() {
        "seb" | "http" => "http",
        "sebs" | "https" => "https",
        _ => return None,
    };
    Some(format!("{scheme}://{rest}"))
}

pub fn is_browser_resource(status: u16, content_type: Option<&str>) -> bool {
    let media_type = content_type
        .and_then(|c| c.split(';').next())
        .map(str::trim);
    status == 401 || media_type == Some("text/html")
}

pub fn is_configuration_download(url: &str, file_name: &str) -> bool {
    let url_path = url.split(['?', '#']).next().unwrap_or(url);
    [file_name, url_path].into_iter().any(|name| {
        Path::new(name)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("seb"))
    })
}
