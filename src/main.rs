use rand::{thread_rng, Rng};
use std::io::{Read, Error, stdout};
use tui::{
    Terminal,
    backend::TermionBackend,
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Chart, Block, Dataset, Axis},
};
use termion::{
    raw::IntoRawMode,
    async_stdin,
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

fn main() -> Result<(), Error> {
    // initialize the terminal
    let stdout = stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut stdin = async_stdin().bytes();
    let mut rng = thread_rng();
    let base = 10.0f64;

    let mut b1 = Body {
        loc: Point {
            x: 0.0,
            y: 350.0 * base.powi(10),
            z: 0.0
        },
        vel: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0 
        },
        acc: Point::build_zero(),
        mass: 1.0 * base.powi(30),
    };
    let mut b2 = Body {
        loc: Point {
            x: 350.0 * base.powi(10),
            y: 100.0 * base.powi(10),
            z: 0.0
        },
        vel: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        acc: Point::build_zero(),
        mass: 1.0 * base.powi(30),
    };
    let mut b3 = Body {
        loc: Point {
            x: -350.0 * base.powi(10),
            y: 200.0 * base.powi(10),
            z: 0.0
        },
        vel: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        acc: Point::build_zero(),
        mass: 1.0 * base.powi(30),
    };

    terminal.clear();
    loop {
        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        }

        for _ in 0..10000 {
            b1.update_acc(&b2, &b3);
            b2.update_acc(&b3, &b1);
            b3.update_acc(&b1, &b2);
            b1.step(500.0);
            b2.step(500.0);
            b3.step(500.0);
        }

        let body1 = vec![(b1.loc.x, b1.loc.y)];
        let body2 = vec![(b2.loc.x, b2.loc.y)];
        let body3 = vec![(b3.loc.x, b3.loc.y)];

        let snapshot1 = Dataset::default()
            .name("Body 1")
            .style(Style::default().fg(Color::White))
            .marker(symbols::Marker::Dot)
            .data(&body1);
        let snapshot2 = Dataset::default()
            .name("Body 2")
            .style(Style::default().fg(Color::Green))
            .marker(symbols::Marker::Dot)
            .data(&body2);
        let snapshot3 = Dataset::default()
            .name("Body 3")
            .style(Style::default().fg(Color::Red))
            .marker(symbols::Marker::Dot)
            .data(&body3);

        let datasets = vec![snapshot1, snapshot2, snapshot3];

        terminal.draw(|f| {
            let size = f.size();
            let chart = Chart::new(datasets)
                .block(Block::default().title("TreBody"))
                .x_axis(Axis::default()
                        .style(Style::default().fg(Color::White))
                        .bounds([-50.0 * base.powi(11), 50.0 * base.powi(11)]))
                .y_axis(Axis::default()
                        .style(Style::default().fg(Color::White))
                        .bounds([-50.0 * base.powi(11), 50.0 * base.powi(11)]));
            f.render_widget(chart, size);
        });
    }
    Ok(())
}
