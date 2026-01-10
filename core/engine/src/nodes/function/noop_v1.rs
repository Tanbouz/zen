use crate::nodes::definition::NodeHandler;
use crate::nodes::result::NodeResult;
use crate::nodes::NodeContext;
use std::fmt::Debug;
use std::sync::Arc;
use zen_expression::variable::ToVariable;

#[derive(Debug, Clone)]
pub struct FunctionV1NodeHandler;

#[derive(Debug, Clone, Default, ToVariable)]
pub struct FunctionV1Trace;

impl NodeHandler for FunctionV1NodeHandler {
    type NodeData = Arc<str>;
    type TraceData = FunctionV1Trace;

    async fn handle(&self, ctx: NodeContext<Self::NodeData, Self::TraceData>) -> NodeResult {
        Err(ctx.make_error(anyhow::anyhow!("QuickJS feature is disabled")))
    }
}
