
# Legibilidade

## Simplicidade Global

Rust tem uma boa simplicidade global por possuir uma estrutura bem definida na definição de variáveis e execução de operações.

Em Rust:
```rust
fn main() {
    let mut x = 5; // declaração e inicialização de uma variável mutável
    x += 2; // incremento
    println!("O valor de x é: {}", x); // saída de texto
}
```
Em C++:
```c++
#include <iostream>

int main() {
    int x = 5; // declaração e inicialização de uma variável
    x += 2; // incremento
    std::cout << "O valor de x é: " << x << std::endl; // saída de texto
    return 0;
}
```
Ambos os exemplos são relativamente simples. Rust pode parecer mais explícito em certos aspectos (como o uso de mut para variáveis mutáveis), mas isso também ajuda na legibilidade ao indicar claramente a mutabilidade.

## Ortogonalidade

Rust promove uma boa ortogonalidade, onde combinações de conceitos primitivos são significativas e consistentes. Isso significa que conceitos como propriedades de mutabilidade, gerenciamento de memória e segurança são tratados de forma consistente em todo o código.

Em Rust:
```rust
struct Pessoa {
    nome: String,
    idade: u32,
}

impl Pessoa {
    fn novo(nome: String, idade: u32) -> Self {
        Pessoa { nome, idade }
    }

    fn cumprimentar(&self) {
        println!("Olá, meu nome é {} e eu tenho {} anos.", self.nome, self.idade);
    }
}

fn main() {
    let mut pessoa = Pessoa::novo(String::from("Alice"), 30); // criação de uma instância de Pessoa
    pessoa.cumprimentar(); // chamada de método imutável
}

```

Em C++:
```c++
#include <iostream>
#include <string>

class Pessoa {
private:
    std::string nome;
    int idade;

public:
    Pessoa(std::string nome, int idade) : nome(nome), idade(idade) {}

    void cumprimentar() const {
        std::cout << "Olá, meu nome é " << nome << " e eu tenho " << idade << " anos." << std::endl;
    }
};

int main() {
    Pessoa pessoa("Alice", 30); // criação de uma instância de Pessoa
    pessoa.cumprimentar(); // chamada de método constante
    return 0;
}
```

Em ambos os exemplos, a ortogonalidade é evidente na forma como Rust e C++ lidam com os conceitos fundamentais de orientação a objetos e gerenciamento de recursos. Rust impõe regras rigorosas para garantir segurança e prevenção de erros, enquanto C++ oferece mais flexibilidade, mas requer maior responsabilidade do programador para garantir a segurança e a eficiência do código.

## Instruções de Controle

Rust evita construções como goto que podem complicar o fluxo de controle. Em vez disso, incentiva o uso de estruturas de controle mais seguras e previsíveis, como if, match e loops.

Em Rust:
```rust
fn main() {
    let numero = 42;

    if numero < 0 {
        println!("O número é negativo");
    } else if numero > 0 {
        println!("O número é positivo");
    } else {
        println!("O número é zero");
    }

    let resposta = match numero {
        0 => "Zero",
        1 => "Um",
        _ => "Outro",
    };

    println!("Resposta: {}", resposta);

    for i in 0..5 {
        println!("Iteração {}", i);
    }
}
```
Em C++:
```c++
#include <iostream>

int main() {
    int numero = 42;

    if (numero < 0) {
        std::cout << "O número é negativo" << std::endl;
    } else if (numero > 0) {
        std::cout << "O número é positivo" << std::endl;
    } else {
        std::cout << "O número é zero" << std::endl;
    }

    switch (numero) {
        case 0:
            std::cout << "Zero" << std::endl;
            break;
        case 1:
            std::cout << "Um" << std::endl;
            break;
        default:
            std::cout << "Outro" << std::endl;
            break;
    }

    for (int i = 0; i < 5; ++i) {
        std::cout << "Iteração " << i << std::endl;
    }

    return 0;
}
```
* **if**, **else** **if**, **else** *versus* **if**, **else if**, **else**: Tanto em Rust quanto em C++, a estrutura condicional if, else if, else é utilizada para ramificar o fluxo de controle com base em condições. Rust e C++ são bastante semelhantes nesse aspecto, promovendo clareza na expressão das condições.

* **match** vs **switch**: Em Rust, match é uma expressão poderosa que cobre todas as possibilidades de um valor de maneira segura e exaustiva. No exemplo, _ é usado para capturar todos os outros casos não especificados. Em C++, switch é usado para a mesma finalidade, mas requer explicitamente um break para evitar a execução contínua de casos subsequentes.

* ***Loops***: Ambas as linguagens suportam *loops* for para iterar sobre uma faixa de valores. Rust usa .. para gerar uma faixa exclusiva (0 até 4 no exemplo), enquanto C++ usa < para definir a condição de término do loop.

Rust, ao evitar construções como goto, promove um código mais estruturado e legível, incentivando o uso de construções de controle mais seguras e expressivas. C++ oferece mais flexibilidade em algumas áreas, mas também permite o uso de goto, o que pode complicar o entendimento e a manutenção do código, se mal utilizado.

