# Philosopher

Implementação do problema do **Jantar dos Filósofos** em Rust, com três abordagens de sincronização.

## Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) (com `cargo` instalado)

## Como executar

Clone o repositório e entre na pasta do projeto:

```bash
cd philosopher
```

### Memória compartilhada — std (`Arc` + `Mutex`)

Versão padrão do projeto, usando apenas a biblioteca padrão (`std::thread`, `std::sync::Mutex`). Os filósofos compartilham garfos via `Arc<Mutex<Fork>>` e evitam deadlock ordenando a aquisição dos locks.

```bash
cargo run --bin shared_memory
```

Ou simplesmente:

```bash
cargo run
```

### Memória compartilhada — Tokio (`Arc` + `Mutex` async)

Mesma lógica da versão std, mas com [Tokio](https://tokio.rs/) (`tokio::spawn`, `tokio::sync::Mutex`, `tokio::time::sleep`).

```bash
cargo run --bin shared_memory_tokio
```

### Canais MPSC

Versão que usa canais assíncronos (`mpsc`) para comunicar os pensamentos dos filósofos e `try_lock` para pegar os garfos.

```bash
cargo run --bin mpsc
```

## Binários disponíveis

| Binário               | Descrição                                              |
| --------------------- | ------------------------------------------------------ |
| `shared_memory`       | `Arc` + `Mutex` com threads std (padrão)               |
| `shared_memory_tokio` | `Arc` + `Mutex` async com Tokio                        |
| `mpsc`                | Canais MPSC + `try_lock` com Tokio                     |

Ao final de cada execução, o tempo total é exibido no terminal (`Tempo total: ...`).

## Outros comandos úteis

```bash
# Compilar sem executar
cargo build

# Executar em modo release (otimizado)
cargo run --release --bin shared_memory
cargo run --release --bin shared_memory_tokio

# Rodar os testes
cargo test
```
