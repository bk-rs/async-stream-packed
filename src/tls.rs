use futures_io::{AsyncRead, AsyncWrite};

use crate::upgradable::Upgrader;

pub trait TlsClientUpgrader<S>: Upgrader<S> {}

impl<S> TlsClientUpgrader<S> for () where S: AsyncRead + AsyncWrite + Send + 'static {}

pub trait TlsServerUpgrader<S>: Upgrader<S> {}

impl<S> TlsServerUpgrader<S> for () where S: AsyncRead + AsyncWrite + Send + 'static {}
