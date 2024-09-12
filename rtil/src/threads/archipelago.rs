use std::error::Error;
use std::thread;
use archipelago_rs::client::{ArchipelagoClient, ArchipelagoClientReceiver, ArchipelagoError};
use archipelago_rs::protocol::ServerMessage;
use crossbeam_channel::Sender;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::AbortHandle;
use crate::threads::{ArchipelagoToRebo, ReboToArchipelago};

pub fn run(archipelago_rebo_tx: Sender<ArchipelagoToRebo>, mut rebo_archipelago_rx: UnboundedReceiver<ReboToArchipelago>) {
    thread::spawn(move || {
        loop {
            let future = async {
                let mut sender = None;
                let mut receiver_abort_handle: Option<AbortHandle> = None;
                loop {
                    match rebo_archipelago_rx.recv().await.unwrap() {
                        ReboToArchipelago::Connect { server_and_port, game, slot, password, items_handling, tags } => {
                            if let Some(receiver) = receiver_abort_handle {
                                receiver.abort();
                            }
                            let mut client = ArchipelagoClient::new(&server_and_port).await?;
                            log!("Connected to archipelago server `{server_and_port}`");
                            client.connect(&game, &slot, password.as_deref(), items_handling, tags)
                                .await?;
                            let (s, mut receiver) = client.split();
                            sender = Some(s);
                            let join_handle = tokio::spawn(handle_receiver(receiver, archipelago_rebo_tx.clone()));
                            receiver_abort_handle = Some(join_handle.abort_handle());
                            log!("Connected to game `{game}` slot `{slot}`");
                        },
                        ReboToArchipelago::ClientMessage(msg) => sender.as_mut().unwrap().send(msg).await?,
                        ReboToArchipelago::Disconnect => {
                            drop(sender.take());
                            if let Some(abort_handle) = receiver_abort_handle.take() {
                                abort_handle.abort();
                            }
                        }
                    }
                }
                Ok::<(), Box<dyn Error>>(())
            };

            let res = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(future);
            archipelago_rebo_tx.send(ArchipelagoToRebo::ConnectionAborted).unwrap();
            log!("Archipelago thread finished with {res:?}, restarting...");
        }
    });
}

async fn handle_receiver(mut receiver: ArchipelagoClientReceiver, archipelago_rebo_tx: Sender<ArchipelagoToRebo>) {
    loop {
        match receiver.recv().await {
            Ok(None) => continue,
            Err(e) => {
                log!("Error reading from archipelago server: {e:?}");
                archipelago_rebo_tx.send(ArchipelagoToRebo::ConnectionAborted).unwrap();
                break;
            }
            Ok(Some(msg)) => archipelago_rebo_tx.send(ArchipelagoToRebo::ServerMessage(msg)).unwrap(),
        }
    }
}
