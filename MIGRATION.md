## Migration to 2.0: Switching from `Url` to `Host` for `FetcherBuilder`

**Date:** 2025-04-03
**Author:** [vintheweirdass](https://github.com/vintheweirdass)

**Rationale:**
This migration changes to `Host` instead of `Url` since we need to add many `Url`s in one object. To support those JSR-to-npm registry url, and the main JSR itself

**Changes:**
* Replacing `Url` to `Host` on `FetcherBuilder`

**Instructions:**
If you have a code like this

```rust
    FetcherBuilder::new()
       .set_host(Url::parse("https://example.com").expect("cant parse"))
```

You need to merge those `Url` to `Host`, using this

```rust
    FetcherBuilder::new()
       .set_host(Host::new().set_main(Url::parse("https://example.com").expect("cant parse")))
```

But for users that are using the default configurations, this should fine and no changes required

**Potential Issues:**
None.

## Migration to 2.0: `from_meta_builder` deprecation

**Date:** 2025-04-03
**Author:** [vintheweirdass](https://github.com/vintheweirdass)

**Rationale:**
This migration changes the function (`from_info`) so it can support not only `MetaBuilder`, but `Meta`, `PackageBuilder`, and so on

**Changes:**
* `from_meta_builder` is replaced with `from_info`

**Instructions:**
Just replace `from_meta_builder` to `from_info`. No impactful changes required

**Potential Issues:**
None. You can safely remove your borrowed objects, or just keep it as your choice
