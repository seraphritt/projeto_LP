# Capacidade de Escrita

## Simplicidade e Ortogonalidade

Rust é projetada com poucos componentes básicos e regras consistentes para combiná-los, o que facilita a aprendizagem e a escrita de código robusto. A ortogonalidade em Rust é moderada; ela enfatiza a segurança e o gerenciamento de memória seguro, o que pode requerer um pouco mais de entendimento para programadores novatos, mas traz benefícios significativos em termos de prevenção de erros.

Olhar página sobre legibilidade

## Suporte para Abstração

Rust oferece suporte robusto para abstração de dados e processo. Ela permite a definição de estruturas complexas e operações abstratas através de mecanismos como traits (traços) e enums, o que facilita a expressão de ideias complexas de forma clara e segura.

Em Rust:
```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

fn main() {
    let dog = Dog;
    dog.make_sound();
}
```

Em C++:
```c++
#include <iostream>
using namespace std;

class Animal {
public:
    virtual void make_sound() = 0;
};

class Dog : public Animal {
public:
    void make_sound() override {
        cout << "Woof!" << endl;
    }
};

int main() {
    Dog dog;
    dog.make_sound();
    return 0;
}
```
Rust suporta abstração com traits, permitindo definir comportamentos comuns que podem ser implementados por diferentes tipos de dados de forma segura.

## Expressividade

Rust é conhecida por sua expressividade, proporcionando formas convenientes de especificar computações de maneira eficiente e segura. Seu sistema de tipos forte e sua inferência de tipos ajudam a reduzir a sobrecarga de escrita de código repetitivo.

C++ oferece expressividade através de funcionalidades como sobrecarga de operadores e templates, mas pode resultar em código mais verboso e propenso a erros.

Em Rust:
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    let sum: i32 = numbers.iter().sum();
    let squares: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    
    println!("Sum: {}", sum);
    println!("Squares: {:?}", squares);
}
```

Em C++:
```c++
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

int main() {
    vector<int> numbers = {1, 2, 3, 4, 5};
    
    int sum = accumulate(numbers.begin(), numbers.end(), 0);
    
    vector<int> squares;
    transform(numbers.begin(), numbers.end(), back_inserter(squares), [](int x) { return x * x; });
    
    cout << "Sum: " << sum << endl;
    cout << "Squares: ";
    for (int num : squares) {
        cout << num << " ";
    }
    cout << endl;
    
    return 0;
}
```

Ambos os exemplos mostram como Rust e C++ podem ser expressivos ao manipular dados e realizar computações. Rust se destaca pela sua sintaxe mais moderna e segura, especialmente ao lidar com manipulação de coleções e closures. Por outro lado, C++ oferece uma rica biblioteca padrão (STL) que permite realizar tarefas similares, mas com uma sintaxe que pode exigir mais código para atingir o mesmo resultado.

Em termos de capacidade de escrita, Rust geralmente proporciona uma experiência mais fluida e segura, especialmente para desenvolvedores que valorizam a prevenção de erros e a clareza do código.

## Sintaxe

A sintaxe em Rust é projetada para ser expressiva e poderosa, usando um sistema de empréstimos e movimentos que ajuda a evitar problemas comuns de gerenciamento de memória. A restrição imposta pelo sistema de tipos contribui para código mais seguro.

C++ tem uma sintaxe flexível, mas pode ser mais verbosa e menos segura se não for utilizada corretamente, especialmente em relação ao gerenciamento de memória.

Olhar página sobre legibilidade