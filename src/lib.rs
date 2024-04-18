// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![doc = include_str!("../README.md")]

use chrono::Local;

use dewi_core::WSTask; 
use dewi_demo_methods::DEWI_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

// This is a Hello World demo for the RISC Zero zkVM.
// By running the demo, Alice can produce a receipt that proves that she knows
// some numbers a and b, such that a*b == 391.
// The factors a and b are kept secret.

// Compute the product a*b inside the zkVM
pub fn process_task(task: WSTask) -> (Receipt, String) {
    let env = ExecutorEnv::builder()
        // Send a & b to the guest
        .write(&task)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    // Compute proving start time
    
    let start = std::time::Instant::now();
    let date = Local::now();
    println!("Start execution at {}", date.format("%Y-%m-%d %H:%M:%S"));
    let receipt = prover.prove(env, DEWI_ELF).unwrap();
    println!("Proving time: {:?}", start.elapsed());
    // Extract journal of receipt
    let output: String = receipt.journal.decode().unwrap();

    // Print, notice, after committing to a journal, the private input became public
    println!("I generated a proof of guest execution!");
    /*for result in &output {
        println!("Device ID: {}, Rewards: {}", result.device_id, result.distance);
    }*/

    (receipt, output)
}


