use leptos::prelude::*;
use saptrans_preview::{Language, Page, Site};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    let generation = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("horloge système invalide")
        .as_millis()
        .to_string();
    println!("Maquette SAPTRANS : http://localhost:3000");
    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                if let Err(error) = serve(stream, &generation) {
                    if !matches!(
                        error.kind(),
                        io::ErrorKind::BrokenPipe | io::ErrorKind::ConnectionReset
                    ) {
                        eprintln!("Requête ignorée : {error}");
                    }
                }
            }
            Err(error) => eprintln!("Connexion ignorée : {error}"),
        }
    }
    Ok(())
}

fn version(generation: &str) -> String {
    let stylesheet = std::fs::metadata("public/style.css")
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_millis())
        .unwrap_or_default();
    format!("{generation}-{stylesheet}")
}

fn serve(mut stream: TcpStream, generation: &str) -> io::Result<()> {
    let mut buffer = [0; 4096];
    let length = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..length]);
    let target = request.split_whitespace().nth(1).unwrap_or("/");
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    if let Some(destination) = Page::legacy_destination(path) {
        let location = if Language::from_query(query) == Language::En {
            format!("{destination}?lang=en")
        } else {
            destination.to_string()
        };
        write!(stream, "HTTP/1.1 308 Permanent Redirect\r\nLocation: {location}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")?;
        return Ok(());
    }
    let language = Language::from_query(query);
    let search_query = query
        .split('&')
        .find_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            (key == "q").then(|| value.replace('+', " ").replace("%20", " "))
        })
        .unwrap_or_default();
    let selected_service = query
        .split('&')
        .find_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            (key == "service" && matches!(value, "transport" | "engins" | "autorelax" | "immobilier"))
                .then(|| value.to_owned())
        })
        .unwrap_or_default();
    let (status, content_type, body) = if path == "/__version" {
        ("200 OK", "text/plain; charset=utf-8", version(generation).into_bytes())
    } else if path == "/style.css" {
        (
            "200 OK",
            "text/css; charset=utf-8",
            std::fs::read("public/style.css")?,
        )
    } else if path == "/logo.svg" {
        (
            "200 OK",
            "image/svg+xml",
            include_bytes!("../public/logo.svg").to_vec(),
        )
    } else if let Some(filename) = path.strip_prefix("/media/legacy/") {
        let media_type = match filename.rsplit_once('.').map(|(_, suffix)| suffix) {
            Some("jpg" | "jpeg") => "image/jpeg",
            Some("png") => "image/png",
            Some("webp") => "image/webp",
            Some("avif") => "image/avif",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            _ => "application/octet-stream",
        };
        if filename.is_empty() || !filename.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '-') || media_type == "application/octet-stream" {
            ("404 Not Found", "text/plain; charset=utf-8", b"Media introuvable".to_vec())
        } else {
            match std::fs::read(format!("public/media/legacy/{filename}")) {
                Ok(bytes) => ("200 OK", media_type, bytes),
                Err(_) => ("404 Not Found", "text/plain; charset=utf-8", b"Media introuvable".to_vec()),
            }
        }
    } else if let Some(font) = match path {
        "/fonts/fira-sans-regular.woff2" => Some(include_bytes!("../public/fonts/fira-sans-regular.woff2").as_slice()),
        "/fonts/fira-sans-medium.woff2" => Some(include_bytes!("../public/fonts/fira-sans-medium.woff2").as_slice()),
        "/fonts/fira-sans-semibold.woff2" => Some(include_bytes!("../public/fonts/fira-sans-semibold.woff2").as_slice()),
        _ => None,
    } {
        ("200 OK", "font/woff2", font.to_vec())
    } else if let Some(page) = Page::from_path(path) {
        let content = view! { <Site page=page language=language query=search_query selected_service=selected_service /> }.to_html();
        let body = format!(
            "<!doctype html><html lang=\"{}\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{}</title><script>let savedTheme=null;try{{savedTheme=localStorage.getItem('saptrans-theme')}}catch{{}}if(savedTheme==='dark'||savedTheme==='light')document.documentElement.setAttribute('data-theme',savedTheme);else if(matchMedia('(prefers-color-scheme: dark)').matches)document.documentElement.setAttribute('data-theme','dark')</script><link rel=\"preconnect\" href=\"https://saptrans.net\"><link rel=\"preconnect\" href=\"https://images.pexels.com\" crossorigin><link rel=\"stylesheet\" href=\"/style.css\"></head><body>{content}<script>{}</script><script>const version=\"{}\";setInterval(async()=>{{try{{const response=await fetch('/__version',{{cache:'no-store'}});if(response.ok&&(await response.text())!==version)location.reload()}}catch{{}}}},1000)</script></body></html>",
            language.code(),
            page.title(language),
            include_str!("../public/theme.js"),
            version(generation)
        );
        ("200 OK", "text/html; charset=utf-8", body.into_bytes())
    } else {
        (
            "404 Not Found",
            "text/plain; charset=utf-8",
            "Page introuvable".as_bytes().to_vec(),
        )
    };
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    stream.write_all(&body)
}
