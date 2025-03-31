use std::collections::HashMap;
use semver::Version;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
/// Versions info from [Meta::versions]
pub struct VersionInfo {
    /// Detect if version is yanked (archived)
    #[serde(default)]
    pub yanked: bool
}
/// Creates a builder for [Meta]
#[derive(Debug, Clone)]
pub struct MetaBuilder {
    /// Package scope
    ///
    /// ## Example
    ///
    /// `dunno` from `@dunno/object`
    pub scope: String,
    /// Package name
    ///
    /// ## Example
    ///
    /// `object` from `@dunno/object`
    pub name: String,
}
impl MetaBuilder {
    /// Creates a builder for [Package]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // ik theres Default, but.. idk
        return Self {
            scope:"".to_string(),
            name:"".to_string()
        };
    }
    /// Set package scope
    pub fn set_scope(mut self, value:String) -> Self {
        self.scope = value;
        return self;
    }
    /// Set package name
    pub fn set_name(mut self, value:String) -> Self {
        self.name = value;
        return self;
    }
}

/// The package metadata result
/// 
/// See https://jsr.io/docs/api#package-metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    /// Package scope
    ///
    /// ## Example
    ///
    /// `dunno` from `@dunno/object`
    pub scope: String,
    /// Package name
    ///
    /// ## Example
    ///
    /// `object` from `@dunno/object`
    pub name: String,
    /// Latest version from one of [Meta::versions]
    pub latest: Version,
    /// List of versions founded from metadata
    pub versions: HashMap<Version, VersionInfo>,
}
impl PartialEq for Meta {
    fn eq(&self, other:&Self) -> bool {
        return self.scope == other.scope && self.name == other.name;
    }
}