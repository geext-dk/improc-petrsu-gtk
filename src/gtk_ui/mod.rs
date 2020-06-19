// gtk_ui/mod.rs - Module with GTK ui code
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

mod app_state;

use app_state::AppState;
use gdk_pixbuf::Pixbuf;
use gio::prelude::*;
use gio::{Cancellable, MemoryInputStream};
use glib::Bytes;
use gtk::prelude::*;
use gtk::{Application, Builder, FileChooserExt, NotebookExt, ResponseType, WidgetExt,
    FileChooserNative, FileChooserAction};
use image::{DynamicImage, ImageOutputFormat};
use improc_petrsu::{
    AdjacencyMode, BinaryImage, BinaryImageConverter, EberlySkeletonizer, PixelColor,
    RosenfeldSkeletonizer, Skeletonizer, ThresholdBinaryImageConverter, ZhangSuenSkeletonizer,
};
use std::env;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use log::{info};

fn open_error_dialog<S: AsRef<str>>(app_state: Rc<AppState>, message: S) {
    app_state.error_dialog.set_property_text(Some(message.as_ref()));
    app_state.error_dialog.run();
    app_state.error_dialog.hide();
}

fn update_image(app_state: Rc<AppState>) {
    info!("Updating the GtkImage");
    let bytes = Bytes::from(&*app_state.get_latest_image());
    let stream = MemoryInputStream::new_from_bytes(&bytes);
    let cancellable = Cancellable::new();
    match Pixbuf::new_from_stream_at_scale(&stream, -1, 400, true, Some(&cancellable)) {
        Ok(p) => {
            app_state.image_view.set_from_pixbuf(Some(&p));
        }
        Err(_) => open_error_dialog(app_state, "Error converting the stream to pixbuf")
    };
}

fn save_image<P: AsRef<Path>>(app_state: Rc<AppState>, path: P) {
    info!("Saving the image in: {}", path.as_ref().to_string_lossy());
    let bytes = app_state.get_latest_image();
    let mut file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path) {
        Ok(f) => f,
        Err(_) => {
            open_error_dialog(app_state.clone(), "Unable to open the file");
            return;
        }
    };

    if file.write_all(&*bytes).is_err() {
        open_error_dialog(app_state.clone(), "Couldn't write the image to the file");
        return;
    }

    if file.sync_all().is_err() {
        open_error_dialog(app_state.clone(), "Couldn't flush the buffers!");
        return;
    }
}

// On file set we should open the image, display it in the GtkImage
// and proceed to the next page
fn file_set_handler(app_state: Rc<AppState>) {
    let filename = match app_state
        .file_chooser_button
        .get_filename() {
        Some(f) => f,
        None => {
            open_error_dialog(app_state, "You didn't choose a file!");
            return;
        }
    };

    info!("Opening the file in: {}", filename.to_string_lossy());
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => {
            open_error_dialog(app_state, "Unable to open the file");
            return;
        }
    };

    let mut buf = Vec::new();
    if file.read_to_end(&mut buf).is_err() {
        open_error_dialog(app_state, "Unable to read the file");
        return;
    }

    app_state.set_original_image(buf);

    update_image(app_state.clone());

    app_state.main_notebook.next_page();
}

// when the 'convert to binary' button clicked we should display a modal window
// waiting for the user to specify threshold, and
fn convert_to_binary_handler(app_state: Rc<AppState>) {
    app_state.threshold_spin_button.set_value(125 as f64);
    if app_state.convert_to_binary_dialog.run() == ResponseType::Ok {
        let threshold = app_state.threshold_spin_button.get_value();
        if threshold < 0.0 || threshold > 255.0 {
            panic!("threshold must be between 0 and 255 inclusive!");
        }

        info!("Loading image into memory...");
        let mut image = match image::load_from_memory(&app_state.get_latest_image()) {
            Ok(img) => img.to_rgb(),
            Err(_) => {
                open_error_dialog(app_state.clone(), "Couldn't read the image bytes");
                return;
            }
        };

        let converter = ThresholdBinaryImageConverter::new(threshold as u32);

        info!("Converting the image to binary...");
        converter.convert_to_binary(&mut image);
        let dyn_image = DynamicImage::ImageRgb8(image);
        let mut vector = Vec::new();

        info!("Writing the result as byte array...");
        if dyn_image
            .write_to(&mut vector, ImageOutputFormat::Png)
            .is_err() {
            open_error_dialog(app_state, "Couldn't write the image as PNG");
            return;
        }

        app_state.push_snapshot(
            format!("Converted to binary with threshold: {}", threshold),
            vector,
        );

        update_image(app_state.clone());
    };

    app_state.convert_to_binary_dialog.hide();
}

