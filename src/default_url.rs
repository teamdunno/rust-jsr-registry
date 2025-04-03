use once_cell::sync::Lazy;
use url::Url;

use crate::priv_default_urls;
// fuck it, since the url is well-formed,
// why the fuck do we need to handle each builder & real struct with this shit
priv_default_urls!(
    DEFAULT_URL = "https://jsr.io",
    DEFAULT_NPM_COMP_URL = "https://npm.jsr.io"
);