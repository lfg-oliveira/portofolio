use std::io;
use actix_files::Files;
use actix_web::{HttpServer, App, web, Responder, HttpResponse, HttpRequest };
use webbrowser::open;
const ADDRESS: &str = "127.0.0.1:8000";
fn clear() {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    std::process::Command::new("clear").status().unwrap();
    #[cfg(any(target_os = "windows"))]
    std::process::Command::new("cls").status().unwrap();
}

async fn hello_world() {
    clear();
    println!(r#"O lugar em que todos começamos foi aqui.
Alguns iniciaram com um print("Hello World").
Outros iniciaram com console.log("Olá mundo!").
Ou até mesmo <h1> Olá, mundo! <h1>.
Mas todos começaram no mesmo lugar.
Sem conhecimento algum."#);
    open("http://127.0.0.1:8000").expect("Não foi possível abrir o browser!");
}

async fn portshell() {
    println!("Pressione a tecla enter para continuar.");
    io::stdin().read_line(&mut String::new()).expect("Não foi possível ler a entrada padrão.");
    hello_world().await;
}



#[actix_web::main]
async fn main()  -> io::Result<()> {
    portshell().await;
    HttpServer::new(|| {
        let app = App::new();
        app
        .service(Files::new("/", "static/").index_file("index.html"))
    })
    .bind(ADDRESS)?
    .run()
    .await
}