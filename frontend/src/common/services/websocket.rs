use std::{cell::RefCell, net::Shutdown, rc::Rc};

use gloo_net::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{channel::mpsc::Sender, SinkExt, StreamExt, FutureExt};
use yew::Callback;


pub struct WebsocketService {
    sender: Option<Sender<Message>>,
    // on_text: Callback<String>,
    shutdown_tx: Option<futures::channel::oneshot::Sender<()>>
}

impl WebsocketService {
    pub fn new() -> Self {
        Self {
            sender: None,
            // on_text: on_text,
            shutdown_tx: None
        }
    }

    pub async fn connect(&mut self, url : &str, on_text: Callback<String>) {
        let ws = WebSocket::open(url).unwrap();
        
        let (shutdown_tx, mut shutdown_rx) = futures::channel::oneshot::channel::<()>();
        
        // if let Some(ws) = self.ws.take() {
            //ws.close(Some(1000u16), Some("Disconnected by user"));
            let (mut write, mut read) = ws.split();
            let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<Message>(1000);
        
            self.sender = Some(in_tx);

            // let on_text_clone = self.on_text.clone();

            // spawn_local(async move {
            //     loop {
            //         futures::select! {
            //             msg = read.next().fuse() => {
            //                 match msg {
            //                     Some(Ok(Message::Text(data))) => {
            //                         gloo::console::log!("from websocket: {}", data.clone());
            //                         on_text_clone.emit(data);
                                    
            //                     }
            //                     Some(Ok(Message::Bytes(b))) => {
            //                         let decoded = std::str::from_utf8(&b);
            //                         if let Ok(val) = decoded {
            //                             gloo::console::log!("from websocket: {}", val);
            //                         }
            //                     }
            //                     Some(Err(e)) => {
            //                         gloo::console::log!("ws: {:?}", e.to_string());
            //                     },
            //                     None => {
            //                         break;
            //                     }
            //                 }
            //             },
            //             msg = in_rx.next().fuse() => {
            //                 if let Some(msg) = msg {
            //                     match msg {
            //                         Message::Text(txt) => {
            //                             write.send(Message::Text(txt)).await.unwrap();
            //                         },
            //                         Message::Bytes(items) => {
            //                             write.send(Message::Bytes(items)).await.unwrap();
            //                         },
            //                     }
            //                 }
            //                 else {
            //                     break;
            //                 }
            //             }
            //         }
            //     }
            //     gloo::console::log!("WebSocket Closed");
            // });
            
        
            spawn_local(async move {
                while let Some(msg) = in_rx.next().fuse().await {
                    match msg {
                        Message::Text(txt) => {
                            gloo::console::log!("got text from channel! {}", txt.clone());
                            write.send(Message::Text(txt)).await.unwrap();
                        },
                        Message::Bytes(items) => {
                            //gloo::console::log!("got binary from channel! {}", items.clone());
                            write.send(Message::Bytes(items)).await.unwrap();
                        },
                    }
                }
                shutdown_tx.send(()).unwrap();
                gloo::console::log!("Channel shutdown");
            });
            
            let on_text_clone = on_text.clone();

            spawn_local(async move {
                loop {
                    futures::select! {
                        msg = read.next().fuse() => {
                            if let Some(msg) = msg {
                                match msg {
                                    Ok(Message::Text(data)) => {
                                        gloo::console::log!("from websocket: {}", data.clone());
                                        on_text_clone.emit(data);
                                        
                                    }
                                    Ok(Message::Bytes(b)) => {
                                        let decoded = std::str::from_utf8(&b);
                                        if let Ok(val) = decoded {
                                            gloo::console::log!("from websocket: {}", val);
                                        }
                                    }
                                    Err(e) => {
                                        gloo::console::log!("ws: {:?}", e.to_string())
                                    }
                                }
                            }
                        },
                        _ = shutdown_rx => {
                            break;
                        }
                    }
                }
                // while let Some(msg) = read.next().await {
                //     gloo::console::log!(msg.is_ok());
                //     match msg {
                //         Ok(Message::Text(data)) => {
                //             gloo::console::log!("from websocket: {}", data.clone());
                //             on_text_clone.emit(data);
                            
                //         }
                //         Ok(Message::Bytes(b)) => {
                //             let decoded = std::str::from_utf8(&b);
                //             if let Ok(val) = decoded {
                //                 gloo::console::log!("from websocket: {}", val);
                //             }
                //         }
                //         Err(e) => {
                //             gloo::console::log!("ws: {:?}", e.to_string())
                //         }
                //     }
                    
                // }
                gloo::console::log!("WebSocket Closed");
            });
        // }  
    }

    pub fn get_sender(&mut self) -> &Sender<Message> {
        match self.sender {
            Some(ref value) => value,
            None => {
                panic!("Websocket is not opened. Consider calling connect before");
            },
        }
    }

    pub fn disconnect (&mut self) {
        if let Some(sender) = self.sender.take() {
            drop(sender);
        }
        // if let Some(ws) = self.ws.take() {
        //     drop(ws);
        //     //ws.close(Some(1000u16), Some("Disconnected by user")).unwrap();
        // }
    }
}