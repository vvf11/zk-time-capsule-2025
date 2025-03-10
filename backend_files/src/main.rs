use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sp1_sdk::{ProverClient, SP1Stdin};

// Загружаем скомпилированный ELF
pub const ELF: &[u8] = include_bytes!("../capsule-program-elf");

#[derive(Deserialize)]
struct ProofRequest {
    unlock_date: u64,
    message: String,
    photo_data: Vec<u8>,
}

#[derive(Serialize)]
struct ProofResponse {
    proof: Vec<u8>,
}

#[post("/generate_proof")]
async fn generate_proof_endpoint(req: web::Json<ProofRequest>) -> impl Responder {
    let current_date = 20250309;

    if current_date >= req.unlock_date {
        return HttpResponse::BadRequest().body("Дата открытия должна быть в будущем!");
    }

    let mut stdin = SP1Stdin::new();
    stdin.write(&req.unlock_date);
    stdin.write(&req.message);
    stdin.write(&req.photo_data);

    let client = ProverClient::from_env();
    let (pk, _vk) = client.setup(ELF);
    let proof = match client.prove(&pk, &stdin).run() {
        Ok(proof) => proof,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Ошибка генерации: {:?}", e)),
    };

    HttpResponse::Ok().json(ProofResponse { proof: proof.bytes() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(generate_proof_endpoint)
            .service(actix_files::Files::new("/", "../frontend").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}