use std::f64;
use std::ops::{Add,Sub,Neg};

pub struct Hex {q: i64, r: i64}

impl Add for Hex {
    type Output = Hex;

    fn add(self, other: Hex) -> Hex {
        Hex{q: self.q + other.q,
            r: self.r + other.r}
    }
}

pub struct Cube {x: i64, y: i64, z: i64}

impl Add for Cube {
    type Output = Cube;

    fn add(self, other: Cube) -> Cube {
        Cube{x: self.x + other.x,
             y: self.y + other.y,
             z: self.z + other.z}
    }
}

impl Neg for Cube {
    type Output = Cube;

    fn neg(self) -> Cube {
        Cube{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Sub for Cube {
    type Output = Cube;

    fn sub(self, other: Cube) -> Cube {
        self + (-other)
    }
}

pub fn hex_to_cube(h: Hex) -> Cube { // axial
    Cube{x: h.q, y: -h.q-h.r, z: h.r}
}

pub fn cube_to_hex(h: Cube) -> Hex { // axial
    Hex{q: h.x, r: h.z}
}

pub struct Unit {members: Vec<Cube>, pivot: Cube}

pub fn unit_rotate(u: Unit, clockwise: bool) -> Unit {
    let mut v = Unit{members: Vec::new(), pivot: u.pivot};
    for h in u.members {
        let tmp_pivot = Cube{x: v.pivot.x, y: v.pivot.y, z: v.pivot.z};
        let other_tmp_pivot = Cube{x: v.pivot.x, y: v.pivot.y, z: v.pivot.z};
        v.members.push(cube_rotate(h - tmp_pivot, clockwise) + other_tmp_pivot);
    }
    return v;
}

pub fn unit_neighbor(u: Unit, direction: i8) -> Unit {
    let mut v = Unit{members: Vec::new(), pivot: u.pivot};
    for h in u.members {
        v.members.push(cube_neighbor(h, direction));
    }
    return v;
}
// ```python
// # convert cube to odd-r offset
// col = x + (z - (z&1)) / 2
// row = z
//
// # convert odd-r offset to cube
// x = col - (row - (row&1)) / 2
// z = row
// y = -x-z
// ```
pub struct OddR {col: i64, row: i64}

impl Add for OddR {
    type Output = OddR;

    fn add(self, other: OddR) -> OddR {
        OddR{col: self.col + other.col, row: self.row + other.row}
    }
}

pub fn cube_to_offset(h: Cube) -> OddR {
    let z = h.z;
    let col = h.x + (z - (z&1)) / 2;
    let row = z;
    OddR{col: col, row: row,}
}

pub fn offset_to_cube(h: OddR) -> Cube {
    let row = h.row;
    let x = h.col - (row - (row&1)) / 2;
    let z = row;
    let y = -x-z;
    Cube{x: x, y: y, z: z,}
}

// ```python
// var directions = [
//    Cube(+1, -1,  0), Cube(+1,  0, -1), Cube( 0, +1, -1),
//    Cube(-1, +1,  0), Cube(-1,  0, +1), Cube( 0, -1, +1)
// ]
//
// function cube_direction(direction):
//     return directions[direction]
//
// function cube_neighbor(hex, direction):
//     return cube_add(hex, cube_direction(direction))
// ```
pub fn cube_direction(direction: i8) -> Cube {
    match (direction % 6) + 6 {
        0 => Cube{x: 1, y: -1, z: 0,},
        1 => Cube{x: 1, y: 0, z: -1,},
        2 => Cube{x: 0, y: 1, z: -1,},
        3 => Cube{x: -1, y: 1, z: 0,},
        4 => Cube{x: -1, y: 0, z: 1,},
        5 => Cube{x: 0, y: -1, z: 1,},
        _ => Cube{x: 0, y: 0, z: 0,}
    }
}

pub fn cube_neighbor(hex: Cube, direction: i8) -> Cube {
    hex + cube_direction(direction)
}

pub fn cube_rotate(hex: Cube, clockwise: bool) -> Cube {
    if clockwise {
        Cube{x: -hex.z, y: -hex.x, z: -hex.y}
    } else {
        Cube{x: -hex.y, y: -hex.z, z: -hex.x}
    }
}

pub fn cube_distance(hex: Cube, other: Cube) -> i64 {
    let difference = hex - other;
    let sum = difference.x.abs() + difference.y.abs() + difference.z.abs();
    return sum / 2_i64;
}

// ```python
// function cube_round(h):
//     var rx = round(h.x)
//     var ry = round(h.y)
//     var rz = round(h.z)
//
//     var x_diff = abs(rx - h.x)
//     var y_diff = abs(ry - h.y)
//     var z_diff = abs(rz - h.z)
//
//     if x_diff > y_diff and x_diff > z_diff:
//         rx = -ry-rz
//     else if y_diff > z_diff:
//         ry = -rx-rz
//     else:
//         rz = -rx-ry
//
//     return Cube(rx, ry, rz)
//
// function cube_lerp(a, b, t):
//     return Cube(a.x + (b.x - a.x) * t,
//                 a.y + (b.y - a.y) * t,
//                 a.z + (b.z - a.z) * t)
//
// function cube_linedraw(a, b):
//     var N = cube_distance(a, b)
//     var results = []
//     for each 0 ≤ i ≤ N:
//         results.append(cube_round(cube_lerp(a, b, 1.0/N * i)))
//     return results
// ```
pub fn cube_linedraw(a: Cube, b: Cube) -> Vec<Cube> {
    // The implementation is terrible here.
    // This should be fixed to not move ownership
    // of `a` and `b` and just use borrowing.
    let Cube{x: ax, y: ay, z: az} = a;
    let Cube{x: bx, y: by, z: bz} = b;
    let n: f64 = cube_distance(a, b) as f64;
    let mut results = vec![];
    for x in 0..n as i32 {
        let acopy = Cube{x: ax, y: ay, z: az};
        let bcopy = Cube{x: bx, y: by, z: bz};
        results.push(cube_round(cube_lerp(acopy, bcopy, 1_f64/n * (x as f64))));
    }
    return results;
}

pub fn cube_lerp(a: Cube, b: Cube, t: f64) -> (f64, f64, f64){
    let x = (a.x + (b.x - a.x)) as f64;
    let y = (a.y + (b.y - a.y)) as f64;
    let z = (a.z + (b.z - a.z)) as f64;
    return (x * t, y * t, z * t);
}

pub fn cube_round(point: (f64, f64, f64)) -> Cube {
    let (x, y, z) = point;
    let rx = x.round();
    let ry = y.round();
    let rz = z.round();

    let x_diff = x.abs();
    let y_diff = y.abs();
    let z_diff = z.abs();

    if x_diff > y_diff && x_diff > z_diff {
        Cube{x: (-ry-rz) as i64, y: ry as i64, z: rz as i64}
    } else {
        if y_diff > z_diff {
            Cube{x: rx as i64, y: (-rx-rz) as i64, z: rz as i64}
        } else {
            Cube{x: rx as i64, y: ry as i64, z: (-rx-ry) as i64}
        }
    }
}

// ```python
// var directions = [
//    Hex(+1,  0), Hex(+1, -1), Hex( 0, -1),
//    Hex(-1,  0), Hex(-1, +1), Hex( 0, +1)
// ]
//
// function hex_direction(direction):
//     return directions[direction]
//
// function hex_neighbor(hex, direction):
//     var dir = hex_direction(direction)
//     return Hex(hex.q + dir.q, hex.r + dir.r)
// ```
pub fn hex_direction(direction: i8) -> Hex {
    match (direction % 6) + 6 {
        0 => Hex{q: 1, r: -1,},
        1 => Hex{q: 1, r: 0,},
        2 => Hex{q: 0, r: -1,},
        3 => Hex{q: -1, r: 0,},
        4 => Hex{q: -1, r: 1,},
        5 => Hex{q: 0, r: 1,},
        _ => Hex{q: 0, r: 0,}
    }
}

pub fn hex_neighbor(hex: Hex, direction: i8) -> Hex {
    hex + hex_direction(direction)
}

// ```python
// var directions = [
//    [ Hex(+1,  0), Hex( 0, -1), Hex(-1, -1),
//      Hex(-1,  0), Hex(-1, +1), Hex( 0, +1) ],
//    [ Hex(+1,  0), Hex(+1, -1), Hex( 0, -1),
//      Hex(-1,  0), Hex( 0, +1), Hex(+1, +1) ]
// ]
//
// function offset_neighbor(hex, direction):
//     var parity = hex.row & 1
//     var dir = directions[parity][direction]
//     return Hex(hex.col + dir.col, hex.row + dir.row)
// ```
pub fn offset_direction(parity: i64, direction: i8) -> OddR {
    match parity {
        0 => match (direction % 6) + 6 {
            0 => OddR{col: 1, row: 0,},
            1 => OddR{col: 0, row: -1,},
            2 => OddR{col: -1, row: -1,},
            3 => OddR{col: -1, row: 0,},
            4 => OddR{col: -1, row: 1,},
            5 => OddR{col: 0, row: 1,},
            _ => OddR{col: 0, row: 0,}
        },
        1 => match (direction % 6) + 6 {
            0 => OddR{col: 1, row: 0,},
            1 => OddR{col: 1, row: -1,},
            2 => OddR{col: 0, row: -1,},
            3 => OddR{col: -1, row: 0,},
            4 => OddR{col: 0, row: 1,},
            5 => OddR{col: 1, row: 1,},
            _ => OddR{col: 0, row: 0,}
        },
        _ => OddR{col: 0, row: 0},
    }
}

pub fn offset_neighbor(hex: OddR, direction: i8) -> OddR {
    // Have to add `hex` second to avoid ownership issues
    offset_direction( (hex.row&1), direction) + hex
}

// ```python
// function hex_corner(center, size, i):
//     var angle_deg = 60 * i   + 30
//     var angle_rad = PI / 180 * angle_deg
//     return Point(center.x + size * cos(angle_rad),
//                  center.y + size * sin(angle_rad))
// ```
pub struct Point {x: f64, y: f64}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point{x: self.x + other.x,
              y: self.y + other.y}
    }
}

pub fn hex_corner(center: Point, size: f64, i: i8) -> Point {
    let corner: f64 = i as f64;
    let angle_deg: f64 = 60_f64 * corner + 30_f64;
    let angle_rad: f64 = f64::consts::PI / 180_f64 * angle_deg;
    Point{x: center.x + size * angle_rad.cos(),
          y: center.y + size * angle_rad.sin()}
}

#[test]
fn it_works() {
    assert_eq!( 3 & 1, 1 );
    let vertex: Point = hex_corner(Point{x: 0_f64, y: 0_f64}, 1_f64, 1);
    let difference: Point = vertex - Point{x: 0_f64, y: 1_f64};
    assert!(difference.x.abs() < 0.0001);
    assert!(difference.y.abs() < 0.0001);
}
