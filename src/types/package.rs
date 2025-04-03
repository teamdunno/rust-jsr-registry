use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;
use std::collections::HashMap;

use crate::{priv_as_ref, priv_from_info, priv_impl_getinfo};

use super::{graph::two::ModuleGraph2};

/// The package result
///
/// See https://jsr.io/docs/api#package-version-metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    /// List of file manifests in one package
    /// 
    /// ([HashMap] Key prefix: `/` ([see this example](https://jsr.io/@dunno/obfuscatemail/1.5.2_meta.json) for differences))
    pub manifest: HashMap<String, Manifest>,
    /// Module graph 1. Since it was used **only** for old & early JSR packages ([example](https://jsr.io/@luca/flag/1.0.1_meta.json)),
    /// [Package::module_graph2] was widely used
    /// 
    /// ([HashMap] Key prefix: `/` ([see this example](https://jsr.io/@dunno/obfuscatemail/1.5.2_meta.json) for differences))
    pub module_graph1: Option<HashMap<String, Value>>,
    /// Module graph 2. This is the most widely used graph on JSR packages
    /// 
    /// ([HashMap] Key prefix: `/` ([see this example](https://jsr.io/@dunno/obfuscatemail/1.5.2_meta.json) for differences))
    pub module_graph2: Option<HashMap<String, ModuleGraph2>>,
    /// Exported files. For main entry, usually `.` exist. But can be changed using `./` prefix
    /// 
    /// The key and value on `exports` outputted from `meta.json` are converted to [HashMap]
    /// 
    /// So if `deno.json` file only has this
    /// ```json
    /// "exports":"hi.ts"
    /// ```
    /// 
    /// JSR will convert it as this in `meta.json`
    /// 
    /// ```json
    /// "exports":{
    ///   ".":"./hi.ts"
    /// }
    /// ```
    /// 
    /// Though, you can change it so it dosent contain the main entry:
    /// 
    /// ```json
    /// "exports":{
    ///   "./hi":"./hi.ts"
    /// }
    /// ```
    /// 
    /// JSR will kept it like that
    pub exports:HashMap<String, String>
}

/// File manifests from [Package::manifest]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Manifest {
    /// Object size
    pub size: u64,
    /// Object checksum (dosent validated, beware)
    pub checksum: String,
}

/// Creates a builder for [Package]
#[derive(Debug, Clone)]
pub struct PackageBuilder {
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
    /// Package version, parsed as [semver::Version]
    pub version: Version,
}
/// Creates a builder for [Package]
impl PackageBuilder {
    /// Creates a builder for [Package]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        return Self {
            version: Version::new(0, 0, 0),
            scope: "".to_string(),
            name: "".to_string(),
        };
    }
    /// Set package version to choose
    pub fn set_version(mut self, value: Version) -> Self {
        self.version = value;
        return self;
    }
    priv_from_info!(version: Version::new(0, 0, 0));
}
impl PartialEq for PackageBuilder {
    fn eq(&self, other: &Self) -> bool {
        return self.scope == other.scope && self.name == other.name && self.version.to_string() == other.version.to_string();
    }
}
impl Eq for PackageBuilder {}
priv_as_ref!(PackageBuilder);
priv_impl_getinfo!(PackageBuilder);

/// Distribution infos for [NpmCompPackage::dist]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NpmCompDist {
    /// The package tarball from npm
    #[serde(with = "crate::serde::url")]
    pub tarball:Url,
    /// The shasum hash from package
    pub shasum:String,
    /// Package integrity (as sha-256)
    pub integrity:String,
}
priv_as_ref!(NpmCompDist);
/// The package result
///
/// See https://jsr.io/docs/api#package-version-metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NpmCompPackage {
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
    /// The version currently selected
    pub version:Version,
    /// Package desription
    pub description:String,
    /// Distribution info
    pub dist:NpmCompDist,
    /// Timestamp for package activities
    pub dependencies:HashMap<String, VersionReq>
}
priv_as_ref!(NpmCompPackage);