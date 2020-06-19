// app_state.rs - Struct with application widgets and state
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

use gtk::prelude::*;
use gtk::{
    ApplicationWindow, Builder, Button, ComboBoxText, Dialog, FileChooserButton, Image, 
    Notebook, SpinButton, MessageDialog,
};
use std::cell::{Ref, RefCell};

pub struct AppState {
    // widgets
    pub main_window: ApplicationWindow,
    pub main_notebook: Notebook,
    pub file_chooser_button: FileChooserButton,
    pub image_view: Image,
    pub convert_to_binary_button: Button,
    pub skeletonize_button: Button,
    pub undo_button: Button,
    pub save_button: Button,

    // dialogs
    pub skeletonize_dialog: Dialog,
    pub convert_to_binary_dialog: Dialog,
    pub error_dialog: MessageDialog,

    pub threshold_spin_button: SpinButton,
    pub skeletonize_algorithm_combo_box: ComboBoxText,

    // data
    image_bytes: RefCell<Vec<u8>>,
    snapshots: RefCell<Vec<ImageSnapshot>>,
}

pub struct ImageSnapshot {
    pub bytes: Vec<u8>,
    pub description: String,
}

impl AppState {
    pub fn new_from_builder(builder: &Builder) -> Option<Self> {
        let main_window: ApplicationWindow = builder.get_object("MainWindow")?;
        let main_notebook: Notebook = builder.get_object("MainNotebook")?;
        let file_chooser_button: FileChooserButton = builder.get_object("FileChooserButton")?;
        let image_view: Image = builder.get_object("ImageView")?;
        let skeletonize_button: Button = builder.get_object("SkeletonizeButton")?;
        let convert_to_binary_button: Button = builder.get_object("ConvertToBinaryButton")?;
        let error_dialog: MessageDialog = builder.get_object("ErrorDialog")?;
        let undo_button: Button = builder.get_object("UndoButton")?;
        let save_button: Button = builder.get_object("SaveButton")?;

        let skeletonize_dialog: Dialog = builder.get_object("SkeletonizeDialog")?;
        let convert_to_binary_dialog: Dialog = builder.get_object("ConvertToBinaryDialog")?;

        let threshold_spin_button: SpinButton = builder.get_object("ThresholdSpinButton")?;
        let skeletonize_algorithm_combo_box = builder.get_object("SkeletonizeAlgorithmComboBox")?;

        Some(AppState {
            main_window,
            main_notebook,
            file_chooser_button,
            image_view,
            skeletonize_button,
            convert_to_binary_button,
            error_dialog,
            undo_button,
            save_button,

            skeletonize_dialog,
            convert_to_binary_dialog,
            threshold_spin_button,
            skeletonize_algorithm_combo_box,

            image_bytes: RefCell::new(Vec::new()),
            snapshots: RefCell::new(Vec::new()),
        })
    }

    pub fn get_latest_image(&self) -> Ref<'_, Vec<u8>> {
        if self.snapshots.borrow().len() > 0 {
            Ref::map(self.snapshots.borrow(), |snaps| {
                &snaps.last().unwrap().bytes
            })
        } else {
            self.image_bytes.borrow()
        }
    }

    pub fn push_snapshot(&self, description: String, bytes: Vec<u8>) {
        self.snapshots
            .borrow_mut()
            .push(ImageSnapshot { bytes, description });
    }
    
    pub fn pop_snapshot(&self) -> Option<ImageSnapshot> {
        self.snapshots.borrow_mut().pop()
    }

    pub fn set_original_image(&self, bytes: Vec<u8>) {
        self.snapshots.borrow_mut().clear();
        self.image_bytes.replace(bytes);
    }
}
