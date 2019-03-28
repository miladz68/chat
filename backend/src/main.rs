// use crate::actix::*;
use ::actix::prelude::*;
use actix_web::*;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

/// Define http actor
struct Ws;

struct AppState {
    clients: Arc<RwLock<HashSet<Addr<Ws>>>>,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self, Arc<AppState>>;
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text("Welcome");
        let addr = ctx.address();
        let mut cls = ctx.state().clients.write().unwrap();
        cls.insert(addr);
        println!("total clients {}", cls.len())
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        // println!("stopping {:?}",);
        let addr = ctx.address();
        let mut cls = ctx.state().clients.write().unwrap();
        cls.remove(&addr);
        println!("total clients {}", cls.len());
        Running::Stop
    }
}
#[derive(Message)]
struct Ms {
    text: Arc<String>,
}
/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<Ms> for Ws {
    type Result = ();

    fn handle(&mut self, msg: Ms, ctx: &mut Self::Context) {
        ctx.text(msg.text);
    }
}

/// Handler for ws::Message message
impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => {
                let cls = ctx.state().clients.read().unwrap();
                let arc = Arc::new(text);
                for cl in cls.iter() {
                    cl.do_send(Ms {
                        text: Arc::clone(&arc),
                    });
                }
            }
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}

fn main() {
    let state = Arc::new(AppState {
        // counter: Arc::new(RwLock::new(HashMap::new())),
        clients: Arc::new(RwLock::new(HashSet::new())),
    });
    server::new(move || {
        App::with_state(state.clone())
            .resource("/ws/", |r| r.f(|req| ws::start(req, Ws)))
            .finish()
    })
    .bind("127.0.0.1:8020")
    .expect("Can not bind to port 8020")
    .run();
}
