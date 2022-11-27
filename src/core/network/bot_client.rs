use std::collections::HashMap;
use std::f32::consts::E;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::Sender;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use futures_util::lock::Mutex;
use log::info;
use serde_json::{to_string, Value};
use tokio_tungstenite::{connect_async};
use tokio_tungstenite::tungstenite::{Error, Message, WebSocket};

use crate::core::bot::{Bot, ResultFrame};
use crate::core::component::event::{Event, PostType};
use crate::core::component::message::text;

use serde::{Deserialize, Serialize};
use crate::bot::event_handle;
use crate::core::event::{EventHandler, GroupMessageEvent};

use crate::service::CONTEXT;

pub struct WsClient;

impl WsClient {
    pub async fn run() {
        let config = CONTEXT.bot_config.clone();
        let url = match config.access_token {
            Some(token) => {
                let url = config.url.expect("[Bot] websocket url is null");
                url::Url::parse_with_params(url.as_str(), &[("access_token", token)]).unwrap()
            }
            None => {
                let url = config.url.expect("[Bot] websocket url is null");
                url::Url::parse(url.as_str()).unwrap()
            }
        };
        let (api_sender, mut api_receiver) = mpsc::channel(1024);

        let (ws_steam, _) = connect_async(url).await.unwrap();
        let bot_id = config.bot_id.unwrap();
        let (mut ws_out, mut ws_in) = ws_steam.split();

        let resp_promises = Arc::new(Mutex::new(HashMap::new()));
        let bot = Bot {
            bot_id: bot_id.clone(),
            bot_name: config.bot_name.unwrap(),
            api_sender: mpsc::Sender::clone(&api_sender),
            resp_promises: resp_promises.clone(),
        };


        info!("[Bot] [client] WebSocket handshake has been successfully completed");

        // 发送 api
        let mut send_task = tokio::spawn(async move {
            while let Some(frame) = api_receiver.recv().await {
                let data = frame.data.unwrap();
                if ws_out.send(Message::text(to_json(data))).await.is_err() {
                    break;
                }
            }
        });

        // 获取 api
        let mut recv_task = tokio::spawn(async move {
            while let Some(msg) = ws_in.next().await {
                let msg = msg.unwrap();
                match msg {
                    Message::Text(msg) => {
                        let json: Value = serde_json::from_str(msg.as_str()).unwrap();
                        let mut bot = bot.clone();

                        tokio::spawn(async move {
                            //将数据 转换成具体结构体
                            let event = Event::get_event(json);
                            match event {
                                Some(event) => {
                                    match event {
                                        Event::ApiResult(event) => {
                                            // 返回消息处理
                                            if let Some(api_resp_sender) = bot.resp_promises.lock().await.remove(event.echo.as_str()) {
                                                let data = event.data;
                                                if event.status.contains("ok") {
                                                    if data["message_id"].is_null() {
                                                        let frame1 = ResultFrame {
                                                            bot_id: bot_id.clone(),
                                                            echo: event.echo,
                                                            ok: true,
                                                            data: Some(data),
                                                            message_id: 0,
                                                        };
                                                        api_resp_sender.send(frame1);
                                                    } else {
                                                        let frame1 = ResultFrame {
                                                            bot_id: bot_id.clone(),
                                                            echo: event.echo,
                                                            ok: true,
                                                            data: Some(data.clone()),
                                                            message_id: data["message_id"].as_i64().unwrap(),
                                                        };
                                                        api_resp_sender.send(frame1);
                                                    }
                                                } else {
                                                    let frame1 = ResultFrame {
                                                        bot_id: bot_id.clone(),
                                                        echo: event.echo,
                                                        ok: false,
                                                        data: Some(data),
                                                        message_id: 0,
                                                    };
                                                    api_resp_sender.send(frame1);
                                                }
                                            }
                                        }
                                        _ => {
                                            // 所有事件处理
                                            event_handle(event, &mut bot).await;
                                        }
                                    }
                                }

                                None => {
                                    //什么事件都没匹配到...
                                }
                            }
                        });
                    }
                    Message::Close(_) => { break; }
                    _ => {
                        break;
                    }
                }
            }
        });
        tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
            }
        ;
    }
}

fn to_json(v: Value) -> String {
    serde_json::to_string(&v).unwrap()
}