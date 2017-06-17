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
#[macro_use]
extern crate relm_derive;

use gtk::{
    ContainerExt,
    EventBox,
    Inhibit,
    Label,
    Window,
    WindowType,
    WidgetExt,
};
use gtk::Orientation::Vertical;
use relm::{
    Component,
    Container,
    ContainerComponent,
    ContainerWidget,
    Relm,
    Update,
    Widget,
    create_container,
};

use self::Msg::*;

struct Button {
    button: gtk::Button,
}

impl Update for Button {
    type Model = ();
    type ModelParam = ();
    type Msg = ();

    fn model(_: &Relm<Self>, _: ()) -> () {
    }

    fn update(&mut self, _msg: ()) {
    }
}

impl Widget for Button {
    type Root = gtk::Button;

    fn root(&self) -> gtk::Button {
        self.button.clone()
    }

    fn view(_relm: &Relm<Self>, _model: Self::Model) -> Self {
        let button = gtk::Button::new_with_label("+");
        Button {
            button: button,
        }
    }
}

struct VBox {
    event_box: EventBox,
    vbox: gtk::Box,
}

impl Container for VBox {
    type Container = gtk::Box;
    type Containers = ();

    fn container(&self) -> &Self::Container {
        &self.vbox
    }

    fn other_containers(&self) -> () {
    }
}

impl Update for VBox {
    type Model = ();
    type ModelParam = ();
    type Msg = ();

    fn model(_: &Relm<Self>, _: ()) -> () {
        ()
    }

    fn update(&mut self, _event: ()) {
    }
}

impl Widget for VBox {
    type Root = EventBox;

    fn root(&self) -> EventBox {
        self.event_box.clone()
    }

    fn view(_relm: &Relm<Self>, _model: Self::Model) -> Self {
        let event_box = EventBox::new();
        let vbox = gtk::Box::new(Vertical, 0);
        event_box.add(&vbox);

        VBox {
            event_box: event_box,
            vbox: vbox,
        }
    }
}

struct MyVBox {
    vbox: ContainerComponent<VBox>,
    _widget: Component<Button>,
}

impl Update for MyVBox {
    type Model = ();
    type ModelParam = ();
    type Msg = ();

    fn model(_: &Relm<Self>, _: ()) -> () {
    }

    fn update(&mut self, _event: ()) {
    }
}

impl Widget for MyVBox {
    type Root = <VBox as Widget>::Root;

    fn root(&self) -> EventBox {
        self.vbox.widget().clone()
    }

    fn view(relm: &Relm<Self>, _model: Self::Model) -> Self {
        let vbox = create_container::<VBox, _>(relm, ());

        let plus_button = gtk::Button::new_with_label("+");
        vbox.add(&plus_button);

        let counter_label = Label::new("0");
        vbox.add(&counter_label);

        let widget = vbox.add_widget::<Button, _>(relm, ());

        let minus_button = gtk::Button::new_with_label("-");
        vbox.add(&minus_button);

        MyVBox {
            vbox: vbox,
            _widget: widget,
        }
    }
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

struct Win {
    _vbox: Component<MyVBox>,
    window: Window,
}

impl Update for Win {
    type Model = ();
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> () {
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Window {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, _model: Self::Model) -> Self {
        let window = Window::new(WindowType::Toplevel);
        let vbox = window.add_widget::<MyVBox, _>(relm, ());
        window.show_all();

        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));

        Win {
            _vbox: vbox,
            window: window,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
