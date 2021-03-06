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

#![feature(proc_macro)]

extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;
#[macro_use]
extern crate relm_test;

use gtk::{
    ButtonExt,
    Inhibit,
    OrientableExt,
    WidgetExt,
};
use gtk::Orientation::Vertical;
use relm::Widget;
use relm_attributes::widget;

use self::Msg::*;

#[derive(Clone)]
pub struct Model {
    counter: i32,
}

#[derive(Msg)]
pub enum Msg {
    Decrement,
    Increment,
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {
            counter: 0,
        }
    }

    fn update(&mut self, event: Msg, model: &mut Model) {
        match event {
            Decrement => model.counter -= 1,
            Increment => model.counter += 1,
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                #[name="inc_button"]
                gtk::Button {
                    clicked => Increment,
                    label: "+",
                },
                #[name="label"]
                gtk::Label {
                    text: &model.counter.to_string(),
                },
                #[name="dec_button"]
                gtk::Button {
                    clicked => Decrement,
                    label: "-",
                },
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

#[cfg(test)]
mod tests {
    use relm;
    use relm_test::click;

    use super::Win;

    #[test]
    fn label_change() {
        let component = relm::init_test::<Win>(()).unwrap();
        let widgets = component.widget();

        assert_text!(widgets.label, 0);
        click(&widgets.inc_button);
        assert_text!(widgets.label, 1);
        click(&widgets.inc_button);
        assert_text!(widgets.label, 2);
        click(&widgets.dec_button);
        assert_text!(widgets.label, 1);
        click(&widgets.dec_button);
        assert_text!(widgets.label, 0);
        click(&widgets.dec_button);
        assert_text!(widgets.label, -1);
    }
}
