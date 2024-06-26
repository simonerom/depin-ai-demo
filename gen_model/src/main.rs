extern crate smartcore;
extern crate csv;
extern crate serde;

use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::metrics::accuracy;
use smartcore::neighbors::knn_classifier::KNNClassifier;
use smartcore::metrics::distance::euclidian::Euclidian;

use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use hex;


#[derive(Debug, Deserialize)]
struct Record {
    right_force: f32,
    left_force: f32,
    interval_ms: f32,
    activity: usize,
}

fn main() {
    // Train and save the model    
    save_model(&train_model("data/run_walk.csv").unwrap(), "model/run_walk.hex").unwrap();

    // Load the model
    let knn = load_model("model/run_walk.hex");
    
    // Example data point: [Right_Force, Left_Force, Interval_ms]
    let data_point = vec![92.0, 76.0, 500.0]; // Example features of a new data point

    // Convert the data point to a DenseMatrix as expected by the predict method
    // Note: The data must be in the shape (n_samples, n_features),
    // so for a single sample, it should be (1, n_features).
    let data_matrix = DenseMatrix::from_2d_array(&[&data_point]);

    // Predict the class for the new data point
    let prediction = knn.predict(&data_matrix).expect("Failed to make prediction");
    print!("Example Prediction: {:?} -> ", data_point);
    if prediction[0] == 1 {
        println!("The user is Running");
    } else {
        println!("The user is Walking");
    }
}

fn train_model(file_path: &str) -> Result<KNNClassifier<f32, usize, DenseMatrix<f32>, Vec<usize>, Euclidian<f32>>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(File::open(file_path)?);
    let mut features: Vec<f32> = Vec::new();
    let mut targets: Vec<usize> = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        features.extend_from_slice(&[record.right_force, record.left_force, record.interval_ms]);
        targets.push(record.activity);
    }

    let n_samples = targets.len();
    let x = DenseMatrix::from_2d_array(&features.chunks(3).collect::<Vec<_>>());
    let y = targets;

    println!("\n********* Training  Model **********");

    // Splitting the dataset and training the KNN Classifier are omitted for simplicity
    // Here's how you might initialize a KNNClassifier with k=3 and Euclidean distance
    let knn = KNNClassifier::fit(&x, &y, Default::default())?;

    // Calculate the accuracy of the model
    let prediction = knn.predict(&x).expect("Failed to make prediction");

    println!("{} samples. Accuracy: {}", n_samples, accuracy(&y, &prediction));
    println!("********** Model trained **********");
    Ok(knn)
}

// Save the model to disk
fn save_model(knn: &KNNClassifier<f32, usize, DenseMatrix<f32>, Vec<usize>, Euclidian<f32>>, file_name: &str) -> Result<(), Box<dyn Error>> {
    let knn_bytes = bincode::serialize(&knn).expect("Can not serialize the model");
    let knn_hex = hex::encode(&knn_bytes);

    File::create(file_name)
        .and_then(|mut f| f.write_all(&knn_hex.as_bytes()))
        .expect("Can not persist model");

    println!("\nModel saved to: {}", file_name);
    Ok(())
}

// Load the model
fn load_model(file_name: &str) -> KNNClassifier<f32, usize, DenseMatrix<f32>, Vec<usize>, Euclidian<f32>> {
    let mut hex_string = String::new();

    File::open(&file_name)
        .and_then(|mut f| f.read_to_string(&mut hex_string))
        .expect("Can not load model");
    
    let buf = hex::decode(&hex_string) // Decode the hex string back into bytes
    .expect("Failed to decode hex string");

    let knn: KNNClassifier<f32, usize, DenseMatrix<f32>, Vec<usize>, Euclidian<f32>> = bincode::deserialize(&buf)
        .expect("Cannot deserialize the model");
    
    knn
}