use anyhow::Result;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{meta::{Meta, MetaBuilder, NpmCompMeta}, package::{Package, PackageBuilder}, priv_as_ref, priv_impl_default, DEFAULT_NPM_COMP_URL, DEFAULT_URL};
pub trait GetProviderScope {
    fn get_provider_scope(&self) -> &str;
}
#[derive(Debug, Clone)]
pub struct FetcherBuilder {
    /// The host
    pub host: Host,
    /// The [reqwest] [Client] to be used in Fetcher
    pub client: Option<Client>,
    /// The jsr provider scope on npm-side. Defaults to `jsr`
    /// 
    /// NOTE: This is **DIFFERENT** than normal package scope
    /// 
    /// See https://jsr.io/docs/api#npm-compatibility-registry-api
    pub provider_scope: String,
}
impl GetProviderScope for FetcherBuilder {
    fn get_provider_scope(&self) -> &str {
        return self.provider_scope.as_str();
    }
}
impl FetcherBuilder {
    /// Creates a builder for [Fetcher]
    pub fn new() -> Self {
        return Self {
            host: Host::new(),
            client: None,
            provider_scope: "jsr".to_string()
        };
    }
    /// Set the host urls
    pub fn set_host(mut self, value:impl AsRef<Host>) -> Self {
        self.host = *value.as_ref();
        return self;
    }
    /// Set the [reqwest] [Client]
    /// 
    /// ## Performance cost
    /// 
    /// Since client was intentionally cloned ([see here](https://docs.rs/reqwest/0.11.4/src/reqwest/async_impl/client.rs.html#61)),
    /// It doesn't impact the performance 
    pub fn set_client(mut self, value:impl AsRef<Client>) -> Self {
        self.client = Some(value.as_ref().clone());
        return self;
    } 
    /// Set the provider scope (for npm-side)
    pub fn set_provider_scope(mut self, value:impl AsRef<String>) -> Self {
        self.provider_scope = value.as_ref().to_string();
        return self;
    }
}
priv_impl_default!(FetcherBuilder);
pub enum HostSelector {
    Main,
    NpmComp
}
/// Creates a host urls for [FetcherBuilder] and [Fetcher]
#[derive(Debug, Clone, Copy)]
pub struct Host {
    pub main:&'static Url,
    pub npm_comp:&'static Url,
}
impl Host {
    /// Creates a host urls for [FetcherBuilder] and [Fetcher]
    pub fn new() -> Self {
        return Self {
            main:&DEFAULT_URL,
            npm_comp:&DEFAULT_NPM_COMP_URL
        };
    }
    /// Set the host url for JSR main packages
    pub fn set_main(mut self, value:&'static Url) -> Self {
        self.main = value;
        return self;
    }
    /// Set the host url for JSR-to-npm compatible packages
    pub fn set_npm_comp(mut self, value:&'static Url) -> Self {
        self.npm_comp = value;
        return self;
    } 
}
priv_as_ref!(Host);
priv_impl_default!(Host);
#[derive(Debug)]
pub struct Fetcher {
    /// The host url (defaults to `https://jsr.io`)
    pub host: Host,

    /// The [reqwest] [Client] to be used in Fetcher
    client: Client,

    /// The jsr provider scope on npm-side. Defaults to `jsr`
    /// 
    /// NOTE: This is **DIFFERENT** than normal package scope
    /// 
    /// See https://jsr.io/docs/api#npm-compatibility-registry-api
    pub provider_scope: String,
}
impl GetProviderScope for Fetcher {
    fn get_provider_scope(&self) -> &str {
        return self.provider_scope.as_str();
    }
}
impl Fetcher {
    /// Creates a new Fetcher
    /// 
    /// ## Panics
    /// 
    /// See the docs for [reqwest::Client::new]. You can prevent this by setting your own [reqwest] [Client] in [FetcherBuilder]
    pub fn new(builder: FetcherBuilder) -> Self {
        return Self {
            host: builder.host,
            client: if let Some(v) = builder.client {
                v
            } else {
                Client::new()
            },
            provider_scope: builder.provider_scope
        };
    }
    /// The fetcher.
    ///
    /// NOTE for [reqwest::Error]: The fetcher **always** hides the url, so its safe
    ///
    /// ## Errors
    ///
    /// Throws [reqwest::Error] or [serde_json::Error]
    async fn fetcher<T: DeserializeOwned>(&self, sel:HostSelector, path: impl Into<String>) -> Result<Option<T>> {
        let host = match sel {
            HostSelector::Main => {
                self.host.main
            }
            HostSelector::NpmComp => {
                self.host.npm_comp
            }
        };
        let url = format!("{}{}", host, path.into());
        let res_raw = self.client.get(url).send().await;
        if let Err(v) = res_raw {
            return Err(v.without_url().into());
        }
    
        let res = res_raw.unwrap();
    
        // Clone status before consuming `res`
        let status = res.status();
        if let Err(v) = res.error_for_status_ref() { // Use `error_for_status_ref()` instead
            let err = v.without_url();
            if status == StatusCode::NOT_FOUND {
                return Ok(None);
            } else {
                return Err(err.into());
            }
        }
    
        // Now `res` is still available, so we can read the body
        let body = res.text().await;
        match body {
            Err(v) => Err(v.without_url().into()),
            Ok(v) => {
                let parsed: T = serde_json::from_str(&v)?; // Deserialize JSON
                Ok(Some(parsed))
            }
        }
    }
    
