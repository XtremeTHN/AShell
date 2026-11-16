// thanks claude

use amane::{Button, Fill::Mask, Point, Rectangle, Service, Stack, children};
use libwayshot::{
    LogicalRegion, Size, WayshotConnection,
    region::{Position, Region},
};

use crate::{components::screenshot::state::ScreenshotState, services::colors::ColorsService};

const HANDLE: f32 = 12.0; // visual diameter
const GRAB: f32 = 14.0; // hit radius, a bit bigger than what's drawn

#[derive(Default, Clone, Copy, PartialEq, Debug)]
enum Mode {
    #[default]
    Idle,
    Create,
    Move,
    Resize,
}

#[derive(Default, Debug)]
struct State {
    initial: Option<Point>,
    current: Option<Point>,
    mode: Mode,
    last: Option<Point>,
}

impl Service for State {
    fn new() -> Self {
        Self::default()
    }

    fn listen() {}
}

fn rect_of(s: &State) -> Option<(f32, f32, f32, f32)> {
    let (a, b) = (s.initial?, s.current?);
    Some((
        a.x.min(b.x),
        a.y.min(b.y),
        (a.x - b.x).abs(),
        (a.y - b.y).abs(),
    ))
}

// Order: top-left, top-right, bottom-left, bottom-right.
// The opposite corner of index i is always 3 - i.
fn corners(x: f32, y: f32, w: f32, h: f32) -> [Point; 4] {
    [
        Point { x, y },
        Point { x: x + w, y },
        Point { x, y: y + h },
        Point { x: x + w, y: y + h },
    ]
}

fn corner_at(s: &State, p: Point) -> Option<usize> {
    let (x, y, w, h) = rect_of(s)?;
    corners(x, y, w, h).iter().position(|c| {
        let (dx, dy) = (c.x - p.x, c.y - p.y);
        dx * dx + dy * dy <= GRAB * GRAB
    })
}

fn inside(s: &State, p: Point) -> bool {
    rect_of(s).is_some_and(|(x, y, w, h)| p.x >= x && p.x <= x + w && p.y >= y && p.y <= y + h)
}

pub fn on_pressed(_: Button) {
    let mut s = State::write();
    s.mode = Mode::Idle;
    s.last = None;
}

fn sync_ready() {
    let ready = rect_of(&State::read()).is_some_and(|(_, _, w, h)| w >= 1.0 && h >= 1.0);

    if ScreenshotState::read().ready != ready {
        ScreenshotState::write().ready = ready;
    }
}

pub fn on_drag(point: Point) {
    {
        let mut s = State::write();

        if s.mode == Mode::Idle {
            // Handles first: they stick out past the rectangle's edge.
            if let Some(i) = corner_at(&s, point) {
                let (x, y, w, h) = rect_of(&s).unwrap();
                s.initial = Some(corners(x, y, w, h)[3 - i]); // anchor the opposite corner
                s.mode = Mode::Resize;
            } else if inside(&s, point) {
                s.mode = Mode::Move;
            } else {
                s.mode = Mode::Create;
                s.initial = Some(point);
            }
            s.last = Some(point);
        }

        match s.mode {
            Mode::Create | Mode::Resize => s.current = Some(point),
            Mode::Move => {
                let last = s.last.unwrap_or(point);
                let (dx, dy) = (point.x - last.x, point.y - last.y);
                if let (Some(a), Some(b)) = (s.initial, s.current) {
                    s.initial = Some(Point {
                        x: a.x + dx,
                        y: a.y + dy,
                    });
                    s.current = Some(Point {
                        x: b.x + dx,
                        y: b.y + dy,
                    });
                }
                s.last = Some(point);
            }
            Mode::Idle => {}
        }
    }

    sync_ready();
}

fn handle(c: Point, color: amane::Color) -> Rectangle {
    Rectangle::new()
        .width(HANDLE)
        .height(HANDLE)
        .radius(HANDLE / 2.0) // fully rounded = circle
        .translate(c.x - HANDLE / 2.0, c.y - HANDLE / 2.0)
        .fill(color)
}

pub fn available() -> bool {
    if let Err(e) = WayshotConnection::new() {
        log::warn!("failed to create wayshot connection: {}", e);
        false
    } else {
        true
    }
}

pub fn region() -> LogicalRegion {
    let Some((x, y, w, h)) = rect_of(&State::read()) else {
        unreachable!()
    };

    LogicalRegion {
        inner: Region {
            position: Position {
                x: x as i32,
                y: y as i32,
            },
            size: Size {
                width: w as u32,
                height: h as u32,
            },
        },
    }
}

pub fn selector() -> Stack {
    let s = State::read();
    let colors = ColorsService::read();
    let outline = colors.c.outline_variant.value();

    let Some((x, y, w, h)) = rect_of(&s) else {
        return Stack::new(children!());
    };

    let body = Rectangle::new()
        .width(w)
        .height(h)
        .translate(x, y)
        .fill(Mask)
        .border(1.0, outline);

    let [tl, tr, bl, br] = corners(x, y, w, h);

    Stack::new(children!(
        body,
        handle(tl, outline),
        handle(tr, outline),
        handle(bl, outline),
        handle(br, outline)
    ))
}
