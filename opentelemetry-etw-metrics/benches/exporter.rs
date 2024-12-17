//! run with `$ cargo bench --bench exporter -- --exact <test_name>` to run specific test for logs
//! So to run test named "fibonacci" you would run `$ cargo bench --bench exporter -- --exact fibonacci`
//! To run all tests for logs you would run `$ cargo bench --bench exporter`
//!
/*
The benchmark results:
criterion = "0.5.1"
OS: Windows 11 Enterprise N, 23H2, Build 22631.4460
Hardware: Intel(R) Xeon(R) Platinum 8370C CPU @ 2.80GHz   2.79 GHz, 16vCPUs
RAM: 64.0 GB
| Test                           | Average time|
|--------------------------------|-------------|
| exporter                       | 847.38µs    |
*/

use opentelemetry::{InstrumentationScope, KeyValue};
use opentelemetry_etw_metrics::MetricsExporter;

use opentelemetry_sdk::{
    metrics::{
        data::{DataPoint, Metric, ResourceMetrics, ScopeMetrics, Sum},
        exporter::PushMetricExporter,
        Temporality,
    },
    Resource,
};

use criterion::{criterion_group, criterion_main, Criterion};

async fn export(mut resource_metrics: ResourceMetrics) {
    let exporter = MetricsExporter::new();
    exporter.export(&mut resource_metrics).await.unwrap();
}

fn create_resource_metrics() -> ResourceMetrics {
    let data_point = DataPoint {
        attributes: vec![KeyValue::new("datapoint key", "datapoint value")],
        start_time: Some(std::time::SystemTime::now()),
        time: Some(std::time::SystemTime::now()),
        value: 1.0_f64,
        exemplars: vec![],
    };

    let sum: Sum<f64> = Sum {
        data_points: vec![data_point.clone(), data_point.clone(), data_point],
        temporality: Temporality::Delta,
        is_monotonic: true,
    };

    let resource_metrics = ResourceMetrics {
        resource: Resource::new(vec![KeyValue::new("service.name", "my-service")]),
        scope_metrics: vec![ScopeMetrics {
            scope: InstrumentationScope::default(),
            metrics: vec![Metric {
                name: "metric_name".into(),
                description: "metric description".into(),
                unit: "metric unit".into(),
                data: Box::new(sum),
            }],
        }],
    };

    resource_metrics
}

fn criterion_benchmark(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    c.bench_function("export", |b| {
        b.to_async(&runtime)
            .iter(|| export(create_resource_metrics()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
