use crate::nodes::definition::NodeHandler;
use crate::nodes::result::NodeResult;
use crate::nodes::NodeContext;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use zen_expression::variable::ToVariable;
use zen_types::decision::FunctionContent;

#[derive(Debug, Clone)]
pub struct FunctionV2NodeHandler;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToVariable)]
pub struct FunctionV2Trace;

impl NodeHandler for FunctionV2NodeHandler {
    type NodeData = FunctionContent;
    type TraceData = FunctionV2Trace;

    async fn handle(&self, ctx: NodeContext<Self::NodeData, Self::TraceData>) -> NodeResult {
        Err(ctx.make_error(anyhow::anyhow!("QuickJS feature is disabled")))
    }
}

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum FunctionError {
        #[error("QuickJS is disabled")]
        Disabled,
    }

    pub type FunctionResult<T = ()> = Result<T, FunctionError>;
}

pub mod listener {
    pub trait RuntimeListener {}
}

pub mod function {
    use super::error::FunctionResult;
    use super::listener::RuntimeListener;

    pub struct FunctionConfig {
        pub listeners: Option<Vec<Box<dyn RuntimeListener>>>,
    }

    #[derive(Debug)]
    pub struct Function;

    impl Function {
        pub async fn create(_config: FunctionConfig) -> FunctionResult<Self> {
            Err(super::error::FunctionError::Disabled)
        }
    }
}

pub mod module {
    pub mod console {
        pub struct ConsoleListener;
        impl super::super::listener::RuntimeListener for ConsoleListener {}
    }

    pub mod http {
        pub mod listener {
            use crate::nodes::function::http_handler::DynamicHttpHandler;

            pub struct HttpListener {
                pub http_handler: DynamicHttpHandler,
            }

            impl super::super::super::listener::RuntimeListener for HttpListener {}
        }
    }

    pub mod zen {
        use crate::loader::DynamicLoader;
        use crate::nodes::custom::DynamicCustomNode;
        use crate::nodes::function::http_handler::DynamicHttpHandler;

        pub struct ZenListener {
            pub loader: DynamicLoader,
            pub custom_node: DynamicCustomNode,
            pub http_handler: DynamicHttpHandler,
        }

        impl super::super::listener::RuntimeListener for ZenListener {}
    }
}
