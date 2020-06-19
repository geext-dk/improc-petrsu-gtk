// main.rs - The main entrypoint to the program
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

#![windows_subsystem = "windows"]
mod gtk_ui;
mod cli;

use std::env;

fn main() {
    env_logger::init();
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        gtk_ui::run_ui();
    } else {
        cli::run_ui();
    }
}
