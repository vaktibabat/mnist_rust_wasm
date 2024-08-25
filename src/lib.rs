use ndarray::{Array1, Array2};
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

const IMG_WIDTH: usize = 28;
const IMG_HEIGHT: usize = 28;
const RGBA: usize = 4;
const NUM_LAYERS: usize = 3;
const NUM_DIGITS: usize = 10;
const LAYER_1: usize = 500;
const LAYER_2: usize = 300;
const LAYER_3: usize = 100;
const INPUT_SIZE: usize = 784;
const THRESHOLD: u8 = 127;
const MAX_PIXEL_VAL: f64 = 255f64;

#[derive(Serialize, Deserialize)]
pub struct NeuralNetJs {
    pub W0: Vec<f64>,
    pub b0: Vec<f64>,
    pub W1: Vec<f64>,
    pub b1: Vec<f64>,
    pub W2: Vec<f64>,
    pub b2: Vec<f64>,
    pub W3: Vec<f64>,
    pub b3: Vec<f64>,
}

pub struct NeuralNet {
    pub layers: Vec<(Array2<f64>, Array1<f64>)>,
}

impl From<NeuralNetJs> for NeuralNet {
    fn from(value: NeuralNetJs) -> Self {
        let w0 = Array2::from_shape_vec((INPUT_SIZE, LAYER_1), value.W0).unwrap();
        let b0 = Array1::from_vec(value.b0);
        let w1 = Array2::from_shape_vec((LAYER_1, LAYER_2), value.W1).unwrap();
        let b1 = Array1::from_vec(value.b1);
        let w2 = Array2::from_shape_vec((LAYER_2, LAYER_3), value.W2).unwrap();
        let b2 = Array1::from_vec(value.b2);
        let w3 = Array2::from_shape_vec((LAYER_3, NUM_DIGITS), value.W3).unwrap();
        let b3 = Array1::from_vec(value.b3);
        let layers = vec![(w0, b0), (w1, b1), (w2, b2), (w3, b3)];

        return NeuralNet { layers }
    }
}

impl NeuralNet {
    pub fn forward(&self, x: Array2<f64>) -> Array2<f64> {
        let mut out = x;

        for (i, (weight, bias)) in self.layers.iter().enumerate() {
            if i != NUM_LAYERS {
                // Apply ReLU
                out = (out.dot(weight) + bias).map(|z| z.max(0f64));
            } else {
                out = out.dot(weight) + bias;
            }
        }

        out
    }
}

#[wasm_bindgen]
pub async fn predict(data: &[u8], model_js: JsValue) -> Vec<f64> {
    let model_js: NeuralNetJs = serde_wasm_bindgen::from_value(model_js).unwrap();
    let model = NeuralNet::from(model_js);
    let inp_data: Vec<f64> = get_pixels(data).into_iter().map(|x| 1f64 - (x / MAX_PIXEL_VAL)).collect();
    let model_input = Array2::from_shape_vec((1, IMG_WIDTH*IMG_HEIGHT), inp_data).unwrap();
    let preds = model.forward(model_input);

    softmax(preds.flatten().to_vec())
}

/// Softmax function - Convert scores into a probability distribution
fn softmax(scores: Vec<f64>) -> Vec<f64> {
    let max = scores.iter().max_by(|x, y| x.total_cmp(y)).unwrap();
    // We use a numerical trick where we shift the elements by the max, because otherwise
    // We would have to compute the exp of very large values which wraps to NaN
    let shift_scores: Vec<f64> = scores.iter().map(|x| x - max).collect();
    let sum: f64 = shift_scores.iter().map(|x| x.exp()).sum();

    (0..scores.len())
        .map(|x| shift_scores[x].exp() / sum)
        .collect()
}

// Initially we get the data as an (28, 28, 4) RGBA tensor, so we need to convert it to greyscale
fn get_pixels(data: &[u8]) -> [f64; IMG_WIDTH * IMG_HEIGHT] {
    let mut tensor = [0f64; IMG_WIDTH * IMG_HEIGHT];

    for i in 0..IMG_WIDTH*IMG_HEIGHT {
        let r = data[i * RGBA];

        // The canvas is black and weight, so we only care about the value of one of the RGB components, which is
        // either 0 or 255
        tensor[i] = if r >= THRESHOLD {MAX_PIXEL_VAL} else {0f64};
    }

    tensor
}