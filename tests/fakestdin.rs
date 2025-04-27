use std::{io::{self}, task::Poll};

use tokio::io::AsyncRead;

pub struct FakeStdin {
    pub input: &'static [u8],
    pub position: usize,
}

impl FakeStdin {
    pub fn new(input: &'static [u8]) -> Self {
        Self { input, position: 0 }
    }
}

impl AsyncRead for FakeStdin {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        let me = self.get_mut();

        // If we've read all the data, return Poll::Ready with Ok
        if me.position >= me.input.len() {
            return Poll::Ready(Ok(()));
        }

        // Calculate how many bytes we can read
        let remaining = &me.input[me.position..];
        let amt = std::cmp::min(remaining.len(), buf.remaining());

        // Copy the bytes into the buffer
        buf.put_slice(&remaining[..amt]);

        // Update our position
        me.position += amt;

        // Always return Ready since we're reading from memory
        Poll::Ready(Ok(()))
    }
}
