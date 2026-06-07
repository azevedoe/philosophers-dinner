// Parâmetros compartilhados para comparar os 3 binários com as mesmas condições.
//
// Ajuste os valores abaixo, salve e rode:
//
//   cargo run --bin shared_memory
//   cargo run --bin shared_memory_tokio
//   cargo run --bin mpsc
//
// Para medição mais estável, use release:
//
//   cargo run --release --bin shared_memory
//   cargo run --release --bin shared_memory_tokio
//   cargo run --release --bin mpsc

/// Tempo simulado pensando (milissegundos).
pub const THINK_MS: u64 = 100;

/// Tempo simulado comendo (milissegundos).
pub const EAT_MS: u64 = 500;

/// Quantas vezes cada filósofo repete pensar → comer.
pub const CYCLES: u32 = 1;

/// Espera entre tentativas de `try_lock` (apenas `mpsc`).
pub const TRY_LOCK_RETRY_MS: u64 = 1;

/// Nomes dos filósofos (define quantos participam da mesa).
pub const PHILOSOPHERS: &[&str] =
    &["Sócrates", "Kant", "Platão", "Aristóteles", "Pitágoras"];

/// Capacidade do buffer do canal `mpsc`.
pub const MPSC_CHANNEL_BUFFER: usize = 32;
