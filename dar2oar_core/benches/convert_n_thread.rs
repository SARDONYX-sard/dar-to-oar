use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dar2oar_core::fs::converter::{parallel, sequential};
use dar2oar_core::{read_mapping_table, Closure, ConvertOptions};
use std::time::Duration;
use tokio::fs;

const REMOVE_TARGET: &str =
    "../test/data/UNDERDOG Animations/meshes/actors/character/animations/OpenAnimationReplacer";
const TARGET: &str = "../test/data/UNDERDOG Animations";
const TABLE_PATH: &str = "../test/mapping_tables/UnderDog Animations_v1.9.6_mapping_table.txt";

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("dar2oar sequential vs parallel");
    group
        .measurement_time(Duration::from_secs(23))
        .sample_size(10);

    group.bench_function("dar2oar multi thread", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.to_async(rt).iter(|| async {
            if std::path::Path::new(REMOVE_TARGET).exists() {
                fs::remove_dir_all(REMOVE_TARGET).await.unwrap();
            }
            let mapping = read_mapping_table(TABLE_PATH).await.unwrap();

            parallel::convert_dar_to_oar(
                black_box(ConvertOptions {
                    dar_dir: TARGET.into(),
                    section_table: Some(mapping),
                    ..Default::default()
                }),
                Closure::default,
            )
            .await
        })
    });

    group.bench_function("dar2oar single thread", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.to_async(&rt).iter(|| async {
            if std::path::Path::new(REMOVE_TARGET).exists() {
                fs::remove_dir_all(REMOVE_TARGET).await.unwrap();
            }
            let mapping = read_mapping_table(TABLE_PATH).await.unwrap();

            sequential::convert_dar_to_oar(
                black_box(ConvertOptions {
                    dar_dir: TARGET.into(),
                    section_table: Some(mapping),
                    ..Default::default()
                }),
                Closure::default,
            )
            .await
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
