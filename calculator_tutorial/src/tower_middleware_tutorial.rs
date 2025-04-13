use std::time::Duration;
use tower::Service;
use std::task::{Context, Poll};
use tokio::time::{sleep, Sleep};
use std::{future::Future, pin::Pin};
use pin_project::pin_project;
use std::fmt;

// adds a timelimit to the inner service

#[derive(Debug, Clone)]
pub struct Timeout<S> {
    inner: S,
    timeout: Duration,
}

impl<S> Timeout<S> {
    pub fn new(inner: S, timeput: Duration) -> Self {
        Self { inner, timeout: timeput }
    }
}


impl<S, Req> Service<Req> for Timeout<S>
where 
    S: Service<Req>, 
    S::Error: Into<BoxError>    
{
    // type aliases
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future>; // what is to be returned by the call fn


    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // the context struct seams to enable access to the rest of the runtime 
        // allowing us to set a reminder of sorts
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let response_future = self.inner.call(req);

        let sleep = sleep(self.timeout);

        ResponseFuture {
            response_future,
            sleep
        }
    }
}

pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[pin_project] // to pin the fields pf a struct, Pin<&mut Struct> to a Pin<&mut Field>.
pub struct ResponseFuture<F> {
    #[pin]
    response_future: F,
    #[pin]
    sleep: Sleep
}

impl<F, Resp, Err> Future for ResponseFuture<F> 
where 
    F: Future<Output = Result<Resp, Err>>, 
    Err: Into<BoxError>    
{
    
    type Output = Result<Resp, BoxError>;

    fn poll(
        self: Pin<&mut Self>, // pin guarantees that that the future will not be moved in memory once polling begins 
        cx: &mut Context<'_>
    ) -> Poll<Self::Output> {
        
        let pinned_fields = self.project();
        let pinned_response_future = pinned_fields.response_future;
        let pinned_sleep = pinned_fields.sleep;
        
        // checks if either the response furure is ready or the time is up

        match pinned_response_future.poll(cx) {
            Poll::Ready(res) => {
                let res = res.map_err(Into::into);
                return Poll::Ready(res)
            },
            Poll::Pending => {}
        }

        match pinned_sleep.poll(cx) {
            Poll::Ready(()) => {
                let res = Box::new(TimeoutError(()));
                return Poll::Ready(Err(res))
            },
            Poll::Pending => {}
        }

        Poll::Pending
    }
}

#[derive(Debug, Default)]
pub struct TimeoutError(());

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("request timed out") // adds some padding to the message, default is 4 spaces
    }
}

impl std::error::Error for TimeoutError {}

