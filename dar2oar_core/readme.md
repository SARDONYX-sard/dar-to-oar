# dar2oar_core

`dar2oar_core` is a Rust crate that provides functionality for converting
Dynamic Animation Replacer (DAR) files to Overwrite Animation Replacer (OAR)
files. The crate includes modules for parsing conditions, handling DAR syntax,
managing values, and dealing with file systems.

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
[dev-dependencies]
anyhow = { version = "1.0.75", features = ["backtrace"] }
tokio = { version = "1.33.0", features = [
  "fs",
  "io-util",
  "macros",
  "rt",
] } # Async Executor
once_cell = "1.18.0"
pretty_assertions = "1.4.0"
tracing-appender = "0.2"
tracing-subscriber = "0.3.17"
```

### Async with non Progress report

```rust
use anyhow::Result;
use dar2oar_core::{convert_dar_to_oar, ConvertOptions, get_mapping_table};

const DAR_DIR: &str = "../test/data/UNDERDOG Animations";
const TABLE_PATH: &str = "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt";
const LOG_PATH: &str = "../convert.log";

/// Initialization macro for setting up logging.
macro_rules! logger_init {
    () => {
        let (non_blocking, _guard) =
            tracing_appender::non_blocking(std::fs::File::create(LOG_PATH).unwrap());
        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_max_level(tracing::Level::DEBUG)
            .init();
    };
}

/// Asynchronous function to create conversion options.
async fn create_options<'a>() -> Result<ConvertOptions<'a, &'a str>> {
    Ok(ConvertOptions {
        dar_dir: DAR_DIR,
        section_table: get_mapping_table(Some(TABLE_PATH)).await,
        ..Default::default()
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    logger_init!();
    convert_dar_to_oar(create_options().await?, |_| {}).await?;
    Ok(())
}
```

### Parallel Async with Progress report

```rust
use anyhow::Result;
use dar2oar_core::{convert_dar_to_oar, ConvertOptions, get_mapping_table};

const DAR_DIR: &str = "../test/data/UNDERDOG Animations";
const TABLE_PATH: &str = "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt";
const LOG_PATH: &str = "../convert.log";

/// Initialization macro for setting up logging.
macro_rules! logger_init {
    () => {
        let (non_blocking, _guard) =
            tracing_appender::non_blocking(std::fs::File::create(LOG_PATH).unwrap());
        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_max_level(tracing::Level::DEBUG)
            .init();
    };
}

/// Asynchronous function to create conversion options.
async fn create_options<'a>() -> Result<ConvertOptions<'a, &'a str>> {
    Ok(ConvertOptions {
        dar_dir: DAR_DIR,
        section_table: get_mapping_table(Some(TABLE_PATH)).await,
        run_parallel: true,
        ..Default::default()
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    use once_cell::sync::Lazy;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    logger_init!();
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

    // Receive progress updates and print messages.
    while let Some(idx) = rx.recv().await {
        static NUM: Lazy<AtomicUsize> = Lazy::new(AtomicUsize::default);
        let num = NUM.load(Ordering::Acquire);

        if num != 0 {
            println!("[recv] Converted: {}/{}", idx, num);
        } else {
            NUM.store(idx, Ordering::Release);
            println!("[recv] Converted: {}", idx);
        }
    }

    Ok(())
}
```

## How to run bench

Requirements

- test mod: "../test/data/UNDERDOG Animations"
- mapping_table: "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt"

```shell
cargo bench
# dar2oar sequential vs parallel/dar2oar multi thread
#                         time:   [1.5880 s 1.6398 s 1.6992 s]
# dar2oar sequential vs parallel/dar2oar single thread
#                         time:   [1.9711 s 2.0215 s 2.0908 s]
```
