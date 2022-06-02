use std::{any::Any, future::Future, pin::Pin};

use async_trait::async_trait;
use tokio::sync::oneshot::Sender;

use crate::{Actor, Context};

pub type HandlerFn<A, M, R> =
    for<'a> fn(&'a mut A, M, &'a mut Context<A>) -> Pin<Box<dyn Future<Output = R> + Send + 'a>>;

#[async_trait]
pub trait Task<A: Actor>: Send {
    async fn run(self: Box<Self>, actor: A, context: &mut Context<A>) -> A;

    fn into_message(self: Box<Self>) -> Box<dyn Any>;
}

pub struct HandlerTask<A: Actor, M, R> {
    message: M,
    function: HandlerFn<A, M, R>,
    sender: Sender<R>,
}

impl<A: Actor, M: 'static, R: 'static> HandlerTask<A, M, R> {
    pub fn new(message: M, function: HandlerFn<A, M, R>, sender: Sender<R>) -> Self {
        Self {
            message,
            function,
            sender,
        }
    }
}

#[async_trait]
impl<A: Actor, M: 'static + Send, R: 'static + Send> Task<A> for HandlerTask<A, M, R> {
    async fn run(self: Box<Self>, mut actor: A, context: &mut Context<A>) -> A {
        let Self {
            message,
            function,
            sender,
        } = *self;
        let result = function(&mut actor, message, context).await;
        // If sending failed, it means the receiver has already been dropped.
        // In that case no one cares about the result anyway.
        let _ = sender.send(result);
        actor
    }

    fn into_message(self: Box<Self>) -> Box<dyn Any> {
        Box::new(self.message)
    }
}
