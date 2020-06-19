// convert_to_binary_handler/mod.rs - Handles converting to a binary image
// Copyright (C) 2019 Denis Karpovskiy
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod convert_to_binary_config;

use improc_petrsu::{
    BinaryImageConverter,
    ThresholdBinaryImageConverter
};

pub use convert_to_binary_config::ConvertToBinaryConfig;

pub struct ConvertToBinaryHandler {

}

impl ConvertToBinaryHandler {
    pub fn execute(config: &ConvertToBinaryConfig) {
        let converter = ThresholdBinaryImageConverter::new(config.threshold);

        println!("Opening the image...");
        let mut image = image::open(config.input_file).unwrap_or_else(|err| {
            eprintln!("Error opening image: {}", err);
            std::process::exit(1);
        }).to_rgb();

        println!("Converting the image to binary...");
        converter.convert_to_binary(&mut image);

        println!("Saving...");
        image.save(config.output_file).unwrap_or_else(|err| {
            eprintln!("Failed to save the resulting image: {}", err);
            std::process::exit(1);
        });

        println!("Done.");
    }
}
