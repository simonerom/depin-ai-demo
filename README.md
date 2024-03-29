# depin-ai-demo

A simple example demonstrating how to run an AI-based prover on W3bstream.

## Description

The AI model is designed to predict whether a user is walking or running. It operates under the assumption that the user wears "smart shoes" equipped with two smart sensors connected via bluetooth to each other and to a mobile app. As the user walks, the main sensor measures the vertical force exerted by the feets, and record the interval in milliseconds between two consecutive steps. It then calculates average values across several steps and sends this information to the mobile app that in turn sends it to the W3bstream project. The W3bstream network 'proves' whether the user is currently walking or running through AI prediction and sends the proof to the project's contract.

## Requirements

This demo is based on the **Smartcore** AI library for Rust, specifically **version 0.3.2**.

It utilizes the KNN Classifier for simplicity, and no special optimization is included in this example. The dataset, training process, classifier, and optimizations can be easily adapted to create a model for other use cases, but such customizations are left as an exercise for the reader.

## Quick Start

### Create the model

```sh
cd gen_model
cargo run
```

You should see an output similar to:

```sh
Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/runner_ai`
********* Training  Model **********
100 samples. Accuracy: 1
********** Model trained **********
Prediction: [92.0, 76.0, 400.0] -> Running
```

The model file is located in the `model`directory. Feel free to customize the training data in the `data/` directory and modify `src/main.rs` as needed.

### Build the W3bstream prover

### Deploy the test contract

### Deploy the W3bstream configuration

### Send test data
