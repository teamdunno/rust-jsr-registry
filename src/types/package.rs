use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::{graph::two::ModuleGraph2, meta::MetaBuilder};

/// The package result
///
/// See https://jsr.io/docs/api#package-version-metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Set package scope
    ///
    /// For ergonomic use, you could use the combination of [PackageBuilder::from_meta_builder] and [MetaBuilder]
    pub fn set_scope(mut self, value: String) -> Self {
        self.scope = value;
        return self;
    }
    /// Set package name
    ///
    /// For ergonomic use, you could use the combination of [PackageBuilder::from_meta_builder] and [MetaBuilder]
    pub fn set_name(mut self, value: String) -> Self {
        self.name = value;
        return self;
    }
    /// Set package version to choose
    pub fn set_version(mut self, value: Version) -> Self {
        self.version = value;
        return self;
    }
    /// Set `scope` and `name` from [MetaBuilder]
    pub fn from_meta_builder(mut self, builder: &MetaBuilder) -> Self {
        self.scope = (&*builder.scope).to_string();
        self.name = (&*builder.name).to_string();
        return self;
    }
}