    /// Get package metadata, contains version details, and more
    /// 
    /// Returns `Ok(None)` if the server returned `404`. 
    /// If the other failed status code returned, `Err` will be returned.
    /// Else, `Ok(Some(Meta))` returned normally as success
    ///
    /// See https://jsr.io/docs/api#package-version
    /// 
    /// ## Errors
    ///
    /// Throws [reqwest::Error] or [serde_json::Error]
    pub async fn get_meta(&self, value:impl AsRef<MetaBuilder>) -> Result<Option<Meta>> {
        let res = value.as_ref();
        return self
            .fetcher::<Meta>(HostSelector::Main, format!("@{}/{}/meta.json", res.scope, res.name))
            .await;
    }
    /// Get metadatas from packages, contains version details, and more
    ///
    /// See https://jsr.io/docs/api#package-version
    /// 
    /// ## Errors
    ///
    /// Throws [reqwest::Error] or [serde_json::Error]
    pub async fn get_metas<T: AsRef<MetaBuilder>, U: IntoIterator<Item = T> + ExactSizeIterator>(&self, values:U) -> Result<Vec<Meta>> {
        let mut results = Vec::with_capacity(values.len()); 
        for each in values {
            let res = self.get_meta(each).await?;
            if let Some(v) = res {
                results.push(v);
            }
        }
        Ok(results)
    }
    /// Get package **with** specific version on it. 
    /// 
    /// Returns `Ok(None)` if the server returned `404`. 
    /// If the other failed status code returned, `Err` will be returned.
    /// Else, `Ok(Some(Package))` returned normally as success
    /// 
    /// See https://jsr.io/docs/api#package-version-metadata
    /// 
    /// To get the list of versions, use [Fetcher::get_meta]
    ///
    /// ## Errors
    ///
    /// Throws [reqwest::Error] or [serde_json::Error]
    pub async fn get_package(&self, value: impl AsRef<PackageBuilder>) -> Result<Option<Package>> {
        let res = value.as_ref();
        return self
            .fetcher::<Package>(HostSelector::Main, format!("@{}/{}/{}_meta.json", res.scope, res.name, res.version))
            .await;
    }
    /// Get each packages **with** specific version on it
    ///
    /// See https://jsr.io/docs/api#package-version-metadata
    /// 
    /// ## Errors
    ///
    /// Throws [reqwest::Error] or [serde_json::Error]
    pub async fn get_packages<T: AsRef<PackageBuilder>, U: IntoIterator<Item = T> + ExactSizeIterator>(&self, values: U) -> Result<Vec<Package>> {
        let mut results = Vec::with_capacity(values.len()); 
        for each in values {
            let res = self.get_package(each).await?;
            if let Some(v) = res {
                results.push(v);
            }
        }
        Ok(results)
    }
    /// Get JSR-to-npm package meta
    ///
    /// The difference for [Meta] other than the compability, 
    /// is that you dont need to fetch again for specific version.
    /// You just need to get from [NpmCompMeta::versions], and then the result will show up
    /// 
    /// Returns `Ok(None)` if the server returned `404`. 
    /// If the other failed status code returned, `Err` will be returned.
    /// Else, `Ok(Some(NpmCompMeta))` returned normally as success
    /// 
    /// See https://jsr.io/docs/api#package-version-metadata
    /// 
    /// ## Errors
    ///
    /// Throws [reqwest::Error] or [serde_json::Error]
    pub async fn get_npm_comp_meta(&self, value: impl AsRef<MetaBuilder>) -> Result<Option<NpmCompMeta>> {
        let res = value.as_ref();
        return self
            .fetcher::<NpmCompMeta>(HostSelector::NpmComp, format!("@{}/{}__{}", self.provider_scope, res.scope, res.name))
            .await;
    }
    /// Get JSR-to-npm package metas
    ///
    /// The difference for [Meta] other than the compability, 
    /// is that you dont need to fetch again for specific version.
    /// You just need to get [NpmCompMeta::versions] from one of them, and then the result will show up
    /// 
    /// See https://jsr.io/docs/api#package-version-metadata
    /// 
    /// ## Errors
    ///
    /// Throws [reqwest::Error] or [serde_json::Error]
    pub async fn get_npm_comp_metas<T: AsRef<MetaBuilder>, U: IntoIterator<Item = T> + ExactSizeIterator>(&self, values: U) -> Result<Vec<NpmCompMeta>> {
        let mut results = Vec::with_capacity(values.len()); 
        for each in values {
            let res = self.get_npm_comp_meta(each).await?;
            if let Some(v) = res {
                results.push(v);
            }
        }
        Ok(results)
    }
}
impl std::default::Default for Fetcher {
    /// Creates a new fetcher with default configuration
    /// 
    /// ## Panics
    /// 
    /// See the docs for [reqwest::Client::new]. You can prevent this by making your own [FetcherBuilder] and set the [reqwest] [Client] to [Self::new]
    fn default() -> Self {
        return Self::new(FetcherBuilder::new())
    }
}