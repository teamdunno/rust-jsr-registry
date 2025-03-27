use reqwest::Client;
use url::{ParseError, Url};
#[derive(Debug)]
pub struct Package {
    pub host: Url,
    pub scope: String,
    pub name: String,
}
#[derive(Debug)]
struct PackageBuilder {
    pub host: Url,
    pub scope: String,
    pub name: String,
}
impl PackageBuilder {
    /// Create a builder for [Package]
    ///
    /// Panics
    /// Panicked only if failed to parse default host (`https://jsr.io`).
    /// If you want to catch them, use [PackageBuilder::new_failable]
    fn new() -> Self {
        
    }
    fn new_failable() -> Result<Self, ParseError> {
        Ok(Self {
            host: Url::parse("https://jsr.io")?,
            scope: "".to_string(),
            name: "".to_string(),
        })
    }
}
/// Package. Use [Package::new] or new_failable
impl Package {
    fn new(builder: PackageBuilder) -> Self {
        Self {
            host: builder.host,
            scope: builder.scope,
            name: builder.name,
        }
    }
}
//
pub fn get_package() -> Package {}
// #[CFG(TEST)]
// MOD TESTS {
//    USE SUPER::*;
//
//    #[TEST]
//    FN IT_WORKS() {
//        LET RESULT = ADD(2, 2);
//        ASSERT_EQ!(RESULT, 4);
//    }
// }
