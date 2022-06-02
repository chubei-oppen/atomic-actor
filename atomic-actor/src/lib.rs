//! An `Actor` similar to `actix::Actor` but can handle `Message`s with `async`/`await`.
//!
//! This is an `Actor` implementation that mimics the API of `actix`, with several simplications for ease of use.
//!
//! # Example
//!
//! The primary purpose of this library is to enable `Handler`s to handle `Message`s with `async fn`:
//!
//! ```rust
//! use async_trait::async_trait;
//! use atomic_actor::*;
//!
//! struct I32Actor(i32);
//!
//! impl Actor for I32Actor {
//!     type Context = Context<Self>;
//! }
//!
//! #[async_trait]
//! impl Handler<Vec<i32>> for I32Actor {
//!    type Result = i32;
//!
//!    async fn handle(&mut self, message: Vec<i32>, _: &mut Context<Self>) -> i32 {
//!        for value in message {
//!            async { self.0 += value }.await;
//!        }
//!        self.0
//!    }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let addr = I32Actor(0).start();
//!     assert_eq!(addr.send(vec![1, 2, 3]).unwrap().await, 6);
//! }
//! ```
//!
//! `actix` also allows asyncronous message handling through `ActorFuture` and friends.
//! However, the returned `ActorFuture` can not reference the `Actor`, and only gets
//! access to it on every poll, which prevents the use of `async` and `await` language features.
//!
//! # Differences from `actix`
//!
//! - `Handler::handle` is an `async fn`.
//!
//! For this to be sound, `Message`s sent to the same `Actor` must be processed sequentially, hence the name `atomic-actor`.
//!
//! - There's no `Message` trait. All types that are `'static + Send` can be used as `Message`s.
//!
//! - `Actor` has no lifecycle. Once started, it'll run until all `Addr`s are dropped.
//!
//! - `Context` is very simple, can only be used to get a new `Addr` of the actor.
//!
//! - Communication channel betwen `Addr` and the worker thread is unbounded. We may add option for using bounded channel in the future.

mod actor;
mod addr;
mod context;
mod handler;
mod task;

pub use actor::*;
pub use addr::*;
pub use context::*;
pub use handler::*;
