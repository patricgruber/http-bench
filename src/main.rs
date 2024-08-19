use clap::{arg, command, Parser};
use threadpool::ThreadPool;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL to request
    #[arg(short, long)]
    url: String,

    /// Number of requests to send
    #[arg(short, long, default_value_t = 1000)]
    request_count: usize,

    /// Amount of threads to spawn
    #[arg(short, long, default_value_t = 1)]
    thread_count: usize,
}

fn main() {
    let args = Args::parse();
    let pool = ThreadPool::new(args.thread_count);

    for _ in 0..args.request_count {
        let url = args.url.clone();
        pool.execute(move || {
            if let Err(err) = reqwest::blocking::get(url) {
                eprintln!("{}", err.without_url());
            }
        });
    }

    pool.join();
}
