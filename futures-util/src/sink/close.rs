use core::marker::PhantomData;
use core::pin::Pin;
use futures_core::future::Future;
use futures_core::task::{Waker, Poll};
use futures_sink::Sink;

/// Future for the `close` combinator, which polls the sink until all data has
/// been closed.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct Close<'a, Si: Sink<Item> + Unpin + ?Sized, Item> {
    sink: &'a mut Si,
    _phantom: PhantomData<fn(Item)>,
}

/// A future that completes when the sink has finished closing.
///
/// The sink itself is returned after closeing is complete.
impl<'a, Si: Sink<Item> + Unpin + ?Sized, Item> Close<'a, Si, Item> {
    pub(super) fn new(sink: &'a mut Si) -> Self {
        Close {
            sink,
            _phantom: PhantomData,
        }
    }
}

impl<Si: Sink<Item> + Unpin + ?Sized, Item> Future for Close<'_, Si, Item> {
    type Output = Result<(), Si::SinkError>;

    fn poll(
        mut self: Pin<&mut Self>,
        waker: &Waker,
    ) -> Poll<Self::Output> {
        Pin::new(&mut self.sink).poll_close(waker)
    }
}
