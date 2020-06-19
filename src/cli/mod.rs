// main.rs - The main entrypoint to the CLI
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

mod skeletonize_handler;
mod convert_to_binary_handler;

use skeletonize_handler::{ SkeletonizeHandler, SkeletonizeConfig };
use convert_to_binary_handler::{ ConvertToBinaryConfig, ConvertToBinaryHandler };
use clap::{ load_yaml, App };

pub fn run_ui() {
    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml).get_matches();

    if let Some(matches) = args.subcommand_matches("skeletonize") {
        let config = SkeletonizeConfig::new(matches);
        SkeletonizeHandler::execute(&config);
    } else if let Some(matches) = args.subcommand_matches("convert-to-binary") {
        let config = ConvertToBinaryConfig::new(matches);
        ConvertToBinaryHandler::execute(&config);
    }
}
