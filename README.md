# Validador de CPF

Uma biblioteca Rust simples para validação de números de CPF brasileiros.

## Funcionalidades

- Aceita CPF com ou sem formatação.
- Remove automaticamente caracteres não numéricos.
- Verifica se o CPF possui 11 dígitos.
- Rejeita CPFs com todos os dígitos iguais.
- Valida os dois dígitos verificadores conforme o algoritmo oficial.

## Instalação

Adicione a dependência ao seu `Cargo.toml`:

```toml
[dependencies]
validador_cpf = "0.1.0"
```

## Uso

```rust
use validador_cpf::validar_cpf;

fn main() {
    println!("{}", validar_cpf("529.982.247-25"));
    println!("{}", validar_cpf("52998224725"));
}
```

## Exemplos

### CPF válido

```rust
assert!(validar_cpf("529.982.247-25"));
assert!(validar_cpf("52998224725"));
```

### CPF inválido

```rust
assert!(!validar_cpf("111.111.111-11"));
assert!(!validar_cpf("123.456.789-00"));
assert!(!validar_cpf("123"));
```

## API

### `validar_cpf`

```rust
pub fn validar_cpf(cpf: &str) -> bool
```

Recebe um CPF como string e retorna:

| Retorno | Significado |
|----------|-------------|
| `true` | CPF válido |
| `false` | CPF inválido |

## Como funciona

A validação segue as regras oficiais do CPF:

1. Remove caracteres não numéricos.
2. Verifica se existem exatamente 11 dígitos.
3. Rejeita sequências com todos os dígitos iguais.
4. Calcula o primeiro dígito verificador.
5. Calcula o segundo dígito verificador.
6. Compara os resultados com os dígitos informados.

## Licença

Este projeto está licenciado sob a licença MIT.
