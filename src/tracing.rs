use std::{
    collections::HashMap,
    io::Write,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Mutex,
    },
    time::Instant,
};

use tracing::{span::Attributes, Id, Subscriber};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

static UNIQUE_ID: AtomicUsize = AtomicUsize::new(0);

pub struct TimingLayer<W> {
    inner: Mutex<HashMap<Id, Timer>>,
    writer: Mutex<W>,
}

impl<W> TimingLayer<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Self {
            inner: Mutex::new(HashMap::with_capacity(128)),
            writer: Mutex::new(writer),
        }
    }
}

#[derive(Clone)]
struct Timer {
    name: &'static str,
    unique_id: usize,
    start: Instant,
    parent_unique_id: Option<usize>,
    parent_start: Option<Instant>,
}

impl<S, W> Layer<S> for TimingLayer<W>
where
    S: Subscriber,
    S: for<'lookup> LookupSpan<'lookup>,
    W: Write + 'static,
{
    fn on_new_span(&self, _attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).unwrap();
        let name = span.name();
        let parent_id = span.parent().map(|parent| parent.id());
        let unique_id = UNIQUE_ID.fetch_add(1, SeqCst);

        let mut inner = self.inner.lock().unwrap();
        let (parent_unique_id, parent_start) = match parent_id.map(|id| {
            let parent = inner.get(&id).unwrap();
            (parent.unique_id, parent.start)
        }) {
            Some((x, y)) => (Some(x), Some(y)),
            None => (None, None),
        };
        let _ = inner.insert(
            id.clone(),
            Timer {
                name,
                unique_id,
                start: Instant::now(),
                parent_unique_id,
                parent_start,
            },
        );
    }
    fn on_enter(&self, id: &Id, _ctx: Context<'_, S>) {
        let inner = self.inner.lock().unwrap();
        let timer = &inner.get(id).unwrap();
        let mut file = self.writer.lock().unwrap();
        let _ = writeln!(
            file,
            "{{type:enter,name:{:?},id:{},parent_id:{:?},elapsed:{}}}",
            timer.name,
            timer.unique_id,
            timer.parent_unique_id,
            if let Some(start) = timer.parent_start {
                start.elapsed().as_micros()
            } else {
                0
            }
        );
    }

    fn on_exit(&self, id: &Id, _ctx: Context<'_, S>) {
        let inner = self.inner.lock().unwrap();
        let timer = &inner.get(id).unwrap();
        let mut file = self.writer.lock().unwrap();
        let _ = writeln!(
            file,
            "{{type:exit,name:{:?},id:{},parent_id:{:?},elapsed:{}}}",
            timer.name,
            timer.unique_id,
            timer.parent_unique_id,
            timer.start.elapsed().as_micros(),
        );
    }
}
