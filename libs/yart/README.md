# YART

YART (Yet Another Rig Tool) is a Rust library for creating tools with the `rig::tool::Tool` trait, providing a convenient procedural macro (`#[rig_tool]`) to generate tool implementations with JSON schema support. It re-exports the `#[rig_tool]` macro from `yart_macro` and shared types/utilities from `yart_shared`, offering a unified API for the `rig` framework.

## Purpose

The `yart` crate is the primary entry point for using YART. It simplifies integration by combining the procedural macro and shared utilities into a single dependency, enabling developers to define async tools with minimal boilerplate.

## Features

- **Procedural Macro**: The `#[rig_tool]` macro generates `rig::tool::Tool` implementations for async functions with 0-2 arguments (optional context, optional args).
- **JSON Schema Support**: Automatically generates JSON schemas for tool arguments using `schemars`.
- **Shared Utilities**: Provides `ToolError`, `ToolOutput`, and `derive_parameters` for consistent error handling and output serialization.
- **Async Support**: Wraps async functions with `Result` returns, handling errors via `ToolError`.
- **Unified API**: Import all functionality with `use yart::*`.

## Installation

Add `yart` to your project’s `Cargo.toml`:

```toml
[dependencies]
yart = "0.1.1"
```

## Usage

Use the `#[rig_tool]` macro to define tools, specifying a `description` and optional `name`. The macro generates a struct implementing `rig::tool::Tool` with methods for instantiation, naming, schema definition, and execution.

### Example

```rust
use std::sync::Arc;
use yart::{ToolError, ToolOutput};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Define argument type
#[derive(Deserialize, Serialize, JsonSchema)]
struct TestArgs {
    input: String,
}

// Define output type
#[derive(Deserialize, Serialize)]
struct TestOutput {
    result: String,
}

// Tool without context
#[yart::rig_tool(description = "Echoes input")]
async fn echo_tool(args: TestArgs) -> anyhow::Result<TestOutput, ToolError> {
    Ok(TestOutput { result: args.input })
}

// Tool with context
#[derive(Clone)]
struct TestContext {
    value: String,
}

#[yart::rig_tool(name = "context_tool", description = "Echoes input with context")]
async fn context_tool(
    ctx: Arc<TestContext>,
    args: TestArgs,
) -> anyhow::Result<TestOutput, ToolError> {
    Ok(TestOutput {
        result: format!("{}: {}", ctx.value, args.input),
    })
}

#[tokio::main]
async fn main() {
    // Use tool without context
    let tool = EchoTool::new();
    let args = TestArgs { input: "hello".to_string() };
    let result = tool.call(args).await.unwrap();
    let output: TestOutput = serde_json::from_value(result.result).unwrap();
    println!("Output: {}", output.result); // Output: hello

    // Use tool with context
    let ctx = Arc::new(TestContext { value: "test".to_string() });
    let tool = ContextTool::new(ctx);
    let args = TestArgs { input: "world".to_string() };
    let result = tool.call(args).await.unwrap();
    let output: TestOutput = serde_json::from_value(result.result).unwrap();
    println!("Output: {}", output.result); // Output: test: world
}
```

## Dependencies

The `yart` crate re-exports functionality from:

- **`yart-macro`**: Provides the `#[rig_tool]` macro for generating tool implementations.
- **`yart-shared`**: Supplies shared types (`ToolError`, `ToolOutput`) and utilities (`derive_parameters`).

You only need to depend on `yart` in your `Cargo.toml`, as it includes both `yart_macro` and `yart_shared`.

## Documentation

- **Crates.io**: [yart](https://crates.io/crates/yart)
- **Repository**: [github.com/pupplecat/yart](https://github.com/pupplecat/yart)
- **API Docs**: [docs.rs/yart](https://docs.rs/yart/0.1.1/yart)

## Contributing

Contributions are welcome! Please submit pull requests or open issues on the [project repository](https://github.com/pupplecat/yart).

## License

MIT
