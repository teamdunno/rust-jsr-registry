/// Module graph 1
/// 
/// JSR docs dosent explain this. Moreover, this only included on old & early JSR packages
#[cfg(feature = "unstable")]
pub mod one {}

/// Objects for moduleGraph2
pub mod two {
    use serde::{Deserialize, Serialize};

    /// Module graph 2
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ModuleGraph2 {
        /// Dependencies that are used in the file
        pub dependencies: Option<Vec<Dependency>>,
    }
    /// Dependency type, as enum
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum DependencyType {
        Static,
        Dynamic
    }
    /// Dependency kind, as enum
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum DependencyKind {
        Import,
        Export
    }    
    /// Dependencies from [ModuleGraph2]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Dependency {
        /// Dependency type ("importing from `import` keyword (static) or `import()` function (dynamic)?")
        pub r#type: DependencyType,
        /// Dependency kind ("Is it imported/exported?")
        pub kind: DependencyKind,
        /// Dependency specifier (module path)
        pub specifier: String,
        /// Specifier range
        /// 
        /// The line count was pretty tricky though, so we does'nt recommend using it
        ///  
        /// But since JSR does'nt document it, 
        /// we assumed that the number is for the opening & closing path of [Dependency::specifier].
        pub specifier_range: ((u32, u32), (u32, u32))
    }
}
