use async_trait::async_trait;

use crate::{Actor, Context};

/// [`Handler`] handles a certain type of message asyncronously.
///
/// # Example
///
/// ```rust
/// use async_trait::async_trait;
/// use atomic_actor::*;
///
/// struct AddOne;
///
/// impl Actor for AddOne {
///     type Context = Context<Self>;
/// }
///
/// #[async_trait]
/// impl Handler<i32> for AddOne {
///     type Result = i32;
///
///     async fn handle(&mut self, message: i32, _: &mut Context<Self>) -> i32 {
///        message + 1
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///    let addr = AddOne.start();
///    assert_eq!(addr.send(1).unwrap().await, 2);
/// }
#[async_trait]
pub trait Handler<M: 'static + Send>: Actor {
    /// The result of the handler. Must be [`Send`].
    type Result: Send;

    /// Handles the message.
    async fn handle(&mut self, message: M, context: &mut Context<Self>) -> Self::Result;
}
