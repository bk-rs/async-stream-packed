use futures_x_io::{AsyncRead, AsyncWrite};

use crate::upgradable::Upgrader;

pub trait TlsClientUpgrader<S>: Upgrader<S> {}

impl<S> TlsClientUpgrader<S> for () where S: AsyncRead + AsyncWrite + Unpin + Send + 'static {}

pub trait TlsServerUpgrader<S>: Upgrader<S> {}

impl<S> TlsServerUpgrader<S> for () where S: AsyncRead + AsyncWrite + Unpin + Send + 'static {}
