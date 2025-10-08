use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct Message {
    text: String,
}

#[derive(Serialize)]
struct Reply {
    ok: bool,
    echo: String, // devolvemos la "respuesta del sistema" aquí
}

#[tokio::main]
async fn main() {
    // CORS abierto para que tu index.html pueda llamar al backend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/messages", post(handle_message))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Servidor en http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_message(Json(msg): Json<Message>) -> Json<Reply> {
    let reply_text = match_book_reply(&msg.text);
    Json(Reply { ok: true, echo: reply_text })
}

// --------- LÓGICA DE COINCIDENCIA PARA TUS 3 LIBROS ---------

// Títulos
const L1: &str = "La chica de corazón oscuro";
const L2: &str = "El jugador mató a Noelle";
const L3: &str = "La valiente guerrera de instrumentos musicales";

fn match_book_reply(user_text: &str) -> String {
    let t = normalize(user_text);

    // —— ATAJOS (coincidencias fuertes) ——
    // L1: si menciona explícitamente "corazon oscuro" (con o sin tilde), es L1 directo.
    if t.contains("corazon oscuro") || (t.contains("corazon") && t.contains("oscuro")) {
        return format!("Ese libro que estás buscando se llama: \"{}\".", L1);
    }

    // L2: si hay “jugador / noelle / videojuego / asesinar-matar”, es L2 directo.
    if ["jugador", "noelle", "videojuego", "asesin", "matar", "mato", "mató"]
        .iter()
        .any(|kw| t.contains(kw))
    {
        return format!("Ese libro que estás buscando se llama: \"{}\".", L2);
    }

    // L3: si combina “guerrera” con “instrumento(s)” o “musical(es)”, elegir L3 directo.
    if t.contains("guerrera") && (t.contains("instrumento") || t.contains("instrumentos")
        || t.contains("musical") || t.contains("musicales") || t.contains("musica"))
    {
        return format!("Ese libro que estás buscando se llama: \"{}\".", L3);
    }

    // —— PUNTUACIÓN SUAVE (fallback) ——
    let mut score1 = 0;
    let mut score2 = 0;
    let mut score3 = 0;

    // L1 señales suaves
    for kw in ["chica", "oscuro"] {
        if t.contains(kw) { score1 += 1; }
    }
    // L2 señales suaves ya cubiertas arriba, pero sumamos por si no activó atajo
    for kw in ["jugador", "noelle", "videojuego", "asesin", "matar", "mato", "mató"] {
        if t.contains(kw) { score2 += 2; }
    }
    // L3 suaves
    for kw in ["guerrera", "instrumento", "instrumentos", "musical", "musicales", "musica"] {
        if t.contains(kw) { score3 += 1; }
    }

    // Ambigüedad “protagonista chica” sin más señales → sugerir L1 y L3
    let vague_female = t.contains("chica") || t.contains("protagonista");
    let has_l3_hint = t.contains("guerrera") || t.contains("instrumento") || t.contains("musical");
    if vague_female && !has_l3_hint && score2 == 0 && score1 > 0 {
        return format!(
            "Hace falta más descripción para reconocer de cuál libro me estás hablando, \
tengo dos libros que se ajustan a tu descripción, en el que la chica es la protagonista: \
\"{}\" y \"{}\". ¿Alguno de estos dos libros es el que estás buscando?",
            L1, L3
        );
    }

    // Selección por puntaje
    let max_score = score1.max(score2).max(score3);
    if max_score > 0 {
        let mut candidates: Vec<&str> = Vec::new();
        if score1 == max_score { candidates.push(L1); }
        if score2 == max_score { candidates.push(L2); }
        if score3 == max_score { candidates.push(L3); }

        if candidates.len() == 1 {
            return format!("Ese libro que estás buscando se llama: \"{}\".", candidates[0]);
        } else {
            let listado = candidates.into_iter()
                .map(|s| format!("\"{}\"", s))
                .collect::<Vec<_>>()
                .join(" y ");
            return format!("Necesito un poco más de detalle. Podría ser {}. ¿Cuál te suena más?", listado);
        }
    }

    "Lo siento, no se hallan libros con las descripciones otorgadas.".to_string()
}


// Normaliza: pasa a minúsculas y quita tildes comunes para comparar
fn normalize(s: &str) -> String {
    let lower = s.to_lowercase();
    lower
        .replace('á', "a")
        .replace('é', "e")
        .replace('í', "i")
        .replace('ó', "o")
        .replace('ú', "u")
        .replace('ñ', "n")
}
