use crate::{Addr, Context};

/// [`Actor`] reacts to messages. See [`Handler`](crate::Handler) for how to define the reaction.
pub trait Actor: 'static + Sized + Send {
    /// The [`Actor`]'s running context. Should be [`Context`]`<Self>`.
    type Context;

    /// Starts running this [`Actor`] and returns an [`Addr`] to it.
    ///
    /// The [`Addr`] can be cloned for sending messages to the [`Actor`] from different threads.
    fn start(self) -> Addr<Self> {
        Context::start(self)
    }
}
