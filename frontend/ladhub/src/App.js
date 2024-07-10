import React from 'react';
import OffersTable from './components/OffersTable';
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1>Offers</h1>
      </header>
      <main>
        <OffersTable />
      </main>
    </div>
  );
}

export default App;