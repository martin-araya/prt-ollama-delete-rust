use actix_web::{delete, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use serde_json::json;
use futures::StreamExt;

#[derive(Deserialize)]
struct DeleteRequest {
    name: String,
}

#[derive(Deserialize)]
struct PullRequest {
    name: String,
    insecure: Option<bool>,
    stream: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct PullResponse {
    message: String,
}

// Endpoint para eliminar un modelo en Ollama
#[delete("/api/delete")]
async fn delete_model(req: web::Json<DeleteRequest>) -> impl Responder {
    let client = Client::new();
    let url = "http://localhost:11434/api/delete";

    // Crear el cuerpo de la solicitud JSON
    let body = json!({
        "name": req.name,
    });

    // Enviar solicitud DELETE a Ollama
    match client.delete(url).json(&body).send().await {
        Ok(response) => {
            if response.status().is_success() {
                HttpResponse::Ok().body("Model deleted successfully.")
            } else if response.status().as_u16() == 404 {
                HttpResponse::NotFound().body("Model not found.")
            } else {
                HttpResponse::InternalServerError().body("Failed to delete the model.")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to connect to Ollama"),
    }
}

// Endpoint para descargar un modelo desde Ollama
#[post("/api/pull")]
async fn pull_model(req: web::Json<PullRequest>) -> impl Responder {
    let client = Client::new();
    let url = "http://localhost:11434/api/pull";

    let body = json!({
        "name": req.name,
        "insecure": req.insecure.unwrap_or(false),
        "stream": req.stream.unwrap_or(true),
    });

    match client.post(url).json(&body).send().await {
        Ok(response) => {
            if req.stream.unwrap_or(true) {
                let mut stream = response.bytes_stream();

                let response_stream = async_stream::stream! {
                    while let Some(chunk) = stream.next().await {
                        match chunk {
                            Ok(bytes) => {
                                yield Ok::<_, actix_web::Error>(web::Bytes::from(bytes));
                            }
                            Err(_) => {
                                yield Err(actix_web::error::ErrorInternalServerError("Error reading stream"));
                            }
                        }
                    }
                };

                return HttpResponse::Ok()
                    .content_type("application/octet-stream")
                    .streaming(response_stream);
            }

            match response.json::<PullResponse>().await {
                Ok(json) => HttpResponse::Ok().json(json),
                Err(_) => HttpResponse::InternalServerError().body("Error parsing Ollama response"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to connect to Ollama"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(delete_model) // Registra el endpoint DELETE
            .service(pull_model)    // Registra el endpoint POST
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await
}
