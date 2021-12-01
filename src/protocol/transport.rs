use crate::{
    error,
    protocol::{codec, insim, Packet},
};
use futures::{Sink, Stream};
use pin_project::pin_project;
use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time;
use tokio_util::codec::Framed;

use super::codec::InsimCodecMode;

#[derive(Eq, PartialEq)]
pub enum TransportState {
    Disconnected,
    Connected,

    Shutdown,
}

#[pin_project]
pub struct Transport<T>
where
    T: AsyncRead + AsyncWrite,
{
    #[pin]
    inner: Framed<T, codec::InsimCodec>,
    ping_at: time::Instant,
    state: TransportState,
}

impl<T> Transport<T>
where
    T: AsyncRead + AsyncWrite,
{
    pub fn new(inner: T, codec_mode: InsimCodecMode) -> Transport<T> {
        Transport {
            inner: Framed::new(inner, codec::InsimCodec::new(codec_mode)),
            ping_at: time::Instant::now(),
            state: TransportState::Connected,
        }
    }

    pub fn shutdown(&mut self) {
        self.state = TransportState::Shutdown;
    }
}

impl<T> Stream for Transport<T>
where
    T: AsyncRead + AsyncWrite,
{
    type Item = Result<Packet, error::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        // TODO: Is this really the right way to do this?
        if self.state == TransportState::Shutdown {
            *self.project().state = TransportState::Disconnected;
            return Poll::Ready(None);
        }

        if self.state == TransportState::Disconnected {
            panic!("polled after disconnect")
        }

        if self.ping_at.elapsed().as_secs() >= 90 {
            tracing::debug!("ping timeout!");
            *self.project().state = TransportState::Disconnected;
            cx.waker().wake_by_ref();
            return Poll::Ready(Some(Err(error::Error::Timeout)));
        }

        match self.as_mut().project().inner.poll_next(cx) {
            #[allow(unused)]
            Poll::Ready(Some(Ok(frame))) => match frame {
                Packet::Tiny(insim::Tiny { reqi: 0, .. }) => {
                    tracing::debug!("ping? pong!");
                    *self.as_mut().project().ping_at = time::Instant::now();

                    // TODO: This is absolutely not the way to do this.
                    // We either using call start_send or we should at least check the result.
                    // FIXME.
                    self.as_mut()
                        .project()
                        .inner
                        .start_send(Packet::from(insim::Tiny {
                            reqi: 0,
                            subtype: 0,
                        }));
                    self.as_mut().project().inner.poll_flush(cx);

                    Poll::Ready(Some(Ok(frame)))
                }
                Packet::Version(insim::Version {
                    insimver: version, ..
                }) => {
                    if version != insim::VERSION {
                        Poll::Ready(Some(Err(error::Error::IncompatibleVersion)))
                    } else {
                        Poll::Ready(Some(Ok(frame)))
                    }
                }
                e => Poll::Ready(Some(Ok(e))),
            },
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e.into()))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T, I: Into<Packet>> Sink<I> for Transport<T>
where
    T: AsyncRead + AsyncWrite,
    I: std::fmt::Debug,
{
    type Error = Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, value: I) -> Result<(), Self::Error> {
        self.project().inner.start_send(value.into())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx)
    }
}
