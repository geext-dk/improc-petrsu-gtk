// skeletonize_handler/mod.rs - Handles the skeletonization
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

pub mod skeletonize_config;

use improc_petrsu::{
    Skeletonizer,
    ZhangSuenSkeletonizer,
    EberlySkeletonizer,
    RosenfeldSkeletonizer,
    BinaryImage,
    PixelColor
};

pub use skeletonize_config::SkeletonizeConfig;
pub use skeletonize_config::SkeletonizationAlgorithm;

pub struct SkeletonizeHandler {

}

impl SkeletonizeHandler {
    pub fn execute(config: &SkeletonizeConfig) {

        let skeletonizer: Box<dyn Skeletonizer> = match config.algorithm {
            SkeletonizationAlgorithm::ZhangSuen => Box::new(ZhangSuenSkeletonizer::new()),
            SkeletonizationAlgorithm::Eberly => Box::new(EberlySkeletonizer::new()),
            SkeletonizationAlgorithm::Rosenfeld => Box::new(RosenfeldSkeletonizer::new(config.adjacency_mode.unwrap()))
        };

        println!("Opening the image...");
        let image = image::open(config.input_file).unwrap_or_else(|err| {
            eprintln!("Error opening image: {}", err);
            std::process::exit(1);
        }).to_rgb();

        println!("Converting the image to binary...");
        let mut binary_image = BinaryImage::from_image(&image, PixelColor::White);

        println!("Skeletonization...");
        skeletonizer.process(&mut binary_image);

        println!("Converting to rgb...");
        let result_image = binary_image.to_rgb_image();

        println!("Saving...");
        result_image.save(config.output_file).unwrap_or_else(|err| {
            eprintln!("Failed to save the resulting image: {}", err);
            std::process::exit(1);
        });

        println!("Done.");
    }
}
