mod bench_config {
    #![allow(dead_code)]
    include!("../bench_config.rs");
}

use std::sync::Arc;
use std::time::Instant;

use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use bench_config::*;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
}

impl Philosopher {
    async fn think(&self) {
        println!("{} está pensando...", self.name);
        sleep(Duration::from_millis(THINK_MS)).await;
    }

    async fn eat(&self) {
        let (first, second) = if Arc::as_ptr(&self.left_fork) < Arc::as_ptr(&self.right_fork) {
            (&self.left_fork, &self.right_fork)
        } else {
            (&self.right_fork, &self.left_fork)
        };

        let _left = first.lock().await;
        let _right = second.lock().await;

        println!("{} está comendo...", self.name);
        sleep(Duration::from_millis(EAT_MS)).await;
        println!("{} terminou de comer", self.name);
    }
}

fn print_run_config() {
    println!("=== shared_memory_tokio (tasks) ===");
    println!(
        "THINK_MS={THINK_MS} EAT_MS={EAT_MS} CYCLES={CYCLES} filósofos={}",
        PHILOSOPHERS.len()
    );
}

#[tokio::main]
async fn main() {
    print_run_config();
    let start = Instant::now();

    let mut forks = vec![];
    (0..PHILOSOPHERS.len()).for_each(|_| forks.push(Arc::new(Mutex::new(Fork))));

    let philosophers: Vec<Philosopher> = PHILOSOPHERS
        .iter()
        .enumerate()
        .map(|(i, name)| Philosopher {
            name: name.to_string(),
            left_fork: Arc::clone(&forks[i]),
            right_fork: Arc::clone(&forks[(i + 1) % PHILOSOPHERS.len()]),
        })
        .collect();

    let mut handles = Vec::new();

    for phil in philosophers {
        handles.push(tokio::spawn(async move {
            for _ in 0..CYCLES {
                phil.think().await;
                phil.eat().await;
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Jantar encerrado (shared_memory_tokio / tokio).");
    println!("Tempo total: {:.2?}", start.elapsed());
}
