// import fetcher, and packages
use rust_jsr_registry::{
    fetcher::Fetcher,
    meta::MetaBuilder, package::PackageBuilder,
};

// add tokio for async purposes
#[tokio::main]
async fn main() {
    // create new package meta builder (as info)
    // so we can resolve the metas and packages
    let info = &MetaBuilder::new()
        .set_scope("dunno")
        .set_name("object");
    // create new fetcher with default config
    let fetcher = Fetcher::default();
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
            PackageBuilder::from_info(info)
                .set_version(latest)
        ).await.expect("failed to fetch package").expect("package wasnt found");
    // pretty-print the package
    print!("{:#?}", pkg)
}