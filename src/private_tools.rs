#[macro_export]
macro_rules! priv_impl_default {
    ($name:ident) => {
        impl std::default::Default for $name {
            fn default() -> Self {
                return $name::new();
            }
        }
    };
}
#[macro_export]
macro_rules! priv_impl_getinfo {
    ($name:ident) => {
        impl crate::info::GetInfo for $name {
            fn get_info(&self) -> crate::info::Info {
                return crate::info::Info {
                    scope: (*self.scope).to_string(),
                    name: (*self.name).to_string(),
                };
            }
        }
    };
}
#[macro_export]
macro_rules! priv_set_info {
    () => {
        /// Set package name
        pub fn set_name(mut self, value: impl Into<String>) -> Self {
            self.name = value.into();
            self
        }
        /// Set package scope
        pub fn set_scope(mut self, value: impl Into<String>) -> Self {
            self.scope = value.into();
            self
        }
    };
}
#[macro_export]
macro_rules! priv_from_info {
    // Version with no arguments
    () => {
        crate::priv_set_info!();
        /// Set `scope` and `name` from struct that extends [crate::info::GetInfo] trait
        pub fn from_info<T: AsRef<U>, U: crate::info::GetInfo>(info: T) -> Self
        {
            let res = info.as_ref().get_info();
            Self {
                scope: res.scope,
                name: res.name,
            }
        }
    };

    // Version with arguments
    ($( $name:ident: $value:expr ),*) => {
        crate::priv_set_info!();
        /// Set `scope` and `name` from struct that extends [crate::info::GetInfo] trait
        pub fn from_info<T: AsRef<U>, U: crate::info::GetInfo>(info: T) -> Self
        {
            let res = info.as_ref().get_info();
            Self {
                $( $name: $value ),*,
                scope: res.scope,
                name: res.name
            }
        }
    };
}
#[macro_export]
macro_rules! priv_default_urls {
    ( $( $name:ident = $url:literal ),* ) => {
        $(
            pub static $name: Lazy<Url> = Lazy::new(|| {
                return Url::parse($url).expect(concat!("Failed to parse default url (", stringify!($name), ") for rust-jsr-registry"))
            });
        )*
    };
}
#[macro_export]
macro_rules! priv_as_ref {
    ($name:ident) => {
        impl AsRef<$name> for $name {
            fn as_ref(&self) -> &$name {
                return self;
            }
        }
    };
}
#[macro_export]
macro_rules! priv_enum_derived {
    ($name:ident, $( $field_name:ident ),*) => {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "lowercase")]
        pub enum $name {
            $( $field_name ),*
        }
    };
}
