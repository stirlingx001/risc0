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

use risc0_zkvm::guest::env;
use image;
risc0_zkvm::guest::entry!(main);

fn main() {
   // Load the first number from the host
   let jpeg_data: Vec<u8> = env::read();
   // Load the second number from the host
   let pixels: Vec<u8> = env::read();
    // Verify that neither of them are 1 (i.e. nontrivial factors)
    let img = image::load_from_memory(&jpeg_data).unwrap();
    let rgb_img = img.to_rgb8();
    let rgb: Vec<u8> = rgb_img.clone().into_raw();

    let eq = rgb == pixels;
    env::commit(&eq);
}

