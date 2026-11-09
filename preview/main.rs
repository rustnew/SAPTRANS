use leptos::prelude::*;
use saptrans_preview::{Page, Site};
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("Maquette SAPTRANS : http://localhost:3000");
    for connection in listener.incoming() {
        let mut stream = connection?;
        let mut buffer = [0; 4096];
        let length = stream.read(&mut buffer)?;
        let request = String::from_utf8_lossy(&buffer[..length]);
        let path = request.split_whitespace().nth(1).unwrap_or("/");
        let (status, content_type, body) = if path == "/style.css" {
            (
                "200 OK",
                "text/css; charset=utf-8",
                include_str!("../public/style.css").to_owned(),
            )
        } else if let Some(page) = Page::from_path(path) {
            let content = view! { <Site page=page /> }.to_html();
            let body = format!(
                "<!doctype html><html lang=\"fr\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{}</title><link rel=\"stylesheet\" href=\"/style.css\"></head><body>{content}</body></html>",
                page.title()
            );
            ("200 OK", "text/html; charset=utf-8", body)
        } else {
            (
                "404 Not Found",
                "text/plain; charset=utf-8",
                "Page introuvable".to_owned(),
            )
        };
        write!(
            stream,
            "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        )?;
        stream.write_all(body.as_bytes())?;
    }
    Ok(())
}
