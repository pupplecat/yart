# YART (Yet Another Rig Tool)

YART is a Rust library suite for creating tools with the `rig::tool::Tool` trait, providing a flexible procedural macro (`#[rig_tool]`) for generating tool implementations with JSON schema support. It is designed for the `rig` framework, enabling async tool definitions with optional context and arguments.

## Structure

YART is split into three crates to adhere to Rust’s proc-macro restrictions and provide a clean API:

- **`yart_shared`**: Contains shared types (`ToolError`, `ToolOutput`) and utilities (`derive_parameters`) used by generated code and consumers.
- **`yart_macro`**: A proc-macro crate exporting the `#[rig_tool]` macro, which generates `rig::tool::Tool` implementations.
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

Clone the YART repository and add the crates to your project’s `Cargo.toml`:

```toml
[dependencies]
yart = { path = "../yart/yart" }
```

Alternatively, if published to crates.io, use:

```toml
[dependencies]
yart = "0.1.0"
```

The `yart` crate depends on `yart_shared` and `yart_macro`, so you only need to include `yart`.

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
├── yart/              # Wrapper crate
│   ├── src/lib.rs     # Re-exports yart_macro and yart_shared
│   └── Cargo.toml
├── yart_macro/        # Proc-macro crate
│   ├── src/lib.rs     # Defines #[rig_tool] macro
│   └── Cargo.toml
├── yart_shared/       # Shared types and utilities
│   ├── src/lib.rs     # Defines ToolError, ToolOutput, derive_parameters
│   └── Cargo.toml
└── README.md
```

## Testing

Run tests for each crate:

```bash
# Test yart_shared
cd yart/yart_shared
cargo test

# Test yart_macro
cd yart/yart_macro
cargo test

# Test yart (integration tests)
cd yart/yart
cargo test
```

## Limitations

- The `description` attribute is required; enforced at compile-time but tested manually due to proc-macro testing constraints.
- Consider using `trybuild` for robust macro testing in `yart_macro`.

## Contributing

Contributions are welcome! Please submit pull requests or open issues on the project repository.

## License

MIT
