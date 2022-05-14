use crate::lib::{
    coordinates,
    mandy::{ManCanvas, Mandy},
};

use super::{coordinates::Coordinates, imag::Imaginary};

pub struct Canvas<'a> {
    pub lines: Vec<Vec<&'a str>>,
}
pub struct Viewer<'a> {
    pub doc: Canvas<'a>,
    pub img: ManCanvas,
    pub terminal_size: coordinates::Coordinates,
}

impl Viewer<'_> {
    pub fn init(term_size: Coordinates, offset: Imaginary, zoom: f64) -> Self {
        let term_canvas = Canvas {
            lines: vec![vec![""; term_size.x]; term_size.y],
        };
        let mut img_canvas = ManCanvas {
            elems: vec![vec![Mandy::new(Imaginary::zero()); term_size.x]; term_size.y],
        };
        let x_seg_size = 8.0 / (zoom * (img_canvas.elems[0].len() as f64)); //length of a chunk of domain
        let y_seg_size = 4.0 / (zoom * (img_canvas.elems.len() as f64)); //length of a chunk of range

        for row in 0..img_canvas.elems.len() {
            for col in 0..img_canvas.elems[row].len() {
                img_canvas.elems[row][col].c = Imaginary {
                    // -4 to 4 -2 to 2
                    // lets divide the x into (number of cols) sections, take centerpoint for each
                    // save for y but with rows
                    a: ((col as f64) * x_seg_size) - (4.0 / zoom) + offset.a,
                    b: (((img_canvas.elems.len() - row) as f64) * y_seg_size) - (2.0 / zoom)
                        + offset.b,
                }
            }
        }
        Self {
            doc: term_canvas,
            img: img_canvas,
            terminal_size: term_size,
        }
    }
}
