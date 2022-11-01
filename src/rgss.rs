// Copyright (C) 2022 Egor Poleshko
//
// This file is part of rsgss.
//
// rsgss is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rsgss is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rsgss.  If not, see <http://www.gnu.org/licenses/>.

use std::{path::Path, thread};

use magnus::{eval, value::Qnil, RArray, RClass, RModule, RString};
use rfd::MessageDialog;

use crate::binding_util;

pub fn spawn_rgss_thread(app_name: String) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        // Initialize VM
        println!("Initializing Ruby Virtual Machine...");
        let _cleanup = unsafe { magnus::embed::init() };

        // Set RSGSS Global Variable to true
        eval::<bool>("$RSGSS = true\n$MKXP = false").unwrap();

        // Set load path
        eval::<RArray>("$LOAD_PATH.unshift File.dirname(__FILE__) + '/lib'").unwrap();

        // Bind all classes
        binding_util::bind().unwrap();
        eval::<Qnil>("require \"irb\"; IRB.start").unwrap();

        // Load scripts
        if !Path::new("Data/xScripts.rxdata").exists() {
            MessageDialog::new()
                .set_title(&app_name)
                .set_level(rfd::MessageLevel::Error)
                .set_description(
                    "Game Scripts do not exist! (File Not Found: Data/xScripts.rxdata)",
                )
                .set_buttons(rfd::MessageButtons::Ok)
                .show();
            std::process::exit(1);
        }
        // Open Scripts
        let file = RClass::from_value(eval("File").unwrap()).unwrap();
        let _file_result: RString = file.funcall("binread", ("Data/xScripts.rxdata",)).unwrap();

        // Unmarshal them
        let _marshal = RModule::from_value(eval("Marshal").unwrap()).unwrap();

        //let array: Vec<_> = marshal.funcall::<&str, (RString,), RArray>("load", (file_result,)).unwrap().to_vec::<RArray>().unwrap();
    })
}
