/// Module graph 1
/// 
/// JSR docs dosent explain this. Moreover, this only included on old & early JSR packages
#[cfg(feature = "unstable")]
pub mod one {}

/// Objects for moduleGraph2
pub mod two {
    use serde::{Deserialize, Serialize};

    use crate::{priv_as_ref, priv_enum_derived};

    /// Module graph 2
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ModuleGraph2 {
        /// Dependencies that are used in the file
        pub dependencies: Option<Vec<Dependency>>,
    }
    priv_as_ref!(ModuleGraph2);
    
    priv_enum_derived!(DependencyType, Static, Dynamic);
    priv_enum_derived!(DependencyKind, Import, Export);

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    /// Dependencies from [ModuleGraph2]
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
    impl PartialOrd for Dependency {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.specifier_range.partial_cmp(&other.specifier_range)
        }
    }
    impl Ord for Dependency {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.specifier_range.cmp(&other.specifier_range)
        }
    }
    priv_as_ref!(Dependency);
}
