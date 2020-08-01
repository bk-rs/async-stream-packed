use crate::tls::TlsClientUpgrader;
use crate::upgradable::UpgradableAsyncStream;

/*
IMAP

openssl s_client -connect imap.126.com:143 -crlf -starttls imap
openssl s_client -connect imap.gmail.com:993 -crlf
*/
pub type ImapClientInnerStream<S, SU> = UpgradableAsyncStream<S, SU>;

impl<S, SU> ImapClientInnerStream<S, SU>
where
    SU: TlsClientUpgrader<S>,
{
    pub fn with_imap_tcp_stream_and_tls_upgrader(tcp_stream: S, tls_upgrader: SU) -> Self {
        Self::new(tcp_stream, tls_upgrader)
    }
}

/*
SMTP

openssl s_client -connect smtp.gmail.com:587 -crlf -starttls smtp
openssl s_client -connect smtp.gmail.com:465 -crlf
*/
pub type SmtpClientInnerStream<S, SU> = UpgradableAsyncStream<S, SU>;

impl<S, SU> SmtpClientInnerStream<S, SU>
where
    SU: TlsClientUpgrader<S>,
{
    pub fn with_smtp_tcp_stream_and_tls_upgrader(tcp_stream: S, tls_upgrader: SU) -> Self {
        Self::new(tcp_stream, tls_upgrader)
    }
}
