use std::sync::{Arc, Weak};

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{task::Task, Actor, Addr};

/// The running context.
///
/// This is currently very simple, can only be used to get a new [`Addr`].
pub struct Context<A: Actor> {
    /// Only none during running.
    actor: Option<A>,
    sender: Weak<UnboundedSender<Box<dyn Task<A>>>>,
    receiver: UnboundedReceiver<Box<dyn Task<A>>>,
}

impl<A: Actor> Context<A> {
    pub(crate) fn start(actor: A) -> Addr<A> {
        let (sender, receiver) = unbounded_channel();
        let sender = Arc::new(sender);
        let context = Context {
            actor: Some(actor),
            sender: Arc::downgrade(&sender),
            receiver,
        };
        tokio::spawn(context.run());
        Addr::new(sender)
    }

    async fn run(mut self) {
        let mut actor = self.actor.take().unwrap();
        while let Some(task) = self.receiver.recv().await {
            actor = task.run(actor, &mut self).await;
        }
    }

    /// Gets an [`Addr`] of the actor if it is still alive.
    pub fn addr(&self) -> Option<Addr<A>> {
        Weak::upgrade(&self.sender).map(Addr::new)
    }
}
