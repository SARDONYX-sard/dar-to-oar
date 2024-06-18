# `dar2oar_core`

`dar2oar_core` is a Rust crate that provides functionality for converting
`DynamicAnimationReplacer`(DAR) files to `OpenAnimationReplacer`(OAR) files. The
crate includes modules for parsing conditions, handling DAR syntax, managing
values, and dealing with file systems.

## Modules

- `condition_parser`: Module to convert a parsed DAR into a serializable OAR
  structure.
- `conditions`: Module for representing conditions used in DAR files.
- `dar_syntax`: Module for handling DAR syntax and conversions.
- `values`: Module for managing values used in OAR files.

## Submodules

- `error`: Submodule for defining error types related to DAR to OAR conversion.
- `fs`: Submodule containing file system-related functionalities, including the
  main conversion function.

## Public Functions and Types

- `convert_dar_to_oar`: The main function for converting DAR files to OAR files.
  It accepts configuration options and a progress callback.
- `Closure`: A struct that provides a default closure for progress reporting.
- `ConvertOptions`: A struct containing various configuration options for the
  conversion process.
- `ConvertedReport`: An enum representing different outcomes of the conversion
  process.
- `remove_oar`: Function for removing OAR files from a directory.
- `unhide_dar`: Function to unhide DAR files after conversion.
- `get_mapping_table`: Function for obtaining a mapping table.
- `read_mapping_table`: Function for reading a mapping table from a specified
  path.

## Examples

- Cargo.toml dependencies

```toml
[dependencies]
anyhow = { version = "1.0.75", features = ["backtrace"] }
dar2oar_core = { git = "https://github.com/SARDONYX-sard/dar-to-oar", tag = "0.7.0" }
quick_tracing = {version = "0.1.5", features = ["derive"] }
tokio = { version = "1.33.0", features = [ "fs", "io-util", "macros", "rt", "rt-multi-thread" ] } # Async Executor
```

### Parallel Async with Progress report

```rust
use anyhow::Result;
use dar2oar_core::{convert_dar_to_oar, ConvertOptions, get_mapping_table};
use std::path::Path;
use tracing::{Level, level_filters::LevelFilter, subscriber::DefaultGuard};

const DAR_DIR: &str = "../test/data/UNDERDOG Animations";
const TABLE_PATH: &str = "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt";

/// Asynchronous function to create conversion options.
async fn create_options() -> Result<ConvertOptions> {
    Ok(ConvertOptions {
        dar_dir: DAR_DIR.into(),
        section_table: get_mapping_table(Some(TABLE_PATH)).await,
        run_parallel: true,
        ..Default::default()
    })
}

#[tokio::main]
#[quick_tracing::try_init(file = "../convert.log", level = "DEBUG")]
async fn main() -> Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(500);

    // Send function for progress reporting.
    let sender = move |idx: usize| {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(idx).await.unwrap_or_default();
        });
    };

    // Spawn conversion process with progress reporting.
    tokio::spawn(convert_dar_to_oar(create_options().await?, sender));

    let mut walk_len = 0usize;
    // Receive progress updates and print messages.
    while let Some(idx) = rx.recv().await {
         match walk_len == 0 {
            true => walk_len = idx, // NOTE: 1st received index is length.
            false => println!("[recv] Converted: {}/{}", idx + 1, walk_len),
         }
    }

    Ok(())
}
```

## Documentation

```shell
cargo doc --open
```

## How to run bench

1. Requirements

   - test mod: `../test/data/EVG Conditional Idles`
   - `mapping_table`:
     `../test/settings/EVG Conditional Idles_v1.4.2_mapping_table.txt`

2. Execute the following command

```shell
cargo bench
```

3.Output sample

```shell
Gnuplot not found, using plotters backend
Benchmarking dar2oar sequential vs parallel/dar2oar multi thread: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 23.0s. You may wish to increase target time to 43.4s or enable flat sampling.
dar2oar sequential vs parallel/dar2oar multi thread
                        time:   [694.11 ms 737.74 ms 789.09 ms]
                        change: [+69870% +73764% +78219%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
dar2oar sequential vs parallel/dar2oar single thread
                        time:   [1.6347 s 1.7119 s 1.7996 s]
                        change: [+207973% +218915% +231114%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) low mild
  1 (10.00%) high severe
```