Em resumo, Rust e C++ proporcionam ferramentas poderosas para controle de fluxo, mas Rust, com suas abordagens mais modernas e restritivas, tende a promover práticas de programação mais seguras e legíveis, especialmente em cenários onde o controle de fluxo é crucial para a clareza do código.

## Tipos de Dados e Estruturas

Rust oferece tipos de dados poderosos e seguros, como enums, structs e tuples, que são úteis e adequados para definir estruturas complexas de dados de forma clara e concisa.

Em Rust:
```rust
// Definindo uma enumeração para representar diferentes tipos de formas geométricas
enum Forma {
    Retangulo { largura: u32, altura: u32 },
    Circulo(f64),
    Quadrado(u32),
}

// Definindo uma estrutura para representar uma pessoa
struct Pessoa {
    nome: String,
    idade: u8,
}

// Definindo uma tupla para armazenar coordenadas
struct Coordenadas(i32, i32);

fn main() {
    // Exemplos de uso dos tipos definidos
    let retangulo = Forma::Retangulo { largura: 30, altura: 50 };
    let circulo = Forma::Circulo(3.5);
    let pessoa = Pessoa { nome: String::from("Alice"), idade: 25 };
    let coordenadas = Coordenadas(10, 20);

    match retangulo {
        Forma::Retangulo { largura, altura } => {
            println!("Retângulo: largura = {}, altura = {}", largura, altura);
        }
        _ => {}
    }

    match circulo {
        Forma::Circulo(raio) => {
            println!("Círculo: raio = {}", raio);
        }
        _ => {}
    }

    println!("Pessoa: {} tem {} anos.", pessoa.nome, pessoa.idade);
    println!("Coordenadas: ({}, {})", coordenadas.0, coordenadas.1);
}
```
Em C++:
```c++
#include <iostream>
#include <string>
#include <tuple>

// Definindo uma enumeração para representar diferentes tipos de formas geométricas
enum class Forma {
    Retangulo,
    Circulo,
    Quadrado,
};

// Definindo uma estrutura para representar uma pessoa
struct Pessoa {
    std::string nome;
    unsigned int idade;
};

// Usando uma tupla para armazenar coordenadas
typedef std::tuple<int, int> Coordenadas;

int main() {
    // Exemplos de uso dos tipos definidos
    Forma forma = Forma::Retangulo;
    Pessoa pessoa = {"Alice", 25};
    Coordenadas coordenadas = std::make_tuple(10, 20);

    switch (forma) {
        case Forma::Retangulo:
            std::cout << "Retângulo" << std::endl;
            break;
        case Forma::Circulo:
            std::cout << "Círculo" << std::endl;
            break;
        case Forma::Quadrado:
            std::cout << "Quadrado" << std::endl;
            break;
    }

    std::cout << "Pessoa: " << pessoa.nome << " tem " << pessoa.idade << " anos." << std::endl;
    std::cout << "Coordenadas: (" << std::get<0>(coordenadas) << ", " << std::get<1>(coordenadas) << ")" << std::endl;

    return 0;
}
```

* Enums: Rust usa enums de forma poderosa, permitindo a definição de variantes com dados associados, como Forma::Retangulo { largura, altura }. C++ também tem enums, mas não suporta dados associados sem o uso de classes.
* Structs: Ambas as linguagens suportam structs para definir estruturas de dados. Rust permite métodos e implementações diretamente associadas às structs, enquanto em C++ é necessário usar classes para isso.
* Tuples: Rust e C++ suportam tuples para agrupar múltiplos valores em um único tipo. Rust usa a sintaxe (valor1, valor2) para criar e acessar tuples, enquanto C++ usa std::tuple e std::get para o mesmo propósito.

Rust se destaca na segurança e na expressividade de seus tipos de dados, garantindo que cada valor tenha um único proprietário (propriedade de propriedade). Isso ajuda a prevenir erros comuns de gerenciamento de memória, como vazamentos e referências inválidas. Em contraste, C++ oferece mais flexibilidade, mas com um custo maior em termos de gerenciamento de memória seguro e legibilidade.

## Aspectos da Sintaxe

Rust possui uma sintaxe moderna que é projetada para ser clara e expressiva. Por exemplo, utiliza palavras-chave claras e não sobrecarrega o uso de operadores especiais. Identificadores descritivos são incentivados, o que melhora a legibilidade.

Comparando com C++, Rust muitas vezes pode parecer mais verboso em certos aspectos (como o uso de mut para indicar mutabilidade), mas essa verbosidade também pode melhorar a legibilidade ao tornar as intenções do código mais claras. C++ tende a ser mais flexível em alguns aspectos, mas essa flexibilidade pode levar a códigos menos legíveis se não for usada com cuidado.

Em resumo, Rust foi projetado com foco na legibilidade, segurança e performance, procurando equilibrar simplicidade com poder. Embora seja diferente de C++ em muitos aspectos, ambos têm seus méritos e são capazes de produzir código legível, desde que os desenvolvedores sigam boas práticas de codificação
