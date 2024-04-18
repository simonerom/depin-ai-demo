use serde::{Serialize, Deserialize};

// Define the amount of messages to be aggregated in a task.
// Only usedto generate test data.
pub const AGGREGATION_AMOUNT: usize = 10;
// Number of tokens to be distributed just for message contribution.
// i.e., for "proof of online" participation.
pub const BASE_REWARD: u64 = 1;
// Number of tokens to be distributed for each connection.
// i.e., for "proof of work" participation.
pub const REWARD_PER_CONNECTION: u64 = 1;

// This is the struct that represents the W3bstream task that will be sent to the prover.
#[derive(Serialize, Deserialize, Debug)]
pub struct WSTask {
    pub project_id: u64,
    pub task_id: u64,
    pub client_id: String,
    pub sequencer_sign: String,
    pub datas: Vec<String>
}

// This is how the device message looks like.
#[derive(Serialize, Deserialize)]
pub struct DeviceMessage {
    pub device_id: u64,
    pub right_force: f64,
    pub left_force: f64,
    pub interval_ms: f64,
}


