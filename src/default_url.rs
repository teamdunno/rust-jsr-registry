use once_cell::sync::Lazy;
use url::Url;
// fuck it, since the url is well-formed,
// why the fuck do we need to handle each builder & real struct with this shit

/// Default JSR url as static object. Need deref to use it
pub static DEFAULT_URL: Lazy<Url> = Lazy::new(|| {
    Url::parse("https://jsr.io").expect("Failed to parse default url for rust-jsr-registry")
});