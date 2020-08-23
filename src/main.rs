use hippo::jvm::Jvm;
use std::env;
use std::fs::OpenOptions;
use std::io::BufWriter;
use tracing_subscriber::EnvFilter;

fn main() {
    env::set_var("RUST_LOG", "debug");
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("hippo.log")
        .unwrap();
    let writer = BufWriter::new(file);
    let (non_blocking, _guard) = tracing_appender::non_blocking(writer);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .json()
        .init();
    deadlock_detector();

    let mut jvm = Jvm::default();
    jvm.run("main/Main");
}

fn deadlock_detector() {
    // only for #[cfg]
    use parking_lot::deadlock;
    use std::thread;
    use std::time::Duration;

    // Create a background thread which checks for deadlocks every 10s
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10));
        let deadlocks = deadlock::check_deadlock();
        if deadlocks.is_empty() {
            continue;
        }

        println!("{} deadlocks detected", deadlocks.len());
        for (i, threads) in deadlocks.iter().enumerate() {
            println!("Deadlock #{}", i);
            for t in threads {
                println!("Thread Id {:#?}", t.thread_id());
                println!("{:#?}", t.backtrace());
            }
        }
    });
}
