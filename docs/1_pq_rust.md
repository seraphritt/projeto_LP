# Introdução do Projeto

Um projeto de implementação de uma criptomoeda é um exemplo de como as premissas de Rust alinham-se com os requisitos e desafios técnicos envolvidos. Desse modo, exploraramos a linguagem Rust e como ela se porta para este tipo de aplicação devido a exigência de segurança e performance.
## Premissas da Linguagem e sua Aplicabilidade

### Segurança de Memória
* Relevância: Em um projeto de criptomoeda, a segurança é absolutamente crítica. Falhas de segurança podem resultar em perdas financeiras significativas para os usuários e descredibilidade da criptomoeda.
* Implementação: Rust garante segurança de memória através de seu sistema de propriedade (ownership) e empréstimo (borrowing). Isso elimina preocupações comuns como vulnerabilidades de *buffer overflow*, vazamento de memória e race condições de corrida, proporcionando uma base sólida para a implementação segura de uma criptomoeda.

### Performance
* Relevância: Criptomoedas demandam alta performance, tanto para o processamento de transações quanto para a mineração de blocos.
* Implementação: Rust oferece performance comparável a linguagens de baixo nível como C e C++, permitindo controle direto sobre a alocação de memória e otimizações pelo compilador.

### Concorrência Segura
* Relevância: Transações em uma criptomoeda envolvem múltiplos agentes operando simultaneamente. Portanto, é importante garantir que operações concorrentes sejam seguras.
        
* Implementação: Rust oferece primitivas seguras para concorrência, como tipos Send e Sync, que garantem a segurança de operações concorrentes sem comprometer a performance. Isso assegura que a blockchain da criptomoeda possa lidar com transações concorrentes de forma eficiente e sem riscos de inconsistências.

### Abstração sem Overhead
* Relevância: Desenvolver uma criptomoeda requer abstrações de alto nível para lidar com complexidades como transações, contratos inteligentes e validação de blocos.
* Implementação: Rust permite abstrações poderosas e eficientes através de mecanismos como traits, enums e pattern matching. Isso facilita a implementação de algoritmos criptográficos complexos e estruturas de dados customizadas, sem introduzir overheads desnecessários que poderiam impactar negativamente a performance da rede.

## Usuário Característico e Domínio de Aplicação

* Usuário Característico: Desenvolvedores e engenheiros com experiência em linguagens como C, C++ e que desejam desenvolver uma aplicação que exige segurança, performance e escabilidade.

* Domínio de Aplicação: Sistemas Operacionais (Redox OS), compiladores, sistemas embarcados, WebAssembly, Ferramnetas que exigem segurança e controle de memória.
