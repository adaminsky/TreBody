use std::io;
use tui::{
    Terminal,
    backend::TermionBackend,
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Chart, Borders, Block, Dataset, Axis},
};
use termion::{
    event::Event::Key,
    input::MouseTerminal,
    raw::IntoRawMode,
    screen::AlternateScreen,
};

const G: f64 = 6.674*0.00000000001;

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

struct Body {
    loc: Point,
    vel: Point,
    acc: Point,
    mass: f64,
}

impl Point {
    fn to_string(&self) -> String {
        format!("{},{},{}", self.x, self.y, self.z)
    }

    fn build_zero() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn add(&self, rhs: &Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    fn sub(&self, rhs: &Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }

    fn div(&self, c: f64) -> Point {
        Point {
            x: self.x / c,
            y: self.y / c,
            z: self.z / c,
        }
    }

    fn mul(&self, c: f64) -> Point {
        Point {
            x: self.x * c,
            y: self.y * c,
            z: self.z * c,
        }
    }

    fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl Body {
    fn step(&mut self, time_step: f64) {
        self.loc = self.loc.add(&self.vel.mul(time_step));
        self.vel = self.vel.add(&self.acc.mul(time_step));
    }

    fn update_acc(&mut self, b2: &Body, b3: &Body) {
        self.acc = self.loc.sub(&b2.loc)
            .mul(-1.0*G*b2.mass)
            .div(self.loc.sub(&b2.loc)
                 .mag().powi(3))

            .sub(&self.loc.sub(&b3.loc)
                 .mul(G*b3.mass)
                 .div(self.loc.sub(&b3.loc)
                      .mag().powi(3)));
    }
}

#[test]
fn point_add() {
    let a = Point { x: 1.0, y: 0.0, z: 1.0 };
    let b = Point { x: 1.0, y: 1.0, z: 1.0 };
    let sum = a.add(&b);
    assert_eq!(sum.x, 2.0);
    assert_eq!(sum.y, 1.0);
    assert_eq!(sum.z, 2.0);
}

fn main() -> Result<(), io::Error> {
    // initialize the terminal
    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut b1 = Body {
        loc: Point { x: 20.0, y: 0.0, z: -33.0 },
        vel: Point::build_zero(),
        acc: Point::build_zero(),
        mass: 1.0,
    };
    let mut b2 = Body {
        loc: Point { x: -20.0, y: 0.0, z: 15.0 },
        vel: Point::build_zero(),
        acc: Point::build_zero(),
        mass: 1.0,
    };
    let mut b3 = Body {
        loc: Point { x: 0.0, y: 20.0, z: 0.0 },
        vel: Point::build_zero(),
        acc: Point::build_zero(),
        mass: 1.0,
    };

    loop {
        for i in 0..1000 {
            b1.update_acc(&b2, &b3);
            b2.update_acc(&b3, &b1);
            b3.update_acc(&b1, &b2);
            b1.step(100.0);
            b2.step(100.0);
            b3.step(100.0);
        }
        let bodys = vec![(b1.loc.x, b1.loc.y),
                         (b2.loc.x, b2.loc.y),
                         (b3.loc.x, b3.loc.y)];

        let snapshot = Dataset::default()
            .name("test")
            .marker(symbols::Marker::Dot)
            .data(&bodys);

        let datasets = vec![snapshot];

        terminal.draw(|f| {
            let size = f.size();
            let chart = Chart::new(datasets)
                .block(Block::default().title("TreBody"))
                .x_axis(Axis::default()
                        .bounds([-50.0, 50.0]))
                .y_axis(Axis::default()
                        .bounds([-50.0, 50.0]));
            f.render_widget(chart, size);
        });
    }
    Ok(())
}
