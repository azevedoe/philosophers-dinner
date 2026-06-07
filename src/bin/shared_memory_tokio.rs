use std::sync::Arc;
use std::time::Instant;

use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
}

impl Philosopher {
    async fn think(&self) {
        println!("{} está pensando...", self.name);
        sleep(Duration::from_millis(500)).await;
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
        sleep(Duration::from_millis(1000)).await;
        println!("{} terminou de comer", self.name);
    }
}

static PHILOSOPHERS: &[&str] = &["Sócrates", "Kant", "Platão", "Aristóteles", "Pitágoras"];

#[tokio::main]
async fn main() {
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
            for _ in 0..1 {
                phil.think().await;
                phil.eat().await;
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Jantar encerrado (shared_memory_tokio).");
    println!("Tempo total: {:.2?}", start.elapsed());
}
