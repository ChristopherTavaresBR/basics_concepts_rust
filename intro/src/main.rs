use ferris_says::say;  
use std::io::{stdout, BufWriter};  

fn main() {  
    let mensagem = "E aí, Brasil! Hora do Rust! 🦀";  
    let width = mensagem.chars().count();  

    let mut writer = BufWriter::new(stdout());  
    say(mensagem, width, &mut writer).unwrap();  

    // ⚠️ DESAFIO: Modifique a mensagem e veja o Ferris cuspindo ela!  
}  