use super::InitError;
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct AddArgs {
    /// The first number to add.
    x: i32,
    /// The second number to add.
    y: i32,
}
pub struct Adder;

impl Tool for Adder {
    const NAME: &'static str = "Add";

    type Error = rig::tool::ToolError;

    type Args = AddArgs;

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(AddArgs);
        ToolDefinition {
            name: "Add".to_string(),
            description: "Adds two numbers".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(format!("{}", args.x + args.y))
    }
}

impl ToolEmbedding for Adder {
    type InitError = InitError;

    type Context = ();

    type State = ();

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Add two numbers".into(), "x + y".into()]
    }

    fn context(&self) -> Self::Context {}

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Adder)
    }
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
/// Subtract the second value from the first. (x - y).
pub struct SubArgs {
    /// The first number to subtract.
    x: i32,
    /// The second number to subtract.
    y: i32,
}
#[derive(Serialize, Deserialize)]
pub struct Subtract;

impl Tool for Subtract {
    const NAME: &'static str = "Subtract";

    type Error = rig::tool::ToolError;

    type Args = SubArgs;

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(SubArgs);
        ToolDefinition {
            name: "Subtract".to_string(),
            description: "Subtract two numbers".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // println!("{:?}", args);
        Ok(format!("{}", args.x - args.y))
    }
}

impl ToolEmbedding for Subtract {
    type InitError = InitError;

    type Context = ();

    type State = ();

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Subtract two numbers".into(), "x - y".into()]
    }

    fn context(&self) -> Self::Context {}

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Subtract)
    }
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
/// Multiply two numbers.
pub struct MultArgs {
    /// The first number to multiply.
    x: i32,
    /// The second number to multiply.
    y: i32,
}
#[derive(Serialize, Deserialize)]
pub struct Multiply;

impl Tool for Multiply {
    const NAME: &'static str = "Multiply";

    type Error = rig::tool::ToolError;

    type Args = MultArgs;

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(SubArgs);
        ToolDefinition {
            name: "Multiply".to_string(),
            description: "Multiply two numbers".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // println!("{:?}", args);
        Ok(format!("{}", args.x * args.y))
    }
}

impl ToolEmbedding for Multiply {
    type InitError = InitError;

    type Context = ();

    type State = ();

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Multiply two numbers".into(), "x * y".into()]
    }

    fn context(&self) -> Self::Context {}

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Multiply)
    }
}
