use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Instant;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").expect("No se pudo iniciar el servidor");
    println!("Servidor corriendo en http://127.0.0.1:3030");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Error al aceptar conexion: {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Ok(_) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer);
        if let Some(i) = parse_request(&request) {
            let start_time = Instant::now();
            let pi = calculate_pi(i);
            let elapsed = start_time.elapsed().as_secs_f64();
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nValor de Pi para el termino {}: {} (Tiempo: {:.6} segundos)\r\n", i, pi, elapsed);
            stream.write_all(response.as_bytes()).unwrap();
        } else {
            let response = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nSolicitud invalida\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn parse_request(request: &str) -> Option<u64> {
    let parts: Vec<&str> = request.lines().next()?.split_whitespace().collect();
    if parts.len() < 2 || !parts[0].eq("GET") {
        return None;
    }
    if let Some(pos) = parts[1].strip_prefix("/pi/") {
        return pos.parse().ok();
    }
    None
}

fn calculate_pi(n: u64) -> f64 {
    let mut pi = 0.0;
    for k in 0..n {
        let term = (-1.0_f64).powi(k as i32) / (2.0 * k as f64 + 1.0);
        pi += term;
    }
    pi * 4.0
}