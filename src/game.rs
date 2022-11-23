use crate::rand::Rng;
use actix::prelude::*;
use rand;
use std::time::Instant;
use uuid::Uuid;

type Vector<S> = (S, S);
type Colour = (u8, u8, u8);

const INIT_SIZE: u32 = 707; // about 30 pixels diameter
const BASE_MOMENTUM: f32 = 30.0 * 1.0;
const PI: f32 = 3.1415;

impl Actor for Canvas {
    type Context = Context<Self>;
}

struct Canvas {
    time: Instant,
    width: f32,
    height: f32,
    cells: Vec<Addr<Cell>>,
}

impl Handler<SpawnNew> for Canvas {
    type Result = Response<SpawnSuccess>;

    fn handle(&mut self, spawn_new: SpawnNew, ctx: &mut Context<Self>) -> Self::Result {
        let id = Uuid::new_v4();
        let mut rng = rand::thread_rng();
        let cell = Cell {
            id,
            name: spawn_new.name,
            position: (self.width / 2.0, self.height / 2.0), // TODO random,
            size: INIT_SIZE,
            direction_rads: rng.gen_range(0.0..PI),
            canvas: ctx.address(),
        };

        let cell_addr = cell.start();

        self.cells.push(cell_addr);

        Response::reply(SpawnSuccess { id })
    }
}

impl Handler<Tick> for Canvas {
    type Result = Response<()>;

    fn handle(&mut self, tick: Tick, _: &mut Context<Self>) -> Self::Result {
        for c in self.cells.iter() {
            c.send(CellTick {
                delta: 1.0, // TODO this should be based on time
                time: tick.time,
            });
        }

        Response::reply(())
    }
}

impl Handler<CellTick> for Cell {
    type Result = Response<CellInfo>;

    fn handle(&mut self, tick: CellTick, _: &mut Context<Self>) -> Self::Result {
        let (x0, y0) = self.position;
        let momentum_x = self.direction_rads.cos();
        let momentum_y = self.direction_rads.sin();

        let velocity_x = self.size as f32 / momentum_x;
        let velocity_y = self.size as f32 / momentum_y;

        self.position = (x0 + velocity_x * tick.delta, x0 + velocity_y * tick.delta);

        Response::reply(CellInfo {
            id: self.id,
            position: self.position,
            direction_rads: self.direction_rads,
            size: self.size,
        })
    }
}

impl Handler<ActivateCanvas> for Canvas {
    type Result = Response<()>;

    fn handle(&mut self, _: ActivateCanvas, ctx:  &mut Context<Self>) -> Self::Result) {
        // we'll spawn a timer
        todo!("ctx.run_interval()")
    }
}

struct Cell {
    id: Uuid,
    name: String,
    position: Vector<f32>,
    direction_rads: f32,
    size: u32,
    canvas: Addr<Canvas>,
}

struct CellInfo {
    id: Uuid,
    position: Vector<f32>,
    direction_rads: f32,
    size: u32,
}

impl Actor for Cell {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct ActivateCanvas {
}

#[derive(Message)]
#[rtype(result = "SpawnSuccess")]
struct SpawnNew {
    name: String,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
struct Tick {
    time: Instant,
}

#[derive(Message, Clone)]
#[rtype(result = "CellInfo")]
struct CellTick {
    time: Instant,
    delta: f32, // in seconds
}

struct SpawnSuccess {
    id: Uuid,
}
