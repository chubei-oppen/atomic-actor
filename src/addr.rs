use std::{any::Any, pin::Pin, sync::Arc};

use futures::{Future, FutureExt};
use tokio::sync::{
    mpsc::{error::SendError, UnboundedSender},
    oneshot::channel,
};

use crate::{
    task::{HandlerTask, Task},
    Actor, Context, Handler,
};

/// Use [`Addr`] to send messages to [`Actor`] and get the handling result.
///
/// Clones of [`Addr`] send messages to the same [`Actor`] instance.
pub struct Addr<A: Actor> {
    sender: Arc<UnboundedSender<Box<dyn Task<A>>>>,
}

impl<A: Actor> Addr<A> {
    pub(crate) fn new(sender: Arc<UnboundedSender<Box<dyn Task<A>>>>) -> Self {
        Addr { sender }
    }

    /// Sends a message to [`Actor`] and returns the handling result, if the [`Actor`] is a [`Handler`]`<M>`.
    pub fn send<M: 'static + Send>(
        &self,
        message: M,
    ) -> Result<impl Future<Output = A::Result>, SendError<M>>
    where
        A: Handler<M>,
    {
        let (sender, receiver) = channel();
        let task = Box::new(HandlerTask::new(message, handle, sender));
        self.sender.send(task).map_err(|error| {
            let message: M = *Box::<dyn Any + 'static>::downcast(error.0.into_message())
                .expect("It must be the original message that was sent back");
            SendError(message)
        })?;
        Ok(receiver
            .map(|maybe_result| maybe_result.expect("Worker thread has stopped unexpectedly")))
    }
}

fn handle<'a, A: Actor + Handler<M>, M: 'static + Send>(
    actor: &'a mut A,
    message: M,
    context: &'a mut Context<A>,
) -> Pin<Box<dyn Future<Output = A::Result> + Send + 'a>> {
    A::handle(actor, message, context)
}

impl<A: Actor> Clone for Addr<A> {
    fn clone(&self) -> Self {
        Addr {
            sender: self.sender.clone(),
        }
    }
}

impl<A: Actor> std::fmt::Debug for Addr<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Addr")
            .field("sender", &self.sender)
            .finish()
    }
}
