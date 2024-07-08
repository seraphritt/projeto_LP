import React, { useState } from 'react';
import '../styles/Login.css';

const Login = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  const handleSubmit = (event) => {
    event.preventDefault();

    console.log('Username:', username);
    console.log('Password:', password);
  };

  return (
    <div className="login-container">
      <div className="login-form-container">
        <form onSubmit={handleSubmit} className="login-form">
          <div className="site-header">
            <h1>Nome do App</h1>
          </div>
          <h2>Login</h2>
          <div className="form-group">
            <label htmlFor="username">Usuário</label>
            <input
              type="text"
              id="username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label htmlFor="password">Senha</label>
            <input
              type="password"
              id="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
            />
          </div>
          <button type="submit">Entrar</button>
        </form>
        <div className="register-link">
        <p><a href="/register">Cadastre-se aqui</a></p>
      </div>
      </div> 
    </div>
  );
};

export default Login;