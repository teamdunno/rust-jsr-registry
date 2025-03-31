use anyhow::Result;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{meta::{Meta, MetaBuilder}, package::{Package, PackageBuilder}, DEFAULT_URL};
pub struct FetcherBuilder {
    /// The host url (defaults to `https://jsr.io`)
    pub host: &'static Url,
    /// The [reqwest] client to be used in Fetcher
    pub client: Option<Client>,
}
impl FetcherBuilder {
    /// Creates a builder for [Fetcher]
    pub fn new() -> Self {
        return Self {
            host: &DEFAULT_URL,
            client: None,
        };
    }
    /// Set the host url
    pub fn set_host(mut self, value:&'static Url) -> Self {
        self.host = value;
        return self;
    }
    /// Set the [reqwest] client
    pub fn set_client(mut self, value:Client) -> Self {
        self.client = Some(value);
        return self;
    } 
}
pub struct Fetcher {
    /// The host url (defaults to `https://jsr.io`)
    pub host: &'static Url,
    /// The [reqwest] client to be used in Fetcher
    client: Client,
}
impl Fetcher {
    /// Creates a new Fetcher
    /// 
    /// # Panics
    /// 
    /// See the docs for [reqwest::Client::new]. You can prevent this by setting your own [reqwest] client in [FetcherBuilder]
    pub fn new(builder: FetcherBuilder) -> Self {
        return Self {
            host: &builder.host,
            client: if let Some(v) = builder.client {
                v
            } else {
                Client::new()
            },
        };
    }
    /// The fetcher.
    ///
    /// NOTE for [reqwest::Error]: The fetcher **always** hides the url, so its safe
    ///
    /// # Errors
    ///
    /// Throws [reqwest::Error] or [serde_json::Error]
    async fn fetcher<T: DeserializeOwned>(&self, path: impl Into<String>) -> Result<Option<T>> {
        let url = format!("{}{}", &*DEFAULT_URL, path.into());
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
    /// Returns `Some(None)` if the server returned `404`. 
    /// If the other failed status code returned, `Err` will be returned.
    /// Else, `Some(Meta)` returned normally as success
    ///
    /// See https://jsr.io/docs/api#package-version
    /// 
    /// # Errors
    ///
    /// Throws [reqwest::Error] or [serde_json::Error]
    pub async fn get_meta<'a>(&self, builder: &MetaBuilder) -> Result<Option<Meta>> {
        return self
            .fetcher::<Meta>(format!("@{}/{}/meta.json", builder.scope, builder.name))
            .await;
    }
    /// Get metadatas from packages, contains version details, and more
    ///
    /// See https://jsr.io/docs/api#package-version
    /// 
    /// # Errors
    ///
    /// Throws [reqwest::Error] or [serde_json::Error]
    pub async fn get_metas<'a>(&self, builders: &[MetaBuilder]) -> Result<Vec<Meta>> {
        let mut results = Vec::with_capacity(builders.len()); 
        for each in builders {
            let res = self.get_meta(each).await?;
            if let Some(v) = res {
                results.push(v);
            }
        }
        Ok(results)
    }
    /// Get package **with** specific version on it. 
    /// 
    /// Returns `Some(None)` if the server returned `404`. 
    /// If the other failed status code returned, `Err` will be returned.
    /// Else, `Some(Package)` returned normally as success
    /// 
    /// See https://jsr.io/docs/api#package-version-metadata
    /// 
    /// To get the list of versions, use [Fetcher::get_meta]
    ///
    /// # Errors
    ///
    /// Throws [reqwest::Error] or [serde_json::Error]
    pub async fn get_package<'a>(&self, builder: &PackageBuilder) -> Result<Option<Package>> {
        return self
            .fetcher::<Package>(format!("@{}/{}/{}_meta.json", builder.scope, builder.name, builder.version))
            .await;
    }
    /// Get each packages **with** specific version on it
    ///
    /// See https://jsr.io/docs/api#package-version-metadata
    /// 
    /// # Errors
    ///
    /// Throws [reqwest::Error] or [serde_json::Error]
    pub async fn get_packages<'a>(&self, builders: &[PackageBuilder]) -> Result<Vec<Package>> {
        let mut results = Vec::with_capacity(builders.len()); 
        for each in builders {
            let res = self.get_package(each).await?;
            if let Some(v) = res {
                results.push(v);
            }
        }
        Ok(results)
    }
}
