// claude helped but i did this
pub mod top;
use amane::{Animation, Rectangle, Size, Stack, Widget, children};
use std::sync::Arc;

pub const SLIDE: f32 = 30.0;

pub type PageBuilder = Arc<dyn Fn() -> Box<dyn Widget> + Send + Sync + 'static>;

pub struct Page {
    id: Arc<str>,
    builder: PageBuilder,
}

pub struct Transition {
    pub from: Option<PageBuilder>, // None = home
    pub forward: bool,             // true = push, false = pop
}

/// Everything needed to draw one frame. Owns its data (no lock held).
pub struct NavFrame {
    from: Option<Option<PageBuilder>>, // Some = transition running; inner None = home
    to: Option<PageBuilder>,           // None = home
    forward: bool,
    t: f32,
}

pub struct Navigator {
    children: Vec<Page>,
    active_child_name: Option<Arc<str>>,
    pub transition: Option<Transition>,
    pub progress: Animation<f32>,
}

impl Default for Navigator {
    fn default() -> Self {
        Self {
            children: vec![],
            active_child_name: None,
            transition: None,
            progress: Animation::new(1.0), // 1.0 = idle
        }
    }
}

impl Navigator {
    fn begin(&mut self, from: Option<PageBuilder>, forward: bool) {
        self.transition = Some(Transition { from, forward });
        self.progress = Animation::new(0.0);
        self.progress.to(1.0);
    }

    pub fn push<T: Fn() -> Box<dyn Widget> + Send + Sync + 'static>(
        &mut self,
        id: impl Into<Arc<str>>,
        builder: T,
    ) {
        let from = self.active_child(); // capture before changing
        let id = id.into();
        self.children.push(Page {
            id: Arc::clone(&id),
            builder: Arc::new(builder),
        });
        self.active_child_name = Some(id);
        self.begin(from, true);
    }

    pub fn pop(&mut self) {
        if self.children.is_empty() {
            log::warn!("pop called with no pages");
            return;
        }
        let from = self.active_child();
        self.children.pop();
        self.active_child_name = self.children.last().map(|p| Arc::clone(&p.id));
        self.begin(from, false);
    }

    pub fn active_child(&self) -> Option<PageBuilder> {
        self.children
            .iter()
            .find(|p| self.active_child_name == Some(Arc::clone(&p.id)))
            .map(|p| Arc::clone(&p.builder))
    }

    pub fn frame(&self) -> NavFrame {
        let t = self.progress.value();
        let tr = self.transition.as_ref().filter(|_| t < 1.0);
        NavFrame {
            from: tr.map(|tr| tr.from.clone()),
            to: self.active_child(),
            forward: tr.map(|tr| tr.forward).unwrap_or(true),
            t,
        }
    }
}

fn fixed_height(w: &Box<dyn Widget>) -> f32 {
    match w.height() {
        Size::Fixed(height) => height,
        Size::Parent => unreachable!(),
    }
}

fn layer(w: Box<dyn Widget>, width: f32, h: f32, x: f32, opacity: f32) -> Rectangle {
    Rectangle::new()
        .width(width)
        .height(h)
        .boxed_child(w)
        .translate(x, 0.0)
        .opacity(opacity)
}

impl NavFrame {
    /// Returns the widget to show and its height.
    pub fn render(self, home: impl Fn() -> Box<dyn Widget>, width: f32) -> (Box<dyn Widget>, f32) {
        let build = |p: &Option<PageBuilder>| match p {
            Some(b) => b(),
            None => home(),
        };

        match self.from {
            Some(from) => {
                let dir = if self.forward { 1.0 } else { -1.0 };
                let t = self.t;
                let out = build(&from);
                let inc = build(&self.to);

                let (h_out, h_in) = (fixed_height(&out), fixed_height(&inc));
                let h = h_out + (h_in - h_out) * t;

                let stack = Stack::new(children!(
                    layer(out, width, h, -dir * t * SLIDE, 1.0 - t),
                    layer(inc, width, h, dir * (1.0 - t) * SLIDE, t),
                ));
                (Box::new(stack), h)
            }
            None => {
                let w = build(&self.to);
                let h = fixed_height(&w);
                (w, h)
            }
        }
    }
}
