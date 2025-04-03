// import fetcher, and packages
use rust_jsr_registry::{
    fetcher::Fetcher,
    meta::MetaBuilder
};

// add tokio for async purposes
#[tokio::main]
async fn main() {
    // create new package meta builder (as info)
    // so we can resolve the metas and packages
    let info = &MetaBuilder::new()
        .set_scope("kaiverse")
        .set_name("signal-react");
    // create new fetcher with default config
    let fetcher = Fetcher::default();
    // resolve the package meta. If weird thing happens, throw the first .expect()
    // but if the meta wasnt found, throw the second .expect()
    let meta = fetcher
        .get_npm_comp_meta(info)
        .await.expect("failed to get meta").expect("meta wasnt found");

    // get the latest version
    let latest = meta.dist_tags.latest;

    // now if we need to print the latest published date, just do this
    let latest_date = meta.time.versions.inner().get(&latest)
        .expect("latest version date dosent found")
        .to_rfc2822();

    println!("Latest version published: {}", latest_date);

    // no need to fetch itself for specific version, just do this

    // from the `latest` object that we've saved, use the version list to get it
    let pkg = meta.versions.get(&latest).expect("latest version dosent found");
    println!("Dependencies:");
    // each dependencies that we've found, print it
    pkg.dependencies.keys().for_each(move |v| println!("{}", v));
}