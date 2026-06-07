use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
}

impl Philosopher {
    fn think(&self) {
        println!("{} está pensando...", self.name);
        thread::sleep(Duration::from_millis(1));
    }

    fn eat(&self) {
        let (first, second) = if Arc::as_ptr(&self.left_fork) < Arc::as_ptr(&self.right_fork) {
            (&self.left_fork, &self.right_fork)
        } else {
            (&self.right_fork, &self.left_fork)
        };

        let _left = first.lock().unwrap();
        let _right = second.lock().unwrap();

        println!("{} está comendo...", self.name);
        thread::sleep(Duration::from_millis(5));
        println!("{} terminou de comer", self.name);
    }
}

static PHILOSOPHERS: &[&str] = &["Sócrates", "Kant", "Platão", "Aristóteles", "Pitágoras"];

fn main() {
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

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|phil| {
            thread::spawn(move || {
                for _ in 0..1 {
                    phil.think();
                    phil.eat();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Jantar encerrado (shared_memory).");
    println!("Tempo total: {:.2?}", start.elapsed());
}
