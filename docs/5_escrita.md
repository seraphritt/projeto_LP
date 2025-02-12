# Capacidade de Escrita

## Simplicidade e Ortogonalidade

<div style="text-align: justify; margin-bottom: 1em;">Rust é projetada com poucos componentes básicos e regras consistentes para combiná-los, o que facilita a aprendizagem e a escrita de código robusto. A ortogonalidade em Rust é moderada; ela enfatiza a segurança e o gerenciamento de memória seguro, o que pode requerer um pouco mais de entendimento para programadores novatos, mas traz benefícios significativos em termos de prevenção de erros.</div>


## Suporte para Abstração

<div style="text-align: justify; margin-bottom: 1em;">Rust oferece suporte robusto para abstração de dados e processo e possui suporte para classes e registros. Ela permite a definição de estruturas complexas e operações abstratas através de mecanismos como traits (traços) e enums, o que facilita a expressão de ideias complexas de forma clara e segura.</div>

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
<div style="text-align: justify; margin-bottom: 1em;">Rust suporta abstração com traits, permitindo definir comportamentos comuns que podem ser implementados por diferentes tipos de dados de forma segura.</div>

## Expressividade

<div style="text-align: justify; margin-bottom: 1em;">Rust é conhecida por sua expressividade, proporcionando formas convenientes de especificar computações. Seu sistema de tipos forte e sua inferência de tipos ajudam a reduzir a sobrecarga de escrita de código repetitivo.</div>

<div style="text-align: justify; margin-bottom: 1em;">C++ oferece expressividade através de funcionalidades como sobrecarga de operadores e templates, mas pode resultar em código mais verboso e propenso a erros.</div>

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

<div style="text-align: justify; margin-bottom: 1em;">Ambos exemplos mostram como Rust e C++ podem ser expressivos ao manipular dados e realizar computações. Rust se destaca pela sua sintaxe mais moderna e segura, especialmente ao lidar com manipulação de coleções e closures. Por outro lado, C++ oferece uma rica biblioteca padrão (STL) que permite realizar tarefas similares, mas com uma sintaxe que pode exigir mais código para atingir o mesmo resultado.</div>

<div style="text-align: justify; margin-bottom: 1em;">Em termos de capacidade de escrita, Rust geralmente proporciona uma experiência mais fluida, especialmente para desenvolvedores que valorizam a prevenção de erros e a clareza do código.</div>

## Sintaxe

<div style="text-align: justify; margin-bottom: 1em;">
  A sintaxe em Rust é projetada para ser expressiva, usando um sistema de empréstimos (passagem por referência) e movimentos que ajuda a evitar problemas comuns de gerenciamento de memória, bem como instruções de inferência de tipo que auxiliam na <strong>capacidade de escrita</strong>.
</div>

<div style="text-align: justify; margin-bottom: 1em;">C++ tem uma sintaxe flexível, mas pode ser mais verbosa e menos segura se não for utilizada corretamente, especialmente em relação ao gerenciamento de memória.</div>

Em Rust:
```rust
#[derive(Debug)] // utilizado para inferir como imprimir a struct
struct Car {
    brand: String,
    model: String,
    year: u32,
}

// Função que calcula o preço estimado com base no ano do carro
fn calculate_price(car: &Car) -> u32 {
    let base_price = 10_000; // Preço base arbitrário

    // Calculando o preço com base no ano do carro
    let depreciation = (2024 - car.year) * 100; // Depreciação arbitrária por ano
    base_price - depreciation
}

fn main() {
    // Criando um carro
    let my_car = Car {
        brand: String::from("Toyota"),
        model: String::from("Corolla"),
        year: 2018,
    };

    // Chamando a função calculate_price e passando uma referência emprestada para my_car
    let estimated_price = calculate_price(&my_car);

    println!(
        "O preço estimado do {} {} de {} é de {} reais.",
        my_car.brand, my_car.model, my_car.year, estimated_price
    );
    println!("{:?}", my_car);
}
```

Em C++:
```c++
#include <iostream>
#include <string>

// Definindo uma estrutura de dados simples
struct Car {
    std::string brand;
    std::string model;
    int year;
};

// Função que calcula o preço estimado com base no ano do carro
int calculate_price(const Car& car) {
    int base_price = 10000; // Preço base arbitrário

    // Calculando o preço com base no ano do carro
    int depreciation = (2024 - car.year) * 100; // Depreciação arbitrária por ano
    return base_price - depreciation;
}

int main() {
    // Criando um carro
    Car my_car = {"Toyota", "Corolla", 2018};

    // Chamando a função calculate_price e passando uma referência constante para my_car
    int estimated_price = calculate_price(my_car);

    std::cout << "O preço estimado do " << my_car.brand << " " << my_car.model << " de " << my_car.year
              << " é de " << estimated_price << " reais." << std::endl;

    return 0;
}

```