use async_std::sync::RwLock;
use async_trait::async_trait;
use maelstrom::{
    done,
    protocol::{Message, MessageBody},
    Node, Result, Runtime,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone, Default)]
struct BroadcastHandler {
    messages: Arc<RwLock<Vec<u64>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
struct BroadcastReceive {
    #[serde(default)]
    pub typ: String,
    #[serde(default)]
    pub message: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
struct ReadReply {
    #[serde(default)]
    pub messages: Vec<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
struct TopologyReceive {
    #[serde(default)]
    pub topology: HashMap<String, Vec<String>>,
}

impl ReadReply {
    fn with_messages(self, msgs: &Vec<u64>) -> Self {
        let mut t = self;
        t.messages = msgs.to_vec();
        t
    }
}

#[async_trait]
impl Node for BroadcastHandler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        if req.get_type() == "broadcast" {
            let body = req.body.as_obj::<BroadcastReceive>()?;
            self.messages.write().await.push(body.message);
            runtime.reply(req.clone(), &MessageBody::default()).await?;
        }
        if req.get_type() == "read" {
            let msgs = self.messages.read().await;
            runtime
                .reply(req.clone(), ReadReply::default().with_messages(&msgs))
                .await?;
        }
        if req.get_type() == "topology" {
            let _ = req.body.as_obj::<TopologyReceive>()?;
            runtime.reply(req.clone(), MessageBody::default()).await?;
        }
        done(runtime, req)
    }
}

pub(crate) fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let handler = Arc::new(BroadcastHandler::default());
    Runtime::new().with_handler(handler).run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use maelstrom::Runtime;
    use serde_json::{json, Map, Value};

    #[tokio::test]
    async fn handler_process_broadcast_no_panic() {
        let handler = BroadcastHandler::default();
        let mut extra = Map::new();
        extra.insert("message".to_string(), json![0]);
        let _ = handler
            .process(
                Runtime::new(),
                Message {
                    src: "n0".to_string(),
                    dest: "n1".to_string(),
                    body: MessageBody {
                        typ: "broadcast".to_string(),
                        msg_id: 0,
                        in_reply_to: 0,
                        extra: extra,
                    },
                },
            )
            .await;
        assert_eq!(handler.messages.read_blocking().to_vec(), vec![0]);
    }

    #[tokio::test]
    async fn handler_process_read_no_panic() {
        let handler = BroadcastHandler {
            messages: Arc::new(RwLock::new(vec![1])),
        };
        let _ = handler
            .process(
                Runtime::new(),
                Message {
                    src: "n0".to_string(),
                    dest: "n1".to_string(),
                    body: MessageBody {
                        typ: "read".to_string(),
                        msg_id: 0,
                        in_reply_to: 0,
                        extra: Map::default(),
                    },
                },
            )
            .await;
        assert_eq!(handler.messages.read_blocking().to_vec(), vec![1]);
    }

    #[tokio::test]
    async fn handler_process_topology_no_panic() {
        let handler = BroadcastHandler::default();

        let mut topology = Map::new();
        topology.insert("n0".to_string(), vec!["n0"].into());

        let mut extra = Map::new();
        extra.insert("topology".to_string(), Value::Object(topology));

        let _ = handler
            .process(
                Runtime::new(),
                Message {
                    src: "n0".to_string(),
                    dest: "n1".to_string(),
                    body: MessageBody {
                        typ: "topology".to_string(),
                        msg_id: 0,
                        in_reply_to: 0,
                        extra: extra,
                    },
                },
            )
            .await;
        assert_eq!(handler.messages.read_blocking().to_vec(), Vec::<u64>::new());
    }
}
