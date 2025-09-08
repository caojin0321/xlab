<h1>Yoob AI</h1>

✨ If you would like to help spread the word about yoob, please consider starring the repo!

> [!WARNING]
> Here be dragons! As we plan to ship a torrent of features in the following months, future updates **will** contain **breaking changes**. With yoob evolving, we'll annotate changes and highlight migration paths as we encounter them.

## Table of contents

- [Table of contents](#table-of-contents)
- [What is yoob?](#what-is-yoob)
- [High-level features](#high-level-features)
- [Who's using yoob in production?](#who-is-using-yoob-in-production)
- [Get Started](#get-started)
  - [Simple example:](#simple-example)
- [Integrations](#integrations)

## What is yoob?
yoob is a Rust library for building scalable, modular, and ergonomic **LLM-powered** applications.

More information about this crate can be found in the [official](https://docs.yoob.rs) & [crate](https://docs.rs/yoob-core/latest/yoob/) (API Reference) documentations.

## High-level features
- Full support for LLM completion and embedding workflows
- Simple but powerful common abstractions over LLM providers (e.g. OpenAI, Cohere) and vector stores (e.g. MongoDB, SQlite, in-memory)
- Integrate LLMs in your app with minimal boilerplate

## Who is using yoob in production?
Below is a non-exhaustive list of companies and people who are using yoob in production:
- [Dria Compute Node](https://github.com/firstbatchxyz/dkn-compute-node) - a node that serves computation results within the Dria Knowledge Network
- [The MCP Rust SDK](https://github.com/modelcontextprotocol/rust-sdk ) - the official Model Context Protocol Rust SDK. Has an example for usage with yoob.
- [Probe](https://github.com/buger/probe) - an AI-friendly, fully local semantic code search tool.
- [NINE](https://github.com/NethermindEth/nine) - Neural Interconnected Nodes Engine, by [Nethermind.](https://www.nethermind.io/)
- [yoob-onchain-kit](https://github.com/caojin0321/yoob-onchain-kit) - the yoob Onchain Kit. Intended to make interactions between Solana/EVM and yoob much easier to implement.
- [Linera Protocol](https://github.com/linera-io/linera-protocol) - Decentralized blockchain infrastructure designed for highly scalable, secure, low-latency Web3 applications.
- [Listen](https://github.com/piotrostr/listen) - A framework aiming to become the go-to framework for AI portfolio management agents. Powers [the Listen app.](https://app.listen-rs.com/)

Are you also using yoob in production? [Open an issue](https://www.github.com/caojin0321/yoob/issues) to have your name added!

## Get Started
```bash
cargo add yoob-core
```

### Simple example:
```rust
use yoob::{completion::Prompt, providers::openai};

#[tokio::main]
async fn main() {
    // Create OpenAI client and model
    // This requires the `OPENAI_API_KEY` environment variable to be set.
    let openai_client = openai::Client::from_env();

    let gpt4 = openai_client.agent("gpt-4").build();

    // Prompt the model and print its response
    let response = gpt4
        .prompt("Who are you?")
        .await
        .expect("Failed to prompt GPT-4");

    println!("GPT-4: {response}");
}
```
Note using `#[tokio::main]` requires you enable tokio's `macros` and `rt-multi-thread` features
or just `full` to enable all features (`cargo add tokio --features macros,rt-multi-thread`).

You can find more examples each crate's `examples` (ie. [`yoob-core/examples`](./yoob-core/examples)) directory. More detailed use cases walkthroughs are regularly published on our [Dev.to Blog](https://dev.to/0thtachi) and added to yoob's official documentation [(docs.yoob.rs)](http://docs.yoob.rs).

## Supported Integrations

Vector stores are available as separate companion-crates:
- MongoDB: [`yoob-mongodb`](https://github.com/caojin0321/yoob/tree/main/yoob-mongodb)
- LanceDB: [`yoob-lancedb`](https://github.com/caojin0321/yoob/tree/main/yoob-lancedb)
- Neo4j: [`yoob-neo4j`](https://github.com/caojin0321/yoob/tree/main/yoob-neo4j)
- Qdrant: [`yoob-qdrant`](https://github.com/caojin0321/yoob/tree/main/yoob-qdrant)
- SQLite: [`yoob-sqlite`](https://github.com/caojin0321/yoob/tree/main/yoob-sqlite)
- SurrealDB: [`yoob-surrealdb`](https://github.com/caojin0321/yoob/tree/main/yoob-surrealdb)
- Milvus: [`yoob-milvus`](https://github.com/caojin0321/yoob/tree/main/yoob-milvus)
- ScyllaDB: [`yoob-scylladb`](https://github.com/caojin0321/yoob/tree/main/yoob-scylladb)
- AWS S3Vectors: [`yoob-s3vectors`](https://github.com/caojin0321/yoob/tree/main/yoob-s3vectors)

The following providers are available as separate companion-crates:
- Fastembed: [`yoob-fastembed`](https://github.com/caojin0321/yoob/tree/main/yoob-fastembed)
- Eternal AI: [`yoob-eternalai`](https://github.com/caojin0321/yoob/tree/main/yoob-eternalai)


<p align="center">
<br>
<br>
</p>
