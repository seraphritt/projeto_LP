# Rust

## No que consiste a linguagem Rust?

Rust é uma linguagem de programação compilada, multiparadigma  (suporta diferentes paradigmas de programação, como programação imperativa, funcional, orientada a objetos, ...), com foco principal no desempenho e na segurança do sistema. Com um rico sistema de tipos - que podem ser declarados explicitamente ou inferidos pelo compilador - esta linguagem oferece bom equilíbrio entre segurança e perfomance. Seu principal diferencial está em seu método inovador de gerenciamento de memória e em seu robusto compilador, que detecta erros que interfiram na aplicação de projetos em tempo de compilação.

## Gerenciamento da memória

Algumas linguagens utilizam coletor de lixo para gerenciar a memória. Em outras (como C), o programador deve alocar e liberar memória de forma explícita. Para corrigir erros relacionados ao vazamento de dados, Rust se propõe a implementar uma novo método, através de seu sistema único de propriedade (ownership) e empréstimo (borrowing): 

### Ownership
Sistema de posse baseado em um conjunto de regras, verificadas em tempo de compilação. Cada valor em Rust é associado a uma única variável (owner). Quando essa variável sai do escopo, a memória alocada a ela é automaticamente liberada. Esse processo é eficiente e ocorre em tempo de compilação, o que reduz a possibilidade de vazamentos de memória e outros erros relacionados.

### Borrowing
Método que permite que uma variável seja acessada por referência sem perder sua propriedade (ownership). Este método garante que a memória seja gerenciada de forma segura e eficiente, evitando problemas como data races e garantindo maior confiabilidade comparado a linguagens como C.

### Partes da memória
Visto que a linguagem Rust utiliza um método distinto de gerenciamento e armazenamento de memória, é interessante pontuar suas principais áreas de armazenamento: a pilha (stack) e o heap:

* Pilha (Stack): Estrutura de memória que segue a lógica "Last In, First Out" (o último dado a ser inserido é o primeiro a ser removido). Esta importante estrutura tem como principais características:

    * Acesso Rápido: devido a seu método de alocação e liberação de memória, a pilha oferece um acesso muito mais rápido aos dados que a memória Heap.
    * Tamanho Fixo: Os dados armazenados na pilha devem ter um tamanho fixo e conhecido em tempo de compilação. Uma vez alocados, estes dados não podem mudar de tamanho. Por isso, é ideal para dados de tamanho fixo e temporários, como variáveis locais e chamadas de funções. Esta natureza dos dados permite uma rápida e eficiente gestão dos mesmos.

* Heap: Área de memória que permite a alocação dinâmica e flexível de dados. Ao contrário da pilha, esta parte da memória pode armazenar dados de tamanho variável. Esta estrutura tem como principais características:
    * Acesso mais lento: A alocação e liberação de memória no heap são mais lentas que na pilha. O sistema operacional é responsável por encontrar um bloco de memória suficientemente grande e alocar o espaço desejado, retornando um ponteiro para este local.
    * Alocação Dinâmica: Para armazenar dados cuja quantidade e tamanho não são conhecidos em tempo de compilação, a memória heap é a mais indicada. Programas que necessitam utilizar dados que possuem tamanho que pode ser alterado devem fazer uso desta parte da memória.
    * Armazenamento de Ponteiros: O ponteiro retornado pelo heap é armazenado na pilha. Assim, a pilha consegue controlar, de maneira mais rápida e eficaz, o acesso aos ponteiros para esses dados.

Em resumo, a pilha e o heap têm características e propósitos distintos. Porém, ambas são essenciais para a gestão eficiente da memória em Rust. A escolha entre qual área da memória utilizar irá depender dos requisitos específicos de cada programa. A escolha de uso, feita pelo desenvolvedor, irá influenciar diretamente a eficiência e o desempenho do código.

## Por que utilizar Rust?

### Segurança

* Sistema de tipos poderoso que previne erros comuns, como null pointer dereferences.
* Controle rigoroso sobre o gerenciamento de memória, evitando vazamentos e corrupção de memória.
* Semântica de propriedade e sistema de empréstimo que impede falhas de acesso concorrente (race conditions) e compartilhamento inseguro de dados.
* Uso extensivo de "Traits" para garantir comportamentos padronizados e seguros em estruturas de dados e operações.

### Desempenho

* Eficiência na gestão da memória, impedindo que data races ocorram.
* Memória com melhor desempenho.
* Abstrações de custo zero, garantindo maior produtividade sem prejudicar a perfomance.
* Análise e compilação do código feita em tempo de compilação.



<p align="center">
  <img style="border-radius: 5%;" src="../docs/images/rust_imagem.png" alt="Ícone da linguagem de programação Rust">
</p>