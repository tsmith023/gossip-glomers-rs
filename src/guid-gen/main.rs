use async_trait::async_trait;
use maelstrom::{done, protocol::Message, Node, Result, Runtime};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Default)]
struct GenerateHandler {}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
struct ReplyGenerate {
    /// Message type.
    #[serde(rename = "type", default)]
    pub typ: String,
    #[serde(default)]
    pub id: u64,
}

impl ReplyGenerate {
    fn with_guid(self, id: u64) -> Self {
        let mut t = self;
        t.id = id;
        t
    }
    fn with_type(self, typ: impl Into<String>) -> Self {
        let mut t = self;
        t.typ = typ.into();
        t
    }
}

#[async_trait]
impl Node for GenerateHandler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        if req.get_type() == "generate" {
            return runtime
                .send(
                    req.src,
                    ReplyGenerate::default()
                        .with_type("generate_ok")
                        .with_guid(Uuid::new_v4().as_u64_pair().0),
                )
                .await;
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
