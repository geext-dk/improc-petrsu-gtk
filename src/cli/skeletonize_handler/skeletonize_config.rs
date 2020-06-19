// skeletonize_config.rs - Encapsulates settings for skeletonization
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

use clap::{ ArgMatches };
use improc_petrsu::AdjacencyMode;


#[derive(Clone, Copy)]
pub enum SkeletonizationAlgorithm {
    Eberly,
    Rosenfeld,
    ZhangSuen
}

pub struct SkeletonizeConfig<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
    pub algorithm: SkeletonizationAlgorithm, 
    pub adjacency_mode: Option<AdjacencyMode>
}

impl<'a> SkeletonizeConfig<'a> {
    pub fn new(matches: &'a ArgMatches)-> Self {
        let input_file = matches.value_of("input-file").unwrap();
        let output_file = matches.value_of("output-file").unwrap();
        let algorithm = match matches.value_of("algorithm").unwrap() {
            "Eberly" => SkeletonizationAlgorithm::Eberly,
            "Rosenfeld" => SkeletonizationAlgorithm::Rosenfeld,
            "ZhangSuen" => SkeletonizationAlgorithm::ZhangSuen,
            _ => panic!("Unknown algorithm")
        };
        let adjacency_mode = match matches.value_of("adjacency-mode") {
            Some(arg) => match arg {
                "Four" => Some(AdjacencyMode::Four),
                "Eight" => Some(AdjacencyMode::Eight),
                _ => panic!("Unknown adjacency mode")
            },
            None => None
        };

        SkeletonizeConfig {
            input_file,    // Todo: use &str and lifetime instead
            output_file, 
            algorithm,
            adjacency_mode
        }
    }
}
