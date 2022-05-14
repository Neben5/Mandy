mod lib;
use std::io::{self, stdin, stdout, Stdout, Write};

use lib::view::Viewer;
use termion::{event::Key, input::TermRead, raw::RawTerminal, *};

use crate::lib::{coordinates::Coordinates, imag::Imaginary};

fn main() {
    let mut stdout = raw::IntoRawMode::into_raw_mode(stdout()).unwrap();
    let stdin = stdin();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    write!(stdout, "{}", termion::clear::All).unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
    const MAX_ITER: usize = 100;
    let mut offset: Imaginary = Imaginary { a: 0., b: 0. };
    let mut zoom: f64 = 1.0;

    set(offset.a, offset.b, zoom, MAX_ITER, &mut stdout);

    io::stdout().flush().unwrap();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                break;
            }
            Key::Right => {
                offset.a += (1. / (8. * zoom));
                set(offset.a, offset.b, zoom, MAX_ITER, &mut stdout);
            }
            Key::Left => {
                offset.a -= (1. / (8. * zoom));
                set(offset.a, offset.b, zoom, MAX_ITER, &mut stdout);
            }
            Key::Up => {
                offset.b -= (1. / (16. * zoom));
                set(offset.a, offset.b, zoom, MAX_ITER, &mut stdout);
            }
            Key::Down => {
                offset.b += (1. / (16. * zoom));
                set(offset.a, offset.b, zoom, MAX_ITER, &mut stdout);
            }
            Key::Char('s') => {
                zoom -= 1.;
                set(offset.a, offset.b, zoom, MAX_ITER, &mut stdout);
            }
            Key::Char('w') => {
                zoom += 1.;
                set(offset.a, offset.b, zoom, MAX_ITER, &mut stdout);
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}

fn set(
    a_offset: f64,
    b_offset: f64,
    zoom: f64,
    MAX_ITER: usize,
    mut stdout: &mut RawTerminal<Stdout>,
) {
    let mut view = Viewer::init(
        Coordinates::from(terminal_size().unwrap()),
        Imaginary {
            a: a_offset,
            b: b_offset,
        },
        zoom,
    );

    for row in 0..view.img.elems.len() {
        for col in 0..view.img.elems[0].len() {
            while view.img.elems[row][col].niter < MAX_ITER.into()
                && !(view.img.elems[row][col].isdiv())
            {
                view.img.elems[row][col].z = view.img.elems[row][col].iter();
                view.img.elems[row][col].niter += 1;
            }
        }
    }

    for row in 0..view.img.elems.len() {
        for col in 0..view.img.elems[0].len() {
            let c = ((((view.img.elems[row][col].niter - 1) as f64) / (MAX_ITER as f64))
                .sin()
                .sqrt()
                * 255.0) as u8;
            write!(
                *stdout,
                "{}*",
                termion::color::Fg(termion::color::Rgb(c, 0, c))
            )
            .unwrap();
        }
    }
}