fn skeletonize_handler(app_state: Rc<AppState>) {
    app_state
        .skeletonize_algorithm_combo_box
        .set_active_id(Some("0"));

    if app_state.skeletonize_dialog.run() == ResponseType::Ok {
        let algorithm = app_state
            .skeletonize_algorithm_combo_box
            .get_active_id()
            .unwrap();

        let algorithm: u8 = (&algorithm).parse().unwrap();
        let skeletonizer = match algorithm {
            0 => {
                Box::new(RosenfeldSkeletonizer::new(AdjacencyMode::Eight)) as Box<dyn Skeletonizer>
            }
            1 => Box::new(EberlySkeletonizer::new()) as Box<dyn Skeletonizer>,
            2 => Box::new(ZhangSuenSkeletonizer::new()) as Box<dyn Skeletonizer>,
            _ => panic!("Unknow skeletonizer value!"),
        };

        info!("Loading image into memory...");
        let image = match image::load_from_memory(&app_state.get_latest_image()) {
            Ok(img) => img.to_rgb(),
            Err(_) => {
                open_error_dialog(app_state.clone(), "Couldn't read the image bytes");
                return;
            }
        };

        info!("Converting the image to binary...");
        let mut binary_image = BinaryImage::from_image(&image, PixelColor::White);

        info!("Skeletonizing the image...");
        skeletonizer.process(&mut binary_image);

        info!("Converting the image to RGB...");
        let ret = binary_image.to_rgb_image();
        let dyn_image = DynamicImage::ImageRgb8(ret);
        let mut vector = Vec::new();

        info!("Writing the image as byte array...");
        if dyn_image
            .write_to(&mut vector, ImageOutputFormat::Png)
            .is_err() {
            open_error_dialog(app_state, "Couldn't write the image as PNG");
            return;
        }

        let algorithm_str = match algorithm {
            0 => "Rosenfeld",
            1 => "Eberly",
            2 => "Zhang Suen",
            _ => panic!("Unknown skeletonizer value!"),
        };

        app_state.push_snapshot(
            format!(
                "Skeletonized the image with the {} algorithm",
                algorithm_str
            ),
            vector,
        );

        update_image(app_state.clone());
    }

    app_state.skeletonize_dialog.hide();
}

fn undo_handler(app_state: Rc<AppState>) {
    match app_state.pop_snapshot() {
        Some(_) => {
            info!("Undo");
            update_image(app_state);
        }, 
        None => info!("Nothing to undo")
    };
}

fn save_handler(app_state: Rc<AppState>) {
    let file_chooser = FileChooserNative::new(Some("Save the image"),
        Some(&app_state.main_window),
        FileChooserAction::Save,
        None,
        None);

    file_chooser.connect_response(move |chooser, response| {
        match response {
            ResponseType::Accept => {
                save_image(app_state.clone(), chooser.get_filename().unwrap());
            },
            _ => ()
        };
    });

    file_chooser.run();
}

// connect signals, show ui
fn build_ui(application: &gtk::Application, app_state: Rc<AppState>) {
    app_state.main_window.set_application(Some(application));

    let app_state_cloned = app_state.clone();
    app_state.file_chooser_button.connect_file_set(move |_| {
        file_set_handler(app_state_cloned.clone());
    });

    let app_state_cloned = app_state.clone();
    app_state
        .convert_to_binary_button
        .connect_clicked(move |_| {
            convert_to_binary_handler(app_state_cloned.clone());
        });

    let app_state_cloned = app_state.clone();
    app_state.skeletonize_button.connect_clicked(move |_| {
        skeletonize_handler(app_state_cloned.clone());
    });
    
    let app_state_cloned = app_state.clone();
    app_state.undo_button.connect_clicked(move |_| {
        undo_handler(app_state_cloned.clone());
    });

    let app_state_cloned = app_state.clone();
    app_state.save_button.connect_clicked(move |_| {
        save_handler(app_state_cloned.clone());
    });

    app_state.main_window.show_all();
}

pub fn run_ui() {
    let application = Application::new(Some("ru.petrsu.improc-gtk"), Default::default())
        .expect("failed to initialize GTK application");

    let glade_src = include_str!("ui.glade");
    let builder = Builder::new_from_string(glade_src);

    let app_state = match AppState::new_from_builder(&builder) {
        Some(state) => state,
        None => {
            std::process::exit(1);
        }
    };

    let app_state = Rc::new(app_state);

    application.connect_activate(move |app| {
        build_ui(app, app_state.clone());
    });

    application.run(&env::args().collect::<Vec<_>>());
}
