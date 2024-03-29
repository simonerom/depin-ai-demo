# depin-ai-demo

A simple example to demonstrate how to run an AI-based prover in W3nstream.

## Quick Start

### Create the model

```sh
cd gen_model
cargo run
```

You should see an output like: 
```sh
Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/runner_ai`
********* Training  Model **********
100 samples. Accuracy: 1
********** Model trained **********
Prediction: [92.0, 76.0, 400.0] -> Running
```

The model file is located in the `model`directory. Feel free to customize the training data in the `data/` directory and customize `src/main.rs` accordingly.

### Build the W3bstream prover

### Deploy the test contract

### Deploy the W3bstream configuration

### Send test data