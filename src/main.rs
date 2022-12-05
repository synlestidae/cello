extern crate actix;
extern crate druid;
extern crate rand;
extern crate uuid;

mod game;

use crate::actix::Actor;
use core::f32::consts::PI;
use game::{ActivateCanvas, CanvasState, RenderPipeline, SpawnNew};
use std::sync::mpsc::channel;
use std::thread::spawn;
use std::time::Duration;
use std::time::Instant;

use druid::piet::kurbo::Circle;
use druid::widget::{Align, Flex, Label, Painter, TextBox};
use druid::{
    AppLauncher, Color, Data, Env, Lens, LocalizedString, Point, RenderContext, Selector, Target,
    Widget, WidgetExt, WindowDesc,
};

#[derive(Clone, Debug)]
struct MyData;

impl Data for MyData {
    fn same(&self, _: &Self) -> bool {
        false
    }
}

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Cell-O!");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

const WIDTH: f32 = 1000.0 * 2.0;
const HEIGHT: f32 = 1000.0 * 2.0;

fn main() {
    let (sender, receiver) = channel();
    let pipeline = RenderPipeline { sender };

    spawn(move || {
        let runner = actix::System::new();

        let canvas = game::Canvas {
            width: WIDTH,
            height: HEIGHT,
            time: Instant::now(),
            cells: Vec::new(),
            pipeline,
        };
        runner.block_on(async {
            let canvas_addr = canvas.start();
            println!("Start it up");
            canvas_addr.do_send(SpawnNew {
                name: format!("Booboo"),
            });
            canvas_addr.do_send(ActivateCanvas);
            async_std::task::sleep(std::time::Duration::new(60 * 60 * 24, 0)).await;
        });

        //println!("Loopy poopy");
        //let sleep = );
        //runner.block_on(sleep);
        //runner.run();
        //panic!("Actix thread has ended");
    });

    // start the application
    //spawn(move || {
    println!("Spawn!");
    let my_painter = move || {
        Painter::new(move |ctx, data: &MyData, _| {
            let bounds = ctx.size().to_rect();
            let (x0, y0, x1, y1) = (bounds.x0, bounds.y0, bounds.x1, bounds.y1);
            let mut canvas_state = CanvasState {
                cells: vec![],
                height: HEIGHT,
                width: WIDTH,
            };
            println!("Begin iter");
            for cell in receiver.try_iter() {
                canvas_state.cell(&cell);
                break;
            }
            println!("End iter");

            println!("Begin cell {:?}", canvas_state);
            for cell in canvas_state.cells.iter() {
                println!("{:?}", cell);
                let radius = (cell.size as f32 / PI).sqrt();
                let ratio = (x1 - x0) / WIDTH as f64;

                let radius_scaled = radius as f64 * ratio;
                let (cell_x, cell_y) = cell.position;
                let x_scaled = cell_x as f64 * ratio;
                let y_scaled = cell_y as f64 * ratio;
                // first scale down the size
                // then find the position in the bounds
                let circle = Circle::new(
                    Point {
                        x: x_scaled,
                        y: y_scaled,
                    },
                    radius_scaled,
                );
                println!("Circle {:?}", circle);
                ctx.fill(circle, &Color::Rgba32(0xABCDEF));
            }

            println!("Should render now");
        })
    };

    // describe the main window
    let main_window = WindowDesc::new(my_painter)
        //.title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = HelloState {
        name: "World".into(),
    };

    println!("Launching");

    let launcher = AppLauncher::with_window(main_window);

    let handle = launcher.get_external_handle();
    spawn(move || loop {
        handle.submit_command(Selector::NOOP, (), Target::Auto);
        std::thread::sleep(Duration::from_millis(1000 / 60));
    });

    launcher
        .launch(MyData)
        .expect("Failed to launch application");
    //});
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
