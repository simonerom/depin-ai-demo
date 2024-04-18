#![no_main]

use dewi_core::*;
use risc0_zkvm::guest::env;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::error::Error;

use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::neighbors::knn_classifier::KNNClassifier;
use smartcore::metrics::distance::euclidian::Euclidian;
use hex;
use bincode;

risc0_zkvm::guest::entry!(main);

const MODEL_HEX_STR: &str = include_str!("../../../gen_model/model/run_walk.hex");

fn main() {
    // Get the knn model ready
    let model = deserialize_knn_model().expect("Error while deserializing the model");
    
    let messages = get_task_messages();

    // Init the public output: a hashmap where we store rewards due to each device
    let mut rewards_map = HashMap::new();

    // Prepare the AI test vector
    let mut x: Vec<Vec<f32>> = Vec::new();

    for message in messages.iter() {
        x.push(vec![message.right_force as f32, message.left_force as f32, message.interval_ms as f32]);
    }

    // Classify the result behavior
    let y = predict(&model, &x);

    // Calculate the rewards
    for (message, y) in messages.iter().zip(y.iter()) {
        let reward = rewards_map.entry(message.device_id).or_insert(0 as u64);
        if *y == 1 { *reward = *reward + BASE_REWARD; };
    }
    
    // Serialize the result array and send it back to the host
    let stringyfied_res = serde_json::to_string(&rewards_map).unwrap();

    env::commit(&stringyfied_res);
    env::log(&format!("Guest execution completed on {} messages.", messages.len()));
    env::log(&format!("Public output from Journal is: {}", stringyfied_res).as_str());
    env::log(&format!("Proving starts now, please be patient!"));
}

fn get_task_messages() -> Vec<DeviceMessage> {
    env::log(&format!("Processing a Task"));

    // Extract task metadata first
    let project_id: u64 = env::read();
    env::log(&format!("project_id {}", project_id));
    let task_id: u64 = env::read();
    env::log(&format!("task_id {}", task_id));
    let client_id: String = env::read();
    env::log(&format!("client_id {}", client_id));
    let sequencer_sign: String = env::read();
    env::log(&format!("sequencer_sign {}", sequencer_sign));

    // Extract the array of messages
    let datas: Vec<String> = env::read();
    env::log(&format!("Messages {:?}", datas));

    // Parse into an array of DataMessage
    let mut messages = Vec::new();
    for data in datas.iter() {
        let v: JsonValue = serde_json::from_str(data).unwrap();
        let device_id: u64 = v["device_id"].as_u64().unwrap();
        let right_force: f64 = v["right_force"].as_f64().unwrap();
        let left_force: f64 = v["left_force"].as_f64().unwrap();
        let interval_ms: f64 = v["interval_ms"].as_f64().unwrap();
        messages.push(DeviceMessage { device_id, right_force, left_force, interval_ms});
    } 
    messages
}

fn deserialize_knn_model() -> Result<KNNClassifier<f32, usize, DenseMatrix<f32>, Vec<usize>, Euclidian<f32>>, Box<dyn Error>> {
    // Decode the hex string to bytes
    let model_bytes = hex::decode(&MODEL_HEX_STR)
    .expect("Error while decoding the model hex string");

    // Deserialize the bytes into the KNNClassifier model
    let model: KNNClassifier<f32, usize, DenseMatrix<f32>, Vec<usize>, Euclidian<f32>> = bincode::deserialize(&model_bytes)
    .expect("Error while deserializing the model bytes");

    Ok(model)
}

fn predict(model: &KNNClassifier<f32, usize, DenseMatrix<f32>, Vec<usize>, Euclidian<f32>>, x: &Vec<Vec<f32>>) -> Vec<usize> {
    let y = model.predict(&DenseMatrix::from_2d_vec(x)).unwrap();
    
    // Log the prediction
    // println!("Prediction for {:?} is {:?}", x, y);
    
    y
}