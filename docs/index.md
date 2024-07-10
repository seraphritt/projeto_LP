# Wiki do Projeto

## Integrantes

- Bruno Berto de Oliveira Ribeiro (200061089)
- Isaque Augusto da Silva Santos (190089245)
- Moises de Araújo Altounian (200069306)
- Pedro Cesar Ribeiro Passos (180139312)
- Tiago Cabral de Faria (160146712)

<center>
<table style="margin-left: auto; margin-right: auto;">
    <tr>
        <td align="center">
            <a href="https://github.com/AngryLeaderBB">
                <img style="border-radius: 50%;" src="https://avatars.githubusercontent.com/u/73657838?v=4" width="150px;"/>
                <h5 class="text-center">Bruno Berto de Oliveira Ribeiro</h5>
            </a>
        </td>
        <td align="center">
            <a href="https://github.com/seraphritt">
                <img style="border-radius: 50%;" src="https://avatars.githubusercontent.com/u/84244850?v=4" width="150px;"/>
                <h5 class="text-center">Isaque Augusto da Silva Santos</h5>
            </a>
        </td>
        <td align="center">
            <a href="https://github.com/ogmoises">
                <img style="border-radius: 50%;" src="https://avatars.githubusercontent.com/u/144768841?v=4" width="150px;"/>
                <h5 class="text-center">Moises de Araújo Altounian</h5>
            </a>
        </td>
        </td>
        <td align="center">
            <a href="https://github.com/pedrocrp">
                <img style="border-radius: 50%;" src="https://avatars.githubusercontent.com/u/83802848?v=4" width="150px;"/>
                <h5 class="text-center">Pedro Cesar Ribeiro Passos</h5>
            </a>
        </td>
        <td align="center">
            <a href="https://github.com/tiag0cabral">
                <img style="border-radius: 50%;" src="https://avatars.githubusercontent.com/u/19624182?v=4" width="150px;"/>
                <h5 class="text-center">Tiago Cabral de Faria</h5>
            </a>
        </td>
    </tr>
</table>
</center>

## Sobre o Projeto

Este projeto envolve a criação e implementação de uma criptomoeda utilizando *blockchain* em Rust. A *blockchain* serve como o histórico para todas as transações efetuadas pelos usuários da criptomoeda, garantindo segurança e imutabilidade dos dados transacionais.

A linguagem foi selecionada devido às suas características em termos de segurança, eficiência de concorrência e alto desempenho.

## Blockchain

A blockchain é um registro público de todas as transações realizadas em uma rede específica, estruturadas em blocos encadeados que garantem transparência e segurança. Cada um dos blocos contém um conjunto de transações, sendo ligados - de maneira criptografada - ao bloco anterior, formando uma cadeia contínua. Com isso, qualquer tentativa de modificar informações anteriores se torna um processo extremamente complicado, visto que, para isso, é necessária a alteração de todos os blocos que vem em sequência.

### Entendendo uma Blockchain: 

* Criptografia De Chave Pública: Técnica que utiliza um par de chaves criptográficas - uma pública e uma privada.
    * Chave pública: utilizada para criptografar dados, tornando-os acessíveis apenas para quem possui a chave privada correspondente
    * Chave privada: utilizada para decifrar os dados criptografado, garantindo que apenas o destinatário legítimo possa ler as mensagens.


* Algoritmo De Consenso: utilizado em uma blockchain para garantir que todos os participantes da rede concordem sobre o estado atual do sistema de maneira descentralizada. Diferentes algoritmos de consenso desempenham papéis variados na validação das transações:
    * Proof of Work (PoW): em que os participantes resolvem problemas computacionais complexos para validar transações
    * Proof of Stake (PoS): onde a validade das transações é determinada pela quantidade de moeda mantida pelos participantes.

## Criptomoeda

Forma de dinheiro digital que, utilizando criptografia, garante a segurança de transações e o controle da emissão de moedas. Diferentemente das moedas tradicionais, as criptomoedas operam em um sistema descentralizado, o que permite a eliminação de intermediários financeiros. Cada transação é realizada em um bloco da blockchain e essas moedas podem ser utilizadas para cumprir uma variedade de propósitos, como compras de produtos online, investimentos, compras de ativos e transferência de valor monetário entre indivíduos.

