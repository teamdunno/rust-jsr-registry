use std::collections::HashMap;
use semver::Version;
use serde::{Deserialize, Serialize};
use crate::{fetcher::GetProviderScope, priv_as_ref, priv_from_info, priv_impl_getinfo};
pub use crate::serde::{VersionDateTimeMap, TimeInfo as NpmCompTimeInfo};

use super::{error::NpmCompParseError, package::NpmCompPackage};
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
/// Versions info from [Meta::versions]
pub struct VersionInfo {
    /// Detect if version is yanked (archived)
    #[serde(default)]
    pub yanked: bool
}
/// Creates a builder for [Meta]
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Creates a builder for [Meta]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        return Self {
            scope:"".to_string(),
            name:"".to_string()
        };
    }
    /// Convert JSR-to-npm equivalent package name to normal one
    /// 
    /// It needs either [FetcherBuilder] or [crate::fetcher::Fetcher] since it needs [FetcherBuilder::provider_scope] to detect the scope owned in npm-side
    /// 
    /// ## Panics
    /// 
    /// Throw panic if it cant parse, use [Self::try_from_npm_comp_name] to handle it
    pub fn from_npm_comp_name<T:GetProviderScope>(gts:impl AsRef<T>, value:impl Into<String>) -> Self {
        return Self::try_from_npm_comp_name(gts, value).expect("Failed to parse string to MetaBuilder");
    }
    /// Convert JSR-to-npm equivalent package name to normal one, as [Result]
    /// 
    /// It needs either [FetcherBuilder] or [crate::fetcher::Fetcher] since it needs [FetcherBuilder::provider_scope] to detect the scope owned in npm-side
    pub fn try_from_npm_comp_name<T:GetProviderScope>(gts:impl AsRef<T>, value:impl Into<String>) -> Result<Self, NpmCompParseError> {
        let builder_ref = gts.as_ref();
        let prov = builder_ref.get_provider_scope().to_string();
        let v = value.into();
        if !v.starts_with(format!("@{}/", prov).as_str()) {
            return Err(NpmCompParseError::DosentStartWithPrefix(prov.to_string()));
        }
        let calc = 2+prov.len();
        let parts: Vec<&str> = v[calc..].split("__").collect();
    
        if parts.len() != 2 {
            return Err(NpmCompParseError::CompFormat);
        }
    
        return Ok(Self {
            scope: parts[0].to_string(), 
            name: parts[1].to_string()
        })
    }
    priv_from_info!();
}
priv_as_ref!(MetaBuilder);
priv_impl_getinfo!(MetaBuilder);

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
    /// Latest version from one of [Self::versions]
    pub latest: Version,
    /// List of versions founded from metadata
    pub versions: HashMap<Version, VersionInfo>,
}
impl PartialEq for Meta {
    fn eq(&self, other: &Self) -> bool {
        return self.scope == other.scope && self.name == other.name && self.latest.to_string() == other.latest.to_string();
    }
}
impl Eq for Meta {}
priv_impl_getinfo!(Meta);
priv_as_ref!(Meta); 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpmCompDistTags {
    /// Latest version of the package
    pub latest:Version
}

/// JSR-to-npm equivalent package meta
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NpmCompMeta {
    /// JSR-to-npm equivalent package name
    /// 
    /// Note: this is different than normal [Meta::name].
    /// 
    /// If your [MetaBuilder] contains like this
    /// 
    /// ```
    /// MetaBuilder::new()
    ///     .set_scope("dunno")
    ///     .set_name("object")
    /// ```
    /// 
    /// It would be added like this, right?
    /// 
    /// ```
    /// {
    ///     scope: "dunno",
    ///     name: "object"
    /// }
    /// ```
    /// 
    /// Since the scope (in JSR) is actually fake on npm, it would be listed as
    /// 
    /// `@<jsr provider scope>/<jsr package scope>__<jsr package name>`
    /// 
    /// So, this [Self::name] is equivalent to
    /// 
    /// `@jsr/dunno__object`
    pub name:String,
    /// Get versions
    pub versions:HashMap<Version, NpmCompPackage>,
    /// Package desription
    pub description:String,
    /// Distribution tags (only contain `latest` for now) 
    /// 
    /// See https://jsr.io/docs/api#npm-compatibility-registry-api on `dist-tags`
    pub dist_tags:NpmCompDistTags,
    /// Timestamp for package activities
    pub time:NpmCompTimeInfo
}
priv_as_ref!(NpmCompMeta); 