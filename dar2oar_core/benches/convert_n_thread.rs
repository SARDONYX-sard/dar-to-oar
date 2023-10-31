use criterion::async_executor::FuturesExecutor;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dar2oar_core::{
    convert_dar_to_oar,
    fs::{parallel, ConvertOptions},
    read_mapping_table,
};
use std::time::Duration;
use tokio::fs;

const REMOVE_TARGET: &str =
    "../test/data/UNDERDOG Animations/meshes/actors/character/animations/OpenAnimationReplacer";
const TARGET: &str = "../test/data/UNDERDOG Animations";
const TABLE_PATH: &str = "../test/settings/UnderDog Animations_v1.9.6_mapping_table.txt";

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("dar2oar sequential vs parallel");
    group.warm_up_time(Duration::from_secs(70)).sample_size(10);

    group.bench_function("dar2oar multi thread", |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            if std::path::Path::new(REMOVE_TARGET).exists() {
                fs::remove_dir_all(REMOVE_TARGET).await.unwrap();
            }
            let mapping = read_mapping_table(TABLE_PATH).await.unwrap();

            parallel::convert_dar_to_oar(black_box(ConvertOptions {
                dar_dir: TARGET,
                section_table: Some(mapping),
                ..Default::default()
            }))
            .await
        })
    });

    group.bench_function("dar2oar single thread", |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            if std::path::Path::new(REMOVE_TARGET).exists() {
                fs::remove_dir_all(REMOVE_TARGET).await.unwrap();
            }
            let mapping = read_mapping_table(TABLE_PATH).await.unwrap();

            convert_dar_to_oar(black_box(ConvertOptions {
                dar_dir: TARGET,
                section_table: Some(mapping),
                ..Default::default()
            }))
            .await
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
