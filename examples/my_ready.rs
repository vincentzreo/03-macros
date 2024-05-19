use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Future;

#[tokio::main]
async fn main() {
    //let ret = poll_fut(&mut Context::from_waker(futures::task::noop_waker_ref()));
    let fut = MyFut::new(42);
    println!("{:?}", fut.await);
}

#[allow(dead_code)]
fn poll_fut(cx: &mut Context<'_>) -> Poll<usize> {
    let mut fut = MyFut::new(42);
    let fut = Pin::new(&mut fut);
    my_ready!(fut.poll(cx))
}

struct MyFut {
    polled: bool,
    value: usize,
}

impl MyFut {
    fn new(value: usize) -> Self {
        Self {
            polled: false,
            value,
        }
    }
}

impl Future for MyFut {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.value)
        } else {
            self.polled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            std::task::Poll::Ready(val) => std::task::Poll::Ready(val),
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
