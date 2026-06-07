use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;
use tokio::time;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: Sender<String>,
}

impl Philosopher {
    async fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} tem uma nova ideia!", &self.name))
            .await
            .unwrap();
    }

    async fn eat(&self) {
        // Continue tentando até termos ambos os garfos
        let (_left_fork, _right_fork) = loop {
            // Peguem os garfos...
            let left_fork = self.left_fork.try_lock();
            let right_fork = self.right_fork.try_lock();
            let Ok(left_fork) = left_fork else {
                // Se não pegamos o garfo esquerdo, solte o garfo direito se o
                // tivermos e deixe outras tarefas progredirem.
                drop(right_fork);
                time::sleep(time::Duration::from_millis(1)).await;
                continue;
            };
            let Ok(right_fork) = right_fork else {
                // Se não pegamos o garfo direito, solte o garfo esquerdo e deixe
                // outras tarefas progredirem.
                drop(left_fork);
                time::sleep(time::Duration::from_millis(1)).await;
                continue;
            };
            break (left_fork, right_fork);
        };

        println!("{} está comendo...", &self.name);
        time::sleep(time::Duration::from_millis(5)).await;

        // Os _locks_ são descartados aqui
    }
}

static PHILOSOPHERS: &[&str] =
    &["Sócrates", "Hipátia", "Platão", "Aristóteles", "Pitágoras"];

#[tokio::main]
async fn main() {
    let start = Instant::now();

    // Criem os garfos
    let mut forks = vec![];
    (0..PHILOSOPHERS.len()).for_each(|_| forks.push(Arc::new(Mutex::new(Fork))));

    // Criem os filósofos
    let (philosophers, mut rx) = {
        let mut philosophers = vec![];
        let (tx, rx) = mpsc::channel(10);
        for (i, name) in PHILOSOPHERS.iter().enumerate() {
            let left_fork = Arc::clone(&forks[i]);
            let right_fork = Arc::clone(&forks[(i + 1) % PHILOSOPHERS.len()]);
            philosophers.push(Philosopher {
                name: name.to_string(),
                left_fork,
                right_fork,
                thoughts: tx.clone(),
            });
        }
        (philosophers, rx)
        // tx é descartado aqui, então não precisamos descartá-lo explicitamente mais tarde
    };

    // Faça-os pensar e comer
    for phil in philosophers {
        tokio::spawn(async move {
            for _ in 0..1 {
                phil.think().await;
                phil.eat().await;
            }
        });
    }

    // Imprimam seus pensamentos
    while let Some(thought) = rx.recv().await {
        println!("Aqui está um pensamento: {thought}");
    }

    println!("Jantar encerrado (mpsc).");
    println!("Tempo total: {:.2?}", start.elapsed());
}