# YART (Yet Another Rig Tool)

YART is a Rust library suite for creating tools with the `rig::tool::Tool` trait, providing a flexible procedural macro (`#[rig_tool]`) for generating tool implementations with JSON schema support. It is designed for the `rig` framework, enabling async tool definitions with optional context and arguments.

- Example Project: [github.com/pupplecatg/yart-example](https://github.com/pupplecatg/yart-example)

## Structure

YART is split into three crates to adhere to Rust’s proc-macro restrictions and provide a clean API:

- **`yart-shared`**: Contains shared types (`ToolError`, `ToolOutput`) and utilities (`derive_parameters`) used by generated code and consumers.
- **`yart-macro`**: A proc-macro crate exporting the `#[rig_tool]` macro, which generates `rig::tool::Tool` implementations.
- **`yart`**: A wrapper crate that re-exports the `#[rig_tool]` macro from `yart_macro` and types/functions from `yart_shared`, offering a unified interface.

This structure mirrors conventions like `serde`/`serde_derive`, ensuring compatibility and modularity.

## Features

- **Flexible Macro**:
  - `#[rig_tool]` supports 0-2 arguments (optional context, optional args).
  - Attributes: `description` (required), `name` (optional, defaults to function name).
- **Generated Code**:
  - Creates a struct implementing `rig::tool::Tool` with `new`, `name`, `definition`, and `call` methods.
  - Generates JSON schemas for arguments using `schemars`.
- **Async Support**: Wraps async functions with `Result` returns, handling errors via `ToolError`.
- **Unified API**: Import everything via `yart` (e.g., `use yart::*`).

## Installation

```toml
[dependencies]
yart = "0.1.1"
```

The `yart` crate depends on `yart-shared` and `yart-macro`, so you only need to include `yart`.

## Usage

Define a tool using the `#[rig_tool]` macro, specifying a `description` and optional `name`. The macro generates a struct implementing `rig::tool::Tool`.

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

## Project Structure

```text
yart/
├── libs/
│   ├── yart/              # Wrapper crate
│   │   ├── src/lib.rs     # Re-exports yart-macro and yart-shared
│   │   └── Cargo.toml
│   ├── yart-macro/        # Proc-macro crate
│   │   ├── src/lib.rs     # Defines #[rig_tool] macro
│   │   └── Cargo.toml
│   └── yart-shared/       # Shared types and utilities
│       ├── src/lib.rs     # Defines ToolError, ToolOutput, derive_parameters
│       └── Cargo.toml
├── README.md
└── Cargo.toml
```

## Testing

Run tests for each crate:

```bash
# Test yart-shared
cd libs/yart-shared
cargo test

# Test yart-macro
cd libs/yart-macro
cargo test

# Test yart (integration tests)
cd libs/yart
cargo test
```

## Limitations

- The `description` attribute is required; enforced at compile-time but tested manually due to proc-macro testing constraints.
- Consider using `trybuild` for robust macro testing in `yart-macro`.

## Contributing

Contributions are welcome! Please submit pull requests or open issues on the project repository.

## License

MIT
