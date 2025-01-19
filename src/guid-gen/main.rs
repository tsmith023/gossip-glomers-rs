use async_trait::async_trait;
use maelstrom::{done, protocol::Message, Node, Result, Runtime};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Default)]
struct GenerateHandler {}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
struct GuidGenMessageBody {
    /// Message type.
    #[serde(rename = "type", default)]
    pub typ: String,

    #[serde(default)]
    pub msg_id: u64,

    #[serde(default)]
    pub in_reply_to: u64,

    #[serde(default)]
    pub id: u64,
}

#[async_trait]
impl Node for GenerateHandler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        if req.get_type() == "generate" {
            let new = Uuid::new_v4().as_u64_pair();
            let generate_msg = GuidGenMessageBody {
                typ: "generate_ok".to_string(),
                msg_id: new.0,
                in_reply_to: req.body.msg_id,
                id: new.0,
            };
            return runtime.send(req.src, generate_msg).await;
        }
        done(runtime, req)
    }
}

pub(crate) fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let handler = Arc::new(GenerateHandler::default());
    Runtime::new().with_handler(handler).run().await
}
