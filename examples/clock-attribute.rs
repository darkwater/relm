/*
 * Copyright (c) 2017 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#![feature(fn_traits, proc_macro, unboxed_closures)]

extern crate chrono;
extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;
extern crate tokio_core;

use std::time::Duration;

use chrono::{DateTime, Local};
use gtk::{
    Inhibit,
    WidgetExt,
};
use relm::{Relm, Widget};
use relm_attributes::widget;
use tokio_core::reactor::Interval;

use self::Msg::*;

#[derive(Clone)]
pub struct Model {
    time: DateTime<Local>,
}

#[derive(SimpleMsg)]
pub enum Msg {
    Quit,
    Tick,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {
            time: Local::now(),
        }
    }

    fn subscriptions(relm: &Relm<Msg>) {
        let stream = Interval::new(Duration::from_secs(1), relm.handle()).unwrap();
        relm.connect_exec_ignore_err(stream, Tick);
    }

    fn update(&mut self, event: Msg, model: &mut Model) {
        match event {
            Tick => model.time = Local::now(),
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Label {
                text: &model.time.format("%H:%M:%S").to_string(),
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
