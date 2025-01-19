use async_trait::async_trait;
use maelstrom::{done, protocol::Message, Node, Result, Runtime};
use std::sync::Arc;

#[derive(Clone, Default)]
struct EchoHandler {}

#[async_trait]
impl Node for EchoHandler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        if req.get_type() == "echo" {
            let echo = req.body.clone().with_type("echo_ok");
            return runtime.reply(req, echo).await;
        }
        done(runtime, req)
    }
}

pub(crate) fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let handler = Arc::new(EchoHandler::default());
    Runtime::new().with_handler(handler).run().await
}
