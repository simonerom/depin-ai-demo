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

use dewi_core::*;
use serde_json;
use csv::Reader;
use dewi_demo::process_task;
use dewi_demo_methods::DEWI_ID;
use std::error::Error;

fn main() {
    let messages = create_messages_from_csv("test_data.csv").unwrap();

    // let messages_str = serde_json::to_string(&messages).unwrap();
    // env::log("Processing task messages:\n{}", messages_str);

    // Create a task struct
    let task = WSTask {
        project_id: 10002,
        task_id: 1,
        client_id: "client_id".to_string(),
        sequencer_sign: "0x0".to_string(),
        datas: messages,
    };

    // env::log(&format("W3bstream Task: {:?}", task));
    let (receipt, _) = process_task(task);

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify(DEWI_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}

fn create_messages_from_csv(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut messages: Vec<String> = Vec::new();

    // Open the file
    let mut rdr = Reader::from_path(file_path)?;

    // Iterate over each record
    for result in rdr.deserialize() {
        let message: DeviceMessage = result?;
        
        // Serialize the message to a JSON string
        let serstr = serde_json::to_string(&message)?;
        // env::log("Message: {}", serstr);
        messages.push(serstr);
    }

    Ok(messages)
}
