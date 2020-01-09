use teloxide::Bot;
use teloxide::dispatching::private::Dispatcher;
use teloxide::dispatching::SessionState;
use teloxide::dispatching::update_listeners::polling_default;
use teloxide::types::{Update,UpdateKind,Message};
use futures::stream::{StreamExt};
use teloxide::requests::Request;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = &Bot::new("867812646:AAEFJWGeakLiA3QO9G2m_8DQ8T8HhZqG47s");
    let mut updater = Box::pin(polling_default(bot));
    let handler =  |s, upd: Update| async move {
        match upd.kind {
            UpdateKind::Message(m) => {
                warn!("New message has been received: {:?}", m);
                let msg = bot.send_message(m.chat.id, "pong!");
                dbg!(serde_json::to_string(&msg));
                msg.send().await.unwrap();
            },
            _ => {},
        }
        SessionState::Continue(s)
    };
    let mut dp = Dispatcher::<'_, (), _>::new(handler);
    info!("Starting the message handler.");
    loop {
        let u = updater.next().await.unwrap();
        match u {
            Err(e) => { error!("{}", e) },
            Ok(u) => {
                let _ = dp.dispatch(u).await;
            }
        }
    }
}
