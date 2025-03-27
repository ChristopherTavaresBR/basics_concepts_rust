# 🚀 Rust BR: **Sem GC, Com Ginga!**  


# 🎯 Essa bagaça será em PT-BR, tem muito conteúdo gringo EN 🎯


## Antes de começar (Se não tiver instalado)

Fuja do Windows diretamente, tente usar o WSL

### No terminal (Linux/Mac/WSL):  
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  

# Windows (se não houver opções): Baixa o .exe no [site oficial](https://www.rust-lang.org/tools/install)  
```

### CRIANDO O PROJETO COM CARGO (O GERENCIADOR GOD DO RUST)

```bash
# Cria um projeto novo (substitua "nome_do_projeto" pelo nome do teu projeto):  
cargo new nome_do_projeto --bin  # --bin = projeto executável (não biblioteca)  

# Entra na pasta:  
cd nome_do_projeto  
```

✨ MAGIA ACONTECEU, CHORA BB:
O Cargo já criou essa estrutura básica:

```bash
/rust_br  
├── Cargo.toml  # 🏆 ARQUIVO DE CONFIGURAÇÃO (O "coração" do projeto)  
└── src  
    └── main.rs  # 📜 Código inicial (um simples "Hello, world!")  
```

### EDITANDO O Cargo.toml (DEIXANDO BOLADÃO)

```bash
[package]  
name = "nome_do_projeto"  
version = "0.1.0"  
edition = "2024"  # 🚀 Usa a versão mais moderna do Rust!  
authors = ["SeuNome <seu.email@example.com>"]  
description = "Projeto didático de Rust com hype BR! 🦀"  
license = "MIT"  # Ou "Apache-2.0" se preferir  

# 🔥 Dependências (vamos adicionar um crate maneiro pra começar)  
[dependencies]  
ferris-says = "0.3"  # 🦀 Um crate que faz o mascote do Rust "falar"!  
```

Hora da verdade
```bash
# Baixa e compila as dependências (primeira vez pode demorar):  
cargo build  

# Roda o projeto:  
cargo run  
```

###Expectativa e realidade

```bash
 _____________________________
/ E aí, Brasil! Hora do Rust! \
\ 🦀                          /
 -----------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
```

## 🎯 POR QUE ESSE SETUP?

Cargo.toml organizado: Metadados claros + dependência externa (pra mostrar o ecossistema).

Exemplo visual: Ferris (mascote do Rust) "falando".


## Próximo commit do curso! 

### Aqui a gente aprenderá:

Vai testando ai que é suave! Lembre-se de iniciar o projeto, nesse caso não há dependencias.

✅ **Ownership**: Rust é o dono da bola e você não pode passar ela duas vezes!  
✅ **Borrowing**: Pegar emprestado é suave, mas tem que devolver! 

Só cola no main.rs e siga os comentários, qualquer coisa grita na issues

```rust
// 📌 PARTE 1: DONO DA BOLA (OWNERSHIP)  
fn main() {  
    let mut mensagem = String::from("Rust"); // 🏈 Dono da String!  
    rouba_bola(&mensagem); // EMPRESTA (borrow) pra função ler  
    println!("{} ainda é dono da mensagem!", mensagem); // ✅ Funciona!  

    // ⚠️ DESCOMENTE ISSO E VEJA O COMPILADOR CHORAR:  
    // rouba_bola_de_verdade(mensagem); // TRANSFERE ownership!  
    // println!("{}", mensagem); // 💥 Compilador: "Não tem mais, parça!"  
}  

fn rouba_bola(texto: &String) { // 📍 Recebe EM PRÉSTAMO (&)  
    println!("Só li: {}", texto);  
}  

fn rouba_bola_de_verdade(texto: String) { // 🎯 Toma posse!  
    println!("Agora é MEU: {}", texto);  
}  
```


## 🎯 DESAFIO BR:

1. Rode `cargo run` e veja o código funcionar.  
2. **Descomente as linhas comentadas** e tente rodar de novo.  
3. Posta o erro que o compilador gritou! (Exemplo: "Xii, parça, faltou o `mut` aí!").  

```bash
$ cargo run   # 🦀 O compilador é seu amigo (mas xinga muito)  
```

Aqui você aprendeu como funciona um projeto em Rust e se deliciou com 2 conceitos principais: Ownerships e Borrowings. Se ta achando fácil, se prepare que logo a chapa esquenta lá onde o filho chora e a mãe não vê.



```bash
 ______________________
/ Busque conhecimento! \
\                      /
 ----------------------
           \ 
            _~^~^~_
           /  o o  \ 
           '       '
            ˜˜~~~˜˜
 ```
