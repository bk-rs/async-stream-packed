use futures_x_io::{AsyncRead, AsyncWrite};

use crate::tls::TlsClientUpgrader;
use crate::upgradable::{UpgradableAsyncStream, Upgrader};

/*
IMAP

Case1 (143):
TCP
Read(Greeting)
a1 STARTTLS
TLS
a2 LOGIN xx yy

Case2 (993):
TCP
TLS
Read(Greeting)
a1 LOGIN xx yy

e.g. https://github.com/bk-rs/async-stream-tls-upgrader/blob/master/demos/async-net/src/imap_client.rs
*/
pub type ImapClientInnerStream<S, SU> = UpgradableAsyncStream<S, SU>;

impl<S, SU> ImapClientInnerStream<S, SU>
where
    S: AsyncRead + AsyncWrite + Unpin,
    SU: TlsClientUpgrader<S> + Unpin,
    <SU as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin,
{
    pub fn with_imap_client(stream: S, tls_upgrader: SU) -> Self {
        Self::new(stream, tls_upgrader)
    }
}
