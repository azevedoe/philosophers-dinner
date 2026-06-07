mod bench_config {
    #![allow(dead_code)]
    include!("../bench_config.rs");
}

use std::sync::Arc;
use std::time::Instant;

use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

use bench_config::*;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: Sender<String>,
}

impl Philosopher {
    async fn think(&self) {
        println!("{} está pensando...", self.name);
        time::sleep(Duration::from_millis(THINK_MS)).await;

        self.thoughts
            .send(format!("Eureka! {} tem uma nova ideia!", &self.name))
            .await
            .unwrap();
    }

    async fn eat(&self) {
        let (_left_fork, _right_fork) = loop {
            let left_fork = self.left_fork.try_lock();
            let right_fork = self.right_fork.try_lock();

            let Ok(left_fork) = left_fork else {
                drop(right_fork);
                time::sleep(Duration::from_millis(TRY_LOCK_RETRY_MS)).await;
                continue;
            };

            let Ok(right_fork) = right_fork else {
                drop(left_fork);
                time::sleep(Duration::from_millis(TRY_LOCK_RETRY_MS)).await;
                continue;
            };

            break (left_fork, right_fork);
        };

        println!("{} está comendo...", &self.name);
        time::sleep(Duration::from_millis(EAT_MS)).await;
        println!("{} terminou de comer", self.name);
    }
}

fn print_run_config() {
    println!("=== mpsc (tasks + canal) ===");
    println!(
        "THINK_MS={THINK_MS} EAT_MS={EAT_MS} CYCLES={CYCLES} TRY_LOCK_RETRY_MS={TRY_LOCK_RETRY_MS} \
         filósofos={}",
        PHILOSOPHERS.len()
    );
}

#[tokio::main]
async fn main() {
    print_run_config();
    let start = Instant::now();

    let mut forks = vec![];
    (0..PHILOSOPHERS.len()).for_each(|_| forks.push(Arc::new(Mutex::new(Fork))));

    let (philosophers, mut rx) = {
        let mut philosophers = vec![];
        let (tx, rx) = mpsc::channel(MPSC_CHANNEL_BUFFER);

        for (i, name) in PHILOSOPHERS.iter().enumerate() {
            philosophers.push(Philosopher {
                name: name.to_string(),
                left_fork: Arc::clone(&forks[i]),
                right_fork: Arc::clone(&forks[(i + 1) % PHILOSOPHERS.len()]),
                thoughts: tx.clone(),
            });
        }

        (philosophers, rx)
    };

    for phil in philosophers {
        tokio::spawn(async move {
            for _ in 0..CYCLES {
                phil.think().await;
                phil.eat().await;
            }
        });
    }

    while let Some(thought) = rx.recv().await {
        println!("Aqui está um pensamento: {thought}");
    }

    println!("Jantar encerrado (mpsc).");
    println!("Tempo total: {:.2?}", start.elapsed());
}
