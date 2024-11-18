# ML Tracker

A Rust-native machine learning experiment tracking system inspired by MLflow. Track experiments, metrics, and artifacts with local or cloud storage support.

## Features

- Experiment & run management
- Real-time metric tracking
- System metrics monitoring (CPU, Memory, GPU)
- Artifact storage (local filesystem and S3)
- SQLite database backend
- CLI interface
- Async/await support
- Batched metric logging
- Proper shutdown handling

## Quick Start

```rust
let tracker = ExperimentTracker::new(Config::default()).await?;
let experiment = tracker.create_experiment("mnist_training")?;
let mut run = experiment.start_run()?;

// Log metrics
run.log_metric("loss", 2.5).await?;
run.log_metric("accuracy", 0.95).await?;

// Store artifacts
run.log_artifact("model", "model.pt", &model_bytes).await?;

run.finish()?;
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ml-tracker = "0.1.0"
```

## Configuration

```rust
let config = Config::default()
    .with_storage_path("./experiments")
    .with_database_url("sqlite:experiments.db")
    .with_metric_buffer_size(1000)
    .with_metric_flush_interval(Duration::from_secs(60));
```

## Features Matrix

| Feature | Local | S3 |
|---------|-------|-----|
| Metric Tracking | ✓ | ✓ |
| Artifact Storage | ✓ | ✓ |
| System Metrics | ✓ | ✓ |
| Database | SQLite | SQLite |

## Examples

See the `examples/` directory for:
- Basic tracking
- Custom metrics
- Distributed training

## Testing

```bash
cargo test
cargo bench
```

## Performance

- Batched metric logging
- Connection pooling
- Configurable buffer sizes
- Async I/O

## Contributing

1. Fork the repository
2. Create your feature branch
3. Run tests: `cargo test`
4. Submit a pull request

## License

MIT