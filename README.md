# Rust-jsr-registry

Part of the [teamdunno](https://github.com/teamdunno)'s [`jsr-registry`](https://github.com/search?q=org%3Ateamdunno+jsr-registry&type=repositories) packages

This is the unofficial api wrapper for [JSR](https://jsr.io), a new [npm](https://npmjs.com) like registry owned by [Deno](https://deno.com)

## Installation

Get started by running

```shell
$ cargo add rust-jsr-registry
```

## Example

Initialize a cargo project using

```shell
$ cargo init <folder name here>
```

Then on the initialized folder, add `tokio` and `rust-jsr-registry` itself

```shell
$ cargo add tokio rust-jsr-registry
```

Then edit the `src/main.rs` with following content

<details>

```rs
// import fetcher, and packages
use rust_jsr_registry::{
    fetcher::{Fetcher, FetcherBuilder},
    meta::MetaBuilder, package::PackageBuilder,
};

// add tokio to the module
#[tokio::main]
async fn main() {
    // create new package meta builder (as info)
    // so we can resolve the metas and packages
    let info = &MetaBuilder::new()
        .set_scope("dunno".to_string())
        .set_name("object".to_string());
    // create new fetcher with default config
    let fetcher = Fetcher::new(FetcherBuilder::new());
    // resolve the package meta. If weird thing happens, throw the first .expect()
    // but if the meta wasnt found, throw the second .expect()
    let meta = fetcher
        .get_meta(info)
        .await.expect("failed to get meta").expect("meta wasnt found");
    // convert each meta.versions from semver::Version to string, and collect it as array
    let versions: Vec<String> = meta.versions.keys().map(|v| v.to_string()).collect();
    // print the versions that was included
    println!("Versions: {}", versions.join(", "));
    // add an alias for latest version
    let latest = meta.latest;
    // print the latest version
    println!("{}", latest);
    // resolve the package. If weird thing happens, throw the first .expect()
    // but if the package wasnt found, throw the second .expect()
    let pkg = fetcher
        .get_package(
            &PackageBuilder::new()
                .from_meta_builder(info)
                .set_version(latest)
        ).await.expect("failed to fetch package").expect("package wasnt found");
    // pretty-print the package
    print!("{:#?}", pkg)
}

```
</details>

Then run the project

```shell
$ cargo run
```

## Note

Every JSR api is allowed for public use, but they **does'nt** document it very well. See https://jsr.io/docs/api