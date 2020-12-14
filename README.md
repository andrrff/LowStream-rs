<h1 align="center"> LowStream </h1>
<p align="justify"> Aplicacao de streaming performatica e de baixo custo de hardware </p>
<img alt="Website" src="https://img.shields.io/website?down_message=offline&up_message=online&url=https%3A%2F%2Flowstream.tk"> 
<blockquote>By Alexandroviski,André, $enick & Pablo</blockquote> <a href="https://lowstream.tk/">Entrar no site</a>

<h3>Objetivos</h3>

  - [x] Implentação do wasm no projeto; **Concluido :heavy_check_mark:**
  - [x] Implentação do Bulma.io; **Concluido :heavy_check_mark:**
  - [ ] Suporte ao ybc; **Em desenvolvimento :warning:**
  - [ ] Aplicar dinamismo nas funcionalidades; **Em desenvolvimento :warning:**
  - [ ] Router;
  - [ ] Framework Diesel;
  - [ ] Upload videos;
    - [ ] Like e Dislike;
    - [ ] Comentários nos videos;
    - [ ] ~~Download video~~ 
  - [ ] Database para forms de logins;
  - [ ] ~~Adcionar acesso de moderador~~ 
  - [ ] transação descentralizada;
  
  
  <h3>Build</h3>
  
  <h4>Trunk</h4>
  
  Precisa ter instaldo o compilador *Rust*
  Depois de instalado, agora instale os seguintes ferramentas do rustup.
  
  ```
  rustup target add wasm32-unknown-unknown
  ```
  
  ```
  cargo install trunk && cargo install trunk wasm-bindgen-cli
  ```
  Depois de tudo instalado nos conformes, execute o comando:
  *Se quiser pode dá só o comando ```trunk serve```, irá funcionar da mesma forma*
  
  ```
  trunk build && trunk serve
  ```

  <img alt="Noooo a imagem n presta T_T" src="https://i.pinimg.com/564x/05/a8/0e/05a80e4c78c3bd767650229f0407e162.jpg"> 