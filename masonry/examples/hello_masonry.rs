// Copyright 2019 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! This is a very small example of how to setup a masonry application.
//! It does the almost bare minimum while still being useful.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use masonry::dpi::LogicalSize;
use masonry::text::StyleProperty;
use masonry::widget::{Button, Flex, Label, RootWidget};
use masonry::{Action, AppDriver, DriverCtx, WidgetId};
use parley::fontique::Weight;
use winit::window::Window;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

struct Driver;

impl AppDriver for Driver {
    fn on_action(&mut self, _ctx: &mut DriverCtx<'_>, _widget_id: WidgetId, action: Action) {
        match action {
            Action::ButtonPressed(_) => {
                println!("Hello");
            }
            action => {
                eprintln!("Unexpected action {action:?}");
            }
        }
    }
}

fn main() {
    let label = Label::new("Hello")
        .with_style(StyleProperty::FontSize(32.0))
        // Ideally there's be an Into in Parley for this
        .with_style(StyleProperty::FontWeight(Weight::BOLD));
    let button = Button::new("Say hello");

    // Arrange the two widgets vertically, with some padding
    let main_widget = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(button);

    let window_size = LogicalSize::new(400.0, 400.0);
    let window_attributes = Window::default_attributes()
        .with_title("Hello World!")
        .with_resizable(true)
        .with_min_inner_size(window_size);

    masonry::event_loop_runner::run(
        masonry::event_loop_runner::EventLoop::with_user_event(),
        window_attributes,
        RootWidget::new(main_widget),
        Driver,
    )
    .unwrap();
}
