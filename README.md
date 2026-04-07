# DePIN AI Demo

Minimal example of an AI-powered W3bstream prover built with Rust and Risc Zero.

The demo shows how device data can be aggregated, processed by a prover that embeds simple AI logic, and then turned into a proof that can be consumed by a DePIN application.

## What This Demo Shows

This repository combines three pieces:

- a tiny model-generation step
- a Rust prover built on Risc Zero
- a W3bstream-oriented flow for turning device messages into verifiable output

It is best read as a technical demo, not as a production-ready template.

## Workflow

1. Generate the demo AI model.
2. Build and run the prover.
3. Feed sample device messages into the proving flow.
4. Use the resulting proof inside a W3bstream / blockchain integration.

## Quick Start

### 1. Generate the model

```bash
cd gen_model
cargo run
```

### 2. Build and run the prover

```bash
cd ..
cargo run --release
```

On Apple Silicon, you can also try GPU acceleration:

```bash
cargo run --release -F metal
```

## Expected Output

A successful run should:

- load the generated model
- aggregate the sample task input
- execute the guest prover logic
- produce a proof of execution

The repository includes example output in the current README history and sample input in `test_data.csv`.

## Where This Fits

This demo is useful if you want to explore:

- W3bstream-based DePIN architectures
- zk / proving pipelines around device data
- Rust-based prover workflows
- the shape of an end-to-end “real world data -> proof -> on-chain action” system

## W3bstream Integration

After building the prover, you can package it as a W3bstream project and wire its output into a smart contract flow.

The original demo notes in this repository still describe:

- how to create a W3bstream project file
- how to run the project locally on a node
- how to connect proof output to an EVM contract

If you are adapting this for a real project, review those integration sections carefully and update addresses, ABIs, and operational assumptions before using them outside a demo environment.

## Caveats

This repository is intentionally experimental:

- the proving flow is not optimized
- example contracts and endpoints may age quickly
- several sections are still demo-oriented or incomplete

Use it as a learning scaffold and a reference architecture, not as a drop-in production deployment.