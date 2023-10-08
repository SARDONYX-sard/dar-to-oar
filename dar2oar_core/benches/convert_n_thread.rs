use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dar2oar_core::{convert_dar_to_oar, fs::sequential, read_mapping_table};
use std::time::Duration;

const REMOVE_TARGET: &str =
    "../test/data/UNDERDOG Animations/meshes/actors/character/animations/OpenAnimationReplacer";
const TARGET: &str = "../test/data/UNDERDOG Animations";

fn criterion_benchmark(c: &mut Criterion) {
    let config = simple_log::LogConfigBuilder::builder()
        .path("../convert.log")
        .size(100)
        .roll_count(10)
        .level("error")
        .output_file()
        .output_console()
        .build();
    simple_log::new(config).unwrap();

    let mut group = c.benchmark_group("dar2oar sequential vs parallel");
    group.warm_up_time(Duration::from_secs(70)).sample_size(10);

    group.bench_function("dar2oar multi thread", |b| {
        b.iter(|| {
            if std::path::Path::new(REMOVE_TARGET).exists() {
                std::fs::remove_dir_all(REMOVE_TARGET).unwrap();
            }
            let table_content = "../test/settings/mapping_table.txt";
            let mapping = read_mapping_table(table_content).unwrap();

            convert_dar_to_oar(black_box(TARGET), None, None, None, Some(mapping), None)
        })
    });

    group.bench_function("dar2oar single thread", |b| {
        b.iter(|| {
            if std::path::Path::new(REMOVE_TARGET).exists() {
                std::fs::remove_dir_all(REMOVE_TARGET).unwrap();
            }
            let table_content = "../test/settings/mapping_table.txt";
            let mapping = read_mapping_table(table_content).unwrap();

            sequential::convert_dar_to_oar(black_box(TARGET), None, None, None, Some(mapping), None)
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
