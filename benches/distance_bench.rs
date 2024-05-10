use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon::prelude::*;

fn benchmark_distance_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("distance_group");
    group.sample_size(20); // Reduce sample count
    group.measurement_time(std::time::Duration::from_secs(22)); // Increase target time

    group.bench_function("single thread", |b| {
        b.iter(|| {
            for _ in 1..=10000000 {
                calculate_distance(
                    black_box(48.8566f64.to_radians()),
                    black_box(2.3522f64.to_radians()),
                    black_box(51.5074f64.to_radians()),
                    black_box(-0.1278f64.to_radians()),
                );
            }
        })
    });

    group.bench_function("multithreads", |b| {
        b.iter(|| {
            (1..=10000000).into_par_iter().for_each(|_| {
                calculate_distance(
                    black_box(48.8566f64.to_radians()),
                    black_box(2.3522f64.to_radians()),
                    black_box(51.5074f64.to_radians()),
                    black_box(-0.1278f64.to_radians()),
                );
            })
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_distance_calculation);
criterion_main!(benches);

const EARTH_RADIUS_IN_KM: f64 = 6371.0;

fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;

    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_IN_KM * c
}
