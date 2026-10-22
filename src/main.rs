struct Job {
    name: String,
    id: u16,
}
impl Job {
    fn new(name: &str, id: u16) -> Job { // makes a job that owns its name
        Job {
            name: String::from(name),
            id,
        }
    }

    fn show(&self) {
        println!("{:<16} id=0x{:04X}", self.name, self.id);
    }
}

struct JobRun<'a> {
    job: &'a Job, // the job must stay alive for as long as this reference is used
    attempt: u8,
}
impl<'a> JobRun<'a> {
    fn new(job: &'a Job, attempt: u8) -> JobRun<'a> {
        assert!(attempt > 0, "attempts start at one");
        JobRun { job, attempt }
    }

    fn show(&self) { // read the original job through the reference
        println!("{:<16} attempt={} id=0x{:04X}", self.job.name, self.attempt, self.job.id);
    }
}

fn finishRun<'a>(run: JobRun<'a>) -> &'a Job { // takes the run by value and returns its borrowed job
    run.show();
    println!("Completed {}", run.job.name);
    return run.job; // this points to the outer job, so it can outlive the run
}

fn main() {
    let job = Job::new("backup", 0x1234);
    println!("Job before execution:");
    job.show();

    {
        let run = JobRun::new(&job, 1); // the run lives in this inner scope
        println!("\nExecution:");
        assert!(std::ptr::eq(run.job, &job)); // borrowing the same job, not a copy
        let completedJob = finishRun(run);
        assert!(std::ptr::eq(completedJob, &job));
        assert_eq!(completedJob.name, "backup");
        assert_eq!(completedJob.id, 0x1234);
    }

    println!("\nJob after the run's scope has ended:");
    job.show(); // the owner is still alive after the run has been consumed
}
