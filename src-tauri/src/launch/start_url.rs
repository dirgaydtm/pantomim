use plist::{Dictionary, Value};

pub fn start_url(settings: &Dictionary, link: &str) -> Option<String> {
    let url = settings.get("startURL")?.as_string()?.to_string();
    let append = settings
        .get("startURLAppendQueryParameter")
        .and_then(Value::as_boolean)
        .unwrap_or(false);
    let extra = query(link).and_then(|q| q.rfind('?').map(|i| &q[i..]));
    let Some(extra) = extra.filter(|_| append) else {
        return Some(url);
    };

    match query(&url) {
        Some(q) if extra.len() > 1 => {
            let q = format!("?{q}");
            Some(url.replace(&q, &format!("{q}&{}", &extra[1..])))
        }
        _ => Some(format!("{url}{extra}")),
    }
}

fn query(url: &str) -> Option<&str> {
    url.split('#').next()?.split_once('?').map(|(_, q)| q)
}
