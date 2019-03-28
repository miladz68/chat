use format::{Binary, Text};
use std::time::Duration;
use yew::*;

struct Model {
    ws_task: services::websocket::WebSocketTask,
    // wss: services::WebSocketService,
    console: services::ConsoleService,
    // heartbeat: services::IntervalService,
    _handle: services::interval::IntervalTask,
    value: String,
    messages: Vec<String>,
}

enum Msg {
    SendIt,
    Ignore,
    HeartBeatEvent,
    Incoming(String),
    Update(String),
}

struct SocketMsg {
    text: String,
}

impl From<Text> for SocketMsg {
    fn from(st: Text) -> Self {
        let mut sm = SocketMsg {
            text: "".to_string(),
        };
        match st {
            Ok(s) => sm.text = s,
            Err(_) => sm.text = "".to_string(),
        };
        sm
    }
}
impl From<Binary> for SocketMsg {
    fn from(_st: Binary) -> Self {
        SocketMsg {
            text: "binary".to_string(),
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut soc = services::WebSocketService::new();
        let mut interval_service = services::IntervalService::new();
        let cons = services::ConsoleService::new();
        let callback = link.send_back(|txt: SocketMsg| Msg::Incoming(txt.text));
        let notification = |_ev| println!("{}", "notification recieved");
        let interval_callback = link.send_back(|_| Msg::HeartBeatEvent);
        // let interval_callback = link.send_back(|_| Msg::Incoming("event".to_string()));
        let _inters = interval_service.spawn(Duration::from_secs(1), interval_callback.clone());

        let tsk = soc.connect(
            "ws://localhost:8020/ws/",
            callback.into(),
            notification.into(),
        );

        // tsk.send(Ok("test".to_string()));
        Model {
            ws_task: tsk,
            console: cons,
            // wss: soc,
            // heartbeat: interval_service,
            _handle: _inters,
            value: "".to_string(),
            messages: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => false,
            Msg::SendIt => {
                // Update your model on events
                let old = self.value.clone();
                self.value = "".to_string();
                self.ws_task.send(Ok(old));
                true
            }
            Msg::Incoming(input) => {
                self.console.log(&input);
                self.messages.push(input);
                // self.console.count();
                true
            }
            Msg::HeartBeatEvent => {
                // self.console.log("heartbeat");
                // self.ws_task.send(Ok("sending".to_string()));
                true
            }
            Msg::Update(val) => {
                self.value = val;
                false
            }
        }
    }
}
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let item_chat = |item| {
            html! {
                    <li> {item} </li>
            }
        };
        html! {
            // Render your model here
            <div>
                <ul class="uk-list",>
                    {for  self.messages.iter().map(item_chat)  }
                </ul>
                <input  class="uk-input",
                        placeholder="new chat message",
                        oninput=|e| Msg::Update(e.value),
                        value=&self.value,
                        onkeypress=|e| {
                            if e.key() == "Enter" {Msg::SendIt} else {Msg::Ignore}
                        },
                ></input>
                <button class="uk-button uk-button-default",
                        onclick=|_| Msg::SendIt,
                        >{ "send" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>()
}
