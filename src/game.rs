type Vector = (u32, u32);
type Colour = (u8, u8, u8);

struct Canvas {
    width: u32,
    height: u32
}

struct Cell {
    size: u32,
    position: Vector, 
    velocity: Vector
}


