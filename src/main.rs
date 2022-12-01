extern crate actix;
extern crate rand;
extern crate uuid;
extern crate druid;

mod game;

use std::time::Instant;
use crate::actix::Actor;
use game::{ActivateCanvas, SpawnNew};

use druid::widget::{Align, Flex, Label, TextBox, Painter};
use druid::{AppLauncher, Color, Point, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt, RenderContext};
use druid::piet::kurbo::Circle;

#[derive(Clone, Debug)]
struct MyData;

impl Data for MyData {
    fn same(&self, _: &Self) -> bool {
        true
    }
}

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Cell-O!");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

#[actix::main]
async fn main() {
    let canvas = game::Canvas {
        width: 1000.0 * 30.0,
        height: 1000.0 * 30.0,
        time: Instant::now(),
        cells: Vec::new()
    };
    let canvas_addr = canvas.start();
    /*canvas_addr.do_send(ActivateCanvas);
    canvas_addr.do_send(SpawnNew {
        name: format!("Booboo")
    });*/

    let my_painter = Painter::new(|ctx, data: &MyData, _| {
        //let bounds = ctx.size().to_rect();
        let circle = Circle::new(Point { x: 30.0, y: 30.0 }, 150.0);
        ctx.fill(circle, &Color::Rgba32(0xABCDEF));
    });

    // describe the main window
    let main_window = WindowDesc::new(move || my_painter)
        //.title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = HelloState {
        name: "World".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(MyData)
        .expect("Failed to launch application");
    loop {
        async_std::task::sleep(std::time::Duration::new(60 * 60 * 24, 0)).await; // sleep for a day idk
    }
}

fn build_root_widget() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| format!("Hello {}!", data.name));
    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox);

    // center the two widgets in the available space
    Align::centered(layout)
}
