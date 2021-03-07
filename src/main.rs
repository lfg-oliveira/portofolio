use std::env;
use std::io::prelude::*;
use std::io;
use actix_files::Files;
use actix_web::{HttpServer, App, web, Responder, HttpResponse, get };
use webbrowser::open;
use std::fs;

fn portshell(argc:env::Args, strbuf: String) {
    println!("Cheque seu browser!");
    open("http://127.0.0.1:8000").expect("Não foi possível abrir o browser!");
    
}



#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    let argc = env::args().count();
    let _argv = env::args();
    let mut buf = String::new();
    match argc {
        1 => {
            portshell(_argv,buf);
        }
        _ => {
            println!("Esse shell demonstrativo não aceita argumentos. Prosseguindo normalmente.
            Pressione qualquer teclas para continuar.");
            io::stdin().read_line(&mut buf).unwrap();
            portshell(_argv,buf);
        }
    }
    
    HttpServer::new(|| {
        App::new()
        .service(Files::new("/", "static/").index_file("index.html").prefer_utf8(true))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}