// convert_to_binary_config.rs - Encapsulates settings for the converting
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

pub struct ConvertToBinaryConfig<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
    pub threshold: u32
}

impl<'a> ConvertToBinaryConfig<'a> {
    pub fn new(matches: &'a ArgMatches)-> Self {
        let input_file = matches.value_of("input-file").unwrap();
        let output_file = matches.value_of("output-file").unwrap();
        let threshold = match matches.value_of("threshold").unwrap().parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Error parsing the value of the 'threshold' parameter");
                eprintln!("Using the default value of 150");
                150u32
            }
        };

        ConvertToBinaryConfig {
            input_file,
            output_file,
            threshold
        }
    }
}
