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

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use gtk::{
    Button,
    ButtonExt,
    ContainerExt,
    Inhibit,
    Label,
    WidgetExt,
    Window,
    WindowType,
};
use gtk::Orientation::Vertical;
use relm::{RemoteRelm, Widget};

#[derive(Clone)]
struct Model {
    counter: i32,
}

#[derive(Msg)]
enum Msg {
    Decrement,
    Increment,
    Quit,
}

// Create the structure that holds the widgets used in the view.
#[derive(Clone)]
struct Win {
    counter_label: Label,
    window: Window,
}

impl Widget for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = i32;
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;
    // Specify the type of the root widget.
    type Root = Window;

    fn model(counter: i32) -> Model {
        Model {
            counter: counter,
        }
    }

    // Return the root widget.
    fn root(&self) -> &Self::Root {
        &self.window
    }

    fn update(&mut self, event: Msg, model: &mut Model) {
        let label = &self.counter_label;

        match event {
            Msg::Decrement => {
                model.counter -= 1;
                // Manually update the view.
                label.set_text(&model.counter.to_string());
            },
            Msg::Increment => {
                model.counter += 1;
                label.set_text(&model.counter.to_string());
            },
            Msg::Quit => gtk::main_quit(),
        }
    }

    fn view(relm: &RemoteRelm<Self>, model: &Self::Model) -> Self {
        // Create the view using the normal GTK+ method calls.
        let vbox = gtk::Box::new(Vertical, 0);

        let plus_button = Button::new_with_label("+");
        vbox.add(&plus_button);

        let counter_label = Label::new(model.counter.to_string().as_ref());
        vbox.add(&counter_label);

        let minus_button = Button::new_with_label("-");
        vbox.add(&minus_button);

        let window = Window::new(WindowType::Toplevel);

        window.add(&vbox);

        window.show_all();

        // Send the message Increment when the button is clicked.
        connect!(relm, plus_button, connect_clicked(_), Msg::Increment);
        connect!(relm, minus_button, connect_clicked(_), Msg::Decrement);
        connect!(relm, window, connect_delete_event(_, _) (Some(Msg::Quit), Inhibit(false)));

        Win {
            counter_label: counter_label,
            window: window,
        }
    }
}

fn main() {
    Win::run(42).unwrap();
}
