use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ml_tracker::{InMemoryMetricStore, MetricLogger, MetricStore};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use uuid::Uuid;

fn bench_metric_logging(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let store: Arc<Mutex<dyn MetricStore>> =
        Arc::new(Mutex::new(InMemoryMetricStore::new().unwrap()));

    let mut group = c.benchmark_group("metric_logging");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("sequential", size), size, |b, &size| {
            b.iter(|| {
                rt.block_on(async {
                    let run_id = Uuid::new_v4();
                    let mut logger = MetricLogger::new(run_id, store.clone(), Default::default());

                    for i in 0..size {
                        logger.log("bench_metric", i as f64).await.unwrap();
                    }

                    logger.flush().await.unwrap();
                })
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_metric_logging);
criterion_main!(benches);
