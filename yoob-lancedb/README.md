<!-- <div style="display: flex; align-items: center; justify-content: center;">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="../img/yoob_logo_dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="../img/yoob_logo.svg">
        <img src="../img/yoob_logo.svg" width="200" alt="yoob logo">
    </picture>
    <span style="font-size: 48px; margin: 0 20px; font-weight: regular; font-family: Open Sans, sans-serif;"> + </span>
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://companieslogo.com/img/oyoob/MDB_BIG.D-96d632a9.png?t=1720244492">
        <source media="(prefers-color-scheme: light)" srcset="https://cdn.iconscout.com/icon/free/png-256/free-mongodb-logo-icon-download-in-svg-png-gif-file-formats--wordmark-programming-langugae-freebies-pack-logos-icons-1175140.png?f=webp&w=256">
        <img src="https://cdn.iconscout.com/icon/free/png-256/free-mongodb-logo-icon-download-in-svg-png-gif-file-formats--wordmark-programming-langugae-freebies-pack-logos-icons-1175140.png?f=webp&w=256" width="200" alt="MongoDB logo">
    </picture>
</div>

<br><br> -->

## yoob-Lancedb
This companion crate implements a yoob vector store based on Lancedb.

## Pre-requisites
If you are using `yoob-lancedb` locally, you must ensure you have `protoc` (the [Protobuf Compiler](https://protobuf.dev/installation/)) installed.

## Usage

Add the companion crate to your `Cargo.toml`, along with the yoob-core crate:

```toml
[dependencies]
yoob-lancedb = "0.1.0"
yoob-core = "0.4.0"
```

You can also run `cargo add yoob-lancedb yoob-core` to add the most recent versions of the dependencies to your project.

See the [`/examples`](./examples) folder for usage examples.
