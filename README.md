# AI demo for Webstream

Welcome!
`depin-ai-demo` is a minimal example that demonstrates how to create a W3bstream prover based on some AI logic.

## Quick Start

This quick start guide, consisting of two steps, will walk you through the commands needed to generate the AI model, build, and test the Risc Zero prover. The following sections will show you how to create a W3bstream project based on this prover and utilize it to build a DePIN dApp.

### 1. Train the AI model

The first step is to train an AI model and export it in a format that can be statically imported into the W3bstream prover.

[Learn more about the demo AI model ↗](./gen_model/README.md)

```sh
cd gen_model
cargo run
```

### 2. Build and test the prover

Once the AI model is generated, you can include it in the Risc0 prover and build it. The file [test_data.csv](./test_data.csv) contains several test data points.

The following command will build the prover, mock a W3bstream task by aggregating together as many device messages as included in the `test_data.csv`, and pass the task to the prover to process it:

```sh
cd ..
cargo run --release

# On a Macbook with an Apple Silicon "M" chip, you can leverage GPU acceleration with: 
cargo run --release -F metal
```

You should see output like:

```text
Start proving at 2024-04-18 21:23:32
R0VM[938977] Processing a Task
R0VM[956178] project_id 74
R0VM[956822] task_id 1
R0VM[962070] client_id client_id
R0VM[962911] sequencer_sign 0x0
R0VM[1248980] Messages ...
R0VM[2614590] Guest execution completed on 10 messages.
R0VM[2617222] Public output from Journal is: {"1":2,"0":2,"3":1,"2":0}
R0VM[2617347] Proving starts now, please be patient!
Proving time: 86.25007425s
I generated a proof of guest execution!
```

## Create a W3bstream project

Once the risc0 prover is built you can locate it in the `target`directlry. To find the most recent generated you can use the command below:

```sh
export PROVER_FILE=$(find ./target/release -name 'methods.rs' -print0 | xargs -0 stat -f "%Sm: %N" -t "%Y-%m-%d %H:%M:%S" | sort | tail -n 1| awk -F': ' '{print $2}')
```

To create a W3bstream project based on a risc0 prover you have two options:

### 1. Use the developer portal

>*W3bstream's Project registration contract is still pending release*.

As a consequence, the UI is still not available.

### 2. Using the ioctl IoTeX client

#### Create the W3bstream project file

```sh
ioctl ws project config -s "postgres://test_user:test_passwd@postgres:5432/test?sslmode=disable" -t "risc0" -i $PROVER_FILE -c "74" -e "{\"image_id\":\"DEWI_ID\", \"elf\":\"DEWI_ELF\"}"
```

Make sure you use the correct image_id and elf as found in your `method.rs` prover file.

The command will generate a JSON config file named `74` which represents a W3bstream project file.

#### Edit the project file

The [default W3bstream project file](wsproject.json) is configured to generate a proof and output it to the W3bstream log. In order to write that proof to the blockchain and use it in your EVM smart contract you need to configure the `output` setting and set the `type` to `ethereumContract` and include the contract address to submit the proof to and the ABI:

```json
"output": {
        "type": "ethereumContract",
        "ethereum": {
          "chainEndpoint": "https://babel-api.testnet.iotex.io",
          "contractAddress": "0x1BFf17c79b5fa910cC77e95Ca82C7De26fC3C3b0",
          "receiverAddress": "0x66DFbaD50d80a376BFF7446A2895426dEC89c702",
          "contractMethod": "submit",
          "contractAbiJSON": "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_projectId\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_receiver\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"_data_snark\",\"type\":\"bytes\"}],\"name\":\"submit\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]"
        }
      },
...
```

In this configuration, you should replace the receiverContract field with your own dApp contract supposed to receive the risc0 proof, with [this interface](https://github.com/machinefi/sprout/blob/develop/smartcontracts/contracts/interfaces/IDapp.sol).

#### Register your W3bstream project

Project registration is required to make your w3bstream project configuration discoverable by the W3bstream nodes.

>*Project registration contract is still pending release*.

#### Run your project locally

For development purposes, you can avoid registering your W3bstream project on chain and run it locally on your W3bstream node.

- Follow how to run a W3bstream node from the [W3bstream Operator Quick Start](https://github.com/machinefi/sprout/blob/develop/docs/OPERATOR_GUIDE.md).
- Before starting the node, copy your W3bstream project file into the `test/container_model` folder (relative to the W3bstream `docker-compose.yaml` file).
- Edit the sequencer command arguments in  `docker-config.yaml` to setan aggregation amount for our W3bstream project (this is the number of messages that will be aggregated in a block before processing it in the prover). We set 10 in this demo:
  
```yaml
command: [ "-coordinatorAddress", "coordinator:9001", "-databaseDSN", "postgres://test_user:test_passwd@postgres:5432/test?sslmode=disable", "-aggregationAmount", "10"]
```

Make sure the name of your project file is an integer ID: that will be the project ID you will use to send messages to that project, we use 74 in this demo.

- Start the W3bstream node and log sequencer, coordinator and prover:

```sh
docker compose up -d && docker compose logs -f sequencer coordinator prover
```

## Deploy the Contracts

For our demo, we need a few contracts:

- a device registry contract that stores all authorized device IDs and their respective owner account
- an ERC20 token contract that we will use to mint rewards to be sent to device owners
- a "receiver" contract that hosts the main logic of our dApp: receive and validate W3bstream proofs, process the proof output to extract device rewards, lookup device owners in the registry, and finally distribute the rewards.

For this purpose, since the output of our prover is in the same form of the this demo (a `[device_id=>rewards]` mapping) we can use the same set of [contracts](https://github.com/machinefi/iotex-dewi-demo/tree/main/blockchain).

These contracts are already deployed and 5 device ids are already registered and assigned to owners:

```js
Token Contract: 0xff94dea0be4fc5289cb60f63d55eaff71b3e9666
dApp Receiver Contract: 0x66DFbaD50d80a376BFF7446A2895426dEC89c702
Device Registry Contract: TBD

Registered device and owners:

device_id | owner
------------------------------------------------------
0         | 0x1435fc1a9170f15d708fb837d0f8b8f06e8f16e6
1         | 0xc7c415f50829c1f696fb7c16df3635262bf99193
2         | 0x09bb7706adaf412f17da5ab61036df966d96413c
3         | 0xbcafe1986bb8130bea04de6c7482ba37dad77fbd
4         | 0x84158470e36c2d583f98d4e77de8a4d380818df6
------------------------------------------------------
```

## Send messages and receive rewards!

Once a W3bstream node is running and has our project loaded (either locally or from the project registry on-chain contract), we can send messages.

Make sure your ioctl client points to your W3bstream node:

```sh
ioctl config set wsEndpoint 'localhost:9000'    
```

Send messages with:

```sh
ioctl ws message send --project-id 74 --project-version "0.1"  --data "{\"device_id\":\"2\", \"right_force\":\"78\",\"left_force\":\"95\",\"interval_ms\":\"500\", \"receipt_type\":\"Snark\"}"
```

Check the log of W3bstream and notice how, after 10 messages (or whatever set as the aggregation amount) a W3bstream task should be dispatched to the prover and a proof returned after approximately 1.5 minutes and written to the blockchain. A a result, depending on the data messages, some device owner accounts should have received tokens as reward.  

## Customize the prover

TBD

### Customize the device message

TBD

### Customize the AI model

TBD

### Customize the prover logic

TBD

## Performance

This demo code is not optimized in any way. The proving time on a MacBook Pro M1 is approximately 90 seconds when using GPU acceleration.
