# Projeto: HTTP Server em Rust

## Descrição Geral
Este projeto consiste em um servidor HTTP desenvolvido em Rust, projetado para oferecer alta performance e flexibilidade, aproveitando conceitos como programação assíncrona e árvores radix. O objetivo principal do projeto é criar um servidor que permite rotas dinâmicas e eficientes, tornando-o uma base sólida para APIs e aplicações web.

## Funcionalidades Principais

1. **Servidor HTTP Personalizável**:
    - Possibilidade de configurar o número de threads, endereço de escuta e porta.
    - Suporte a múltiplos métodos HTTP, como `GET`, com mapeamento de requisições.

2. **Gerenciamento Dinâmico de Rotas**:
    - Implementação de uma *Radix Tree* para gerenciar e casar rotas de forma eficiente.
    - Suporte a rotas estáticas e dinâmicas com parâmetros de URL, como `/user/:id` ou `/user/:id/details/:name`.

3. **Manipuladores de Requisição**:
    - Sistema de "handlers", permitindo que funções sejam definidas para lidar com respostas personalizadas às requisições recebidas.
    - Capacidade de responder com texto puro e status HTTP apropriado.

4. **Tratamento de Erros**:
    - Erros específicos implementados usando a crate `thiserror` para fácil depuração e manuseio, como `RouterError` para rotas não encontradas ou erros ao casar handlers.

5. **Execução Multithread**:
    - Implementação de um pool de threads que possibilita o processamento eficiente de múltiplas requisições simultaneamente.

6. **Router Dinâmico com Suporte a Parâmetros**:
    - As rotas dinâmicas permitem capturar parâmetros diretamente da URL e utilizá-los dentro da lógica do servidor.

## Estrutura do Projeto

O projeto é composto por diferentes módulos que separam responsabilidades de forma clara para facilitar a manutenção e extensibilidade:

- **`server`**: Componente principal que inicializa e gerencia o servidor HTTP.
- **`router` e `radix_tree`**: Módulos responsáveis pelo mapeamento e casamento de rotas usando a árvore radix.
- **`request`**: Representa as requisições HTTP, processando informações como parâmetros e headers.
- **`response`**: Tratamento das respostas HTTP, incluindo status e conteúdo de retorno.
- **`thread_pool`**: Implementação do pool de threads para executar tarefas em paralelo, melhorando a performance do servidor.

## Exemplo de Uso

Abaixo estão alguns exemplos de rotas configuradas no servidor:

1. **Rota com Parâmetro Simples**:
    - URL: `/user/:id`
    - Resultado esperado: Retorna o ID do usuário enviado como parte da rota.

   ```rust
   server.get("/user/:id", |_req| {
       match _req.params.get("id") {
           Some(id) => Response::text(id),
           None => Response::text("Missing user id").with_status(400),
       }
   });
   ```

2. **Rota com Múltiplos Parâmetros**:
    - URL: `/user/:id/details/:name`
    - Resultado esperado: Combina o ID e o nome recebidos, retornando ambos no corpo da resposta.

   ```rust
   server.get("/user/:id/details/:name", |_req| {
       let name = _req.params.get("name").unwrap();
       let id = _req.params.get("id").unwrap();
       Response::text(&format!("{} {}", name, id))
   });
   ```

3. **Rota Estática**:
    - URL: `/about`
    - Resultado esperado: Retorna uma mensagem estática.

   ```rust
   server.get("/about", |_req| {
       Response::text("Information about the project")
   });
   ```

## Tecnologias Utilizadas

- **Rust**: Linguagem de programação principal do projeto devido à sua performance e segurança.
- **Crates**:
    - `thiserror`: Para tratamento de erros elegantes e padronizados.
    - `tokio`: Para funcionamento assíncrono e gerenciamento de threads.
    - `regex`: Auxilia na manipulação de strings e validação de padrões (se necessário).
    - `anyhow`: Trata erros de forma genérica e simplificada.

## Conclusão

O projeto é uma implementação robusta e eficiente de um servidor HTTP em Rust, com atenção à modularidade e performance. A utilização de árvores radix para gerenciamento de rotas e a implementação de um pool de threads destacam-se como características-chave que tornam o servidor rápido e escalável. Além disso, a facilidade para adicionar novas rotas e parâmetros dinâmicos faz deste projeto uma excelente escolha para pequenas APIs e projetos maiores.