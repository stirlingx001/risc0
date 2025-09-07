// Copyright 2025 RISC Zero, Inc.
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

use jpeg_methods::DECODE_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use image;

// This is a Hello World demo for the RISC Zero zkVM.
// By running the demo, Alice can produce a receipt that proves that she knows
// some numbers a and b, such that a*b == 391.
// The factors a and b are kept secret.

// Compute the product a*b inside the zkVM
pub fn decode(data: Vec<u8>) -> (Receipt, bool) {

    let rgb_img = decode_jpeg_to_rgb(data.clone()).unwrap();
    let pixels: Vec<u8> = rgb_img.clone().into_raw();
    println!("Image size: {} x {}", rgb_img.width(), rgb_img.height());
    println!("RGB pixels length: {}", pixels.len());

    let env = ExecutorEnv::builder()
        .write(&data)
        .unwrap()
        .write(&pixels)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, DECODE_ELF).unwrap().receipt;

    let c: bool = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    // Report the product
    println!("returns {}", c);

    (receipt, c)
}

fn decode_jpeg_to_rgb(jpeg_data: Vec<u8>) -> Result<image::RgbImage, Box<dyn std::error::Error>> {
    let img = image::load_from_memory(&jpeg_data)?;
    let rgb_img = img.to_rgb8();
    Ok(rgb_img)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        // const TEST_FACTOR_ONE: u64 = 17;
        // const TEST_FACTOR_TWO: u64 = 23;
        // let (_, result) = multiply(17, 23);
        // assert_eq!(
        //     result,
        //     TEST_FACTOR_ONE * TEST_FACTOR_TWO,
        //     "We expect the zkVM output to be the product of the inputs"
        // )
    }
}
