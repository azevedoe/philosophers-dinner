# Philosopher

Implementação do problema do **Jantar dos Filósofos** em Rust, com duas abordagens de sincronização usando [Tokio](https://tokio.rs/).

## Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) (com `cargo` instalado)

## Como executar

Clone o repositório e entre na pasta do projeto:

```bash
cd philosopher
```

### Memória compartilhada (`Arc` + `Mutex`)

Versão padrão do projeto. Os filósofos compartilham garfos via `Arc<Mutex<Fork>>` e evitam deadlock ordenando a aquisição dos locks.

```bash
cargo run --bin shared_memory
```

Ou simplesmente:

```bash
cargo run
```

### Canais MPSC

Versão alternativa que usa canais assíncronos (`mpsc`) para comunicar os pensamentos dos filósofos e `try_lock` para pegar os garfos.

```bash
cargo run --bin mpsc
```

## Binários disponíveis


| Binário         | Descrição                                  |
| --------------- | ------------------------------------------ |
| `shared_memory` | Sincronização com `Arc` + `Mutex` (padrão) |
| `mpsc`          | Sincronização com canais MPSC e `try_lock` |


## Outros comandos úteis

```bash
# Compilar sem executar
cargo build

# Executar em modo release (otimizado)
cargo run --release --bin shared_memory

# Rodar os testes
cargo test
```

