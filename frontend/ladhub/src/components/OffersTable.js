import React from 'react';
import './OffersTable.css';

const offers = [
  {
    robo: 'MinorCinch986',
    tipo: 'Vendendo',
    quantidade: '500-2,000',
    moeda: 'BRL',
    metodoPagamento: ['Depósito imediato BB Banc'],
    expiry: '17h',
    timer: '1h',
    sats: '534 K',
    preco: '374,410 BRL/BTC',
    bond: '2%',
    premio: '5%',
    orderId: '#117091',
  },
  {
    robo: 'DozingHistory233',
    tipo: 'Vendendo',
    quantidade: '15-190',
    moeda: 'EUR',
    metodoPagamento: ['Revolut', 'Transferwise'],
    expiry: '19h',
    timer: '8h',
    sats: '263 K',
    preco: '72,253 EUR/BTC',
    bond: '15%',
    premio: '10%',
    orderId: '#117106',
  },
  // Adicione mais dados conforme necessário
];

const OffersTable = () => {
  return (
    <div className="offers-table">
      <table>
        <thead>
          <tr>
            <th>Robô</th>
            <th>É</th>
            <th>Host</th>
            <th>Quantidade</th>
            <th>Moeda</th>
            <th>Método de Pagamento</th>
            <th>Expiry</th>
            <th>Timer</th>
            <th>Sats Now</th>
            <th>Preço</th>
            <th>Bond</th>
            <th>Prêmio</th>
            <th>Order ID</th>
          </tr>
        </thead>
        <tbody>
          {offers.map((offer, index) => (
            <tr key={index}>
              <td>{offer.robo}</td>
              <td>{offer.tipo}</td>
              <td>{/* Imagem do host */}</td>
              <td>{offer.quantidade}</td>
              <td>{offer.moeda}</td>
              <td>
                {offer.metodoPagamento.map((metodo, idx) => (
                  <span key={idx}>{metodo}</span>
                ))}
              </td>
              <td>{offer.expiry}</td>
              <td>{offer.timer}</td>
              <td>{offer.sats}</td>
              <td>{offer.preco}</td>
              <td>{offer.bond}</td>
              <td>{offer.premio}</td>
              <td>{offer.orderId}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default OffersTable;
