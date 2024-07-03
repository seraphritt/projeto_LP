# Confiabilidade

## Verificação de Tipos

Rust realiza verificação de tipos em tempo de compilação de forma rigorosa, o que ajuda a capturar muitos erros comuns antes mesmo de executar o programa.


Em Rust:
```rust
fn main() {
    let x: i32 = "hello"; // Erro de tipo: esperava i32, encontrou &str
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

Rust possui um sistema de tipos muito forte e seguro, verificando a maioria dos erros de tipos em tempo de compilação. Isso ajuda a evitar muitos bugs comuns antes mesmo de executar o programa. As verificações de tipos são realizadas de forma rigorosa, o que contribui para código mais robusto e menos propenso a erros de execução relacionados a tipos.

## Tratamento de Exceções

Rust utiliza o conceito de Resultados (Result) para lidar com erros de forma explícita, evitando exceções tradicionais. Isso promove um código mais previsível e menos propenso a falhas inesperadas.

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

Rust não possui um sistema tradicional de exceções como C++, optando por utilizar o tipo Result para lidar com erros de forma explícita. Isso promove um código mais previsível e menos suscetível a falhas inesperadas devido a exceções não tratadas.

## Aliasing

Rust é forte em prevenir problemas de aliasing, especialmente com o sistema de propriedade (ownership) que garante que apenas uma referência pode modificar dados compartilhados por vez.

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

Rust é forte na prevenção de problemas de aliasing através do sistema de propriedade (ownership) e empréstimos (borrowing). Esses conceitos garantem que apenas uma referência mutável ou várias referências imutáveis possam existir para um único dado recurso ao mesmo tempo, reduzindo drasticamente a possibilidade de erros relacionados a aliasing.

## Legibilidade e Capacidade de Escrita

Ambas as linguagens têm preocupações com legibilidade, mas Rust tem uma sintaxe mais moderna e regras de estilo que favorecem a segurança e a clareza do código.

Em Rust:
```rust
fn main() {
    let mut sum = 0;
    for i in 1..=10 {
        sum += i;
    }
    println!("Soma: {}", sum);
}
```
Em C++:
```c++
#include <iostream>
using namespace std;

int main() {
    int sum = 0;
    for (int i = 1; i <= 10; ++i) {
        sum += i;
    }
    cout << "Soma: " << sum << endl;
    return 0;
}
```

Rust é projetado com uma sintaxe moderna e regras de estilo que promovem a legibilidade do código. A linguagem enfatiza o uso de padrões de programação seguros e claros, tornando mais fácil para os desenvolvedores escreverem e entenderem código complexo.

## Comentário geral
Rust é geralmente considerada mais segura e menos propensa a erros comuns de programação em comparação com C++. A forte verificação de tipos em tempo de compilação, juntamente com a prevenção de problemas de aliasing, fazem de Rust uma escolha preferida para projetos que exigem alta confiabilidade e segurança. Além disso, o tratamento explícito de erros e a ênfase na legibilidade contribuem para um desenvolvimento mais robusto e sustentável.