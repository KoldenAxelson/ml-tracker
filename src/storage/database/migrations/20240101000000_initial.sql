CREATE TABLE experiments (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    tags TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    active_run BLOB
);

CREATE TABLE runs (
    id BLOB PRIMARY KEY,
    experiment_id BLOB NOT NULL,
    status TEXT NOT NULL,
    metrics TEXT NOT NULL,
    params TEXT NOT NULL,
    tags TEXT NOT NULL,
    artifacts TEXT NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP,
    FOREIGN KEY (experiment_id) REFERENCES experiments(id)
);

CREATE INDEX idx_runs_experiment_id ON runs(experiment_id);