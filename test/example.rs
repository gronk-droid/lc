use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

struct WebSocketSession {
    heartbeat: Instant,
}

impl ws::MessageHandler for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: ws::Message, ctx: &mut ws::WebsocketContext<Self>) {
        match msg {
            ws::Message::Ping(_) => {
                self.heartbeat = Instant::now();
                ctx.pong(b"");
            }
            ws::Message::Text(text) => {
                ctx.text(format!("Echo: {}", text));
            }
            _ => (),
        }
    }
}

async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> impl Responder {
    ws::start(WebSocketSession { heartbeat: Instant::now() }, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ws", web::get().to(websocket_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
