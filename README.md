# prt-ollama-delete-rust
Explicación de Cada Parte
Estructura DeleteRequest
rust
Copy code
#[derive(Deserialize)]
struct DeleteRequest {
name: String,
}
Define el cuerpo de la solicitud DELETE, que contiene solo el name del modelo a eliminar.
Endpoint DELETE /api/delete

rust
Copy code
#[delete("/api/delete")]
async fn delete_model(req: web::Json<DeleteRequest>) -> impl Responder {
Define el endpoint DELETE /api/delete.
req: web::Json<DeleteRequest>: Deserializa la solicitud JSON en DeleteRequest para acceder al name.
Enviar la Solicitud DELETE a Ollama

rust
Copy code
let client = Client::new();
let url = "http://localhost:11434/api/delete";

let body = json!({
"name": req.name,
});
Crea un cliente HTTP reqwest y define la URL para enviar la solicitud DELETE.
Configura el cuerpo JSON con el name del modelo.
Manejo de Respuestas de Ollama

rust
Copy code
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
Envía la solicitud DELETE a Ollama y verifica el código de estado de la respuesta:
200 OK: Devuelve "Model deleted successfully."
404 Not Found: Devuelve "Model not found."
Otros códigos de error: Devuelve "Failed to delete the model."
Error en la conexión: Devuelve "Failed to connect to Ollama".
Endpoint POST /api/pull

El endpoint POST /api/pull para descargar un modelo está implementado como antes, con soporte para streaming en tiempo real.
Prueba del Endpoint DELETE /api/delete
Para probar el nuevo endpoint, ejecuta el servidor y realiza una solicitud DELETE con curl:

bash
Copy code
curl -X DELETE http://localhost:8080/api/delete -d '{"name": "llama3:13b"}' -H "Content-Type: application/json"
Respuesta Esperada
200 OK: Si el modelo se elimina correctamente.
404 Not Found: Si el modelo no existe.
500 Internal Server Error: Si ocurre un error en la conexión o en el procesamiento de la solicitud.
Este código permite eliminar un modelo en Ollama y manejar cualquier error en el proceso.