use futures_x_io::{AsyncRead, AsyncWrite};

use crate::tls::TlsClientUpgrader;
use crate::upgradable::{UpgradableAsyncStream, Upgrader};

/*
SMTP

Case1 (587):
TCP
Read(Greeting)
EHLO RUST
STARTTLS
TLS
EHLO RUST
AUTH LOGIN

Case2 (465):
TCP
TLS
Read(Greeting)
EHLO RUST
AUTH LOGIN

e.g. https://github.com/bk-rs/async-stream-tls-upgrader/blob/master/demos/async-net/src/smtp_client.rs
*/
pub type SmtpClientInnerStream<S, SU> = UpgradableAsyncStream<S, SU>;

impl<S, SU> SmtpClientInnerStream<S, SU>
where
    S: AsyncRead + AsyncWrite + Unpin,
    SU: TlsClientUpgrader<S> + Unpin,
    <SU as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin,
{
    pub fn with_smtp_client(stream: S, tls_upgrader: SU) -> Self {
        Self::new(stream, tls_upgrader)
    }
}
