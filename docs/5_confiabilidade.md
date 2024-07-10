# Confiabilidade

## Verificação de Tipos

Rust realiza verificação de tipos em tempo de compilação de forma ainda mais rigorosa do que o C++, verificando também propriedades como a mutabilidade de uma variável e a reutilização dela em outras partes do código.

Em Rust:
```rust
fn main() {
    let x: i32 = "hello"; // Erro de tipo: esperava i32, encontrou &str
    x = 64; // Erro do tipo: não pode haver atribuição dupla em uma variável imutável (por padrão, todas as variáveis são imutáveis.
    println!("Hello, world!");
}
```

Em C++:
```c++
#include <iostream>
using namespace std;

int main() {
    int x = "hello"; // Erro de tipo: conversão de const char* para int
    cout << "Hello, world!" << endl;
    return 0;
}
```

## Tratamento de Exceções

Rust utiliza o conceito de Resultados (Result) para lidar com erros de forma explícita, evitando exceções tradicionais, o que promove um código mais previsível e menos propenso a falhas inesperadas.

Em Rust:
```rust
fn main() -> Result<(), String> {
    let result: Result<i32, String> = Err("Erro ao processar".to_string());
    match result {
        Ok(value) => println!("Valor: {}", value),
        Err(msg) => println!("Erro: {}", msg),
    }
    Ok(())
}
```
Em C++:
```c++
#include <iostream>
using namespace std;

int main() {
    try {
        throw runtime_error("Erro ao processar");
    } catch (const exception& e) {
        cout << "Erro: " << e.what() << endl;
    }
    return 0;
}
```

## Aliasing

Rust é forte na prevenção de problemas de aliasing através do sistema de propriedade (ownership) e empréstimos (borrowing). Esses conceitos garantem que apenas uma referência mutável ou várias referências imutáveis possam existir para um único dado recurso ao mesmo tempo, reduzindo a possibilidade de erros relacionados a aliasing.

Em Rust:
```rust
fn main() {
    let mut x = 5;
    let y = &mut x;
    let z = &mut x; // Erro: não é permitido ter múltiplas referências mutáveis
    *y += 1;
    println!("x: {}", x);
}
```
Em C++:
```c++
#include <iostream>
using namespace std;

int main() {
    int x = 5;
    int& y = x;
    int& z = x; // Compilador permite múltiplas referências
    y += 1;
    cout << "x: " << x << endl;
    return 0;
}
```