use futures_x_io::{AsyncRead, AsyncWrite};

use crate::gradable::Downgrader;
use crate::upgradable::Upgrader;

pub trait HttpTunnelClientGrader<S>: Upgrader<S> + Downgrader<S> {}

impl<S> HttpTunnelClientGrader<S> for () where S: AsyncRead + AsyncWrite + Unpin + Send + 'static {}

pub trait HttpTunnelServerGrader<S>: Upgrader<S> + Downgrader<S> {}

impl<S> HttpTunnelServerGrader<S> for () where S: AsyncRead + AsyncWrite + Unpin + Send + 'static {}
