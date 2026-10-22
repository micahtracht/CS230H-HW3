# CS230H Homework 3: Job Lifetimes

A small Rust simulation using the job scheduler domain from HW2. All code is
in `src/main.rs`, with no dependencies or actual external jobs.

## Run

```sh
cargo run
cargo test
```

## Structs and lifetimes

`Job` owns a job's name and ID. `JobRun<'a>` represents one execution attempt
and contains `job: &'a Job`, a reference to the original job. An execution
attempt depends on the job it is executing.

The lifetime parameter `'a` ties the reference in `JobRun` to the borrowed
`Job`. Rust prevents the run from using that reference after the job is gone;
the annotation describes this relationship rather than extending any lifetime.

`main` creates the job in the outer scope and its run in an inner scope.
Assertions check that the run references the original job and that its data
is preserved. After the inner scope ends, `main` prints the job again to show
that the owner is still available. Run `cargo run` to execute these checks.

`finishRun<'a>(run: JobRun<'a>) -> &'a Job` takes the referencing struct by
value, prints its completion, and returns the reference to the original job.
The explicit lifetime ties the returned reference to the job borrowed by the
run. The run is consumed by the function, but the outer job remains alive.
