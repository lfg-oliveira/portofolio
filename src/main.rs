use std::io::prelude::*;
use std::io;
use actix_files::Files;
use actix_web::{HttpServer, App, web, Responder, HttpResponse, HttpRequest, get };
use webbrowser::open;


fn hello_world() {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    std::process::Command::new("clear").status().unwrap();
    #[cfg(any(target_os = "windows"))]
    std::process::Command::new("cls").status().unwrap();
    println!(r#"O lugar em que todos começamos foi aqui.
Alguns iniciaram com um print("Hello World").
Outros iniciaram com console.log("Olá mundo!").
Ou até mesmo <h1> Olá, mundo! <h1>.
Mas todos começaram no mesmo lugar.
Sem conhecimento algum."#);
}

async fn portshell(_strbuf: &mut str) {
    println!("Pressione a tecla enter para continuar.");
    io::stdin().read_line(&mut String::from(_strbuf)).expect("Não foi possível ler a entrada padrão.");
    hello_world();
    open("http://127.0.0.1:8000").expect("Não foi possível abrir o browser!");
}



#[actix_web::main]
async fn main()  -> io::Result<()> {
    let mut _strbuf = String::new();
    portshell(&mut _strbuf).await;
    HttpServer::new(|| {
        let app = App::new();
        app
        .service(Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}