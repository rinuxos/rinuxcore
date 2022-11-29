//
// MIT License
//
// Copyright (c) 2022 AtomicGamer9523
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//! Tasking utilities

use std3::boxed::Box;
use std3::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll},
};

#[unstable(feature = "rinuxcore_task", issue = "none")]
pub mod executor;
#[unstable(feature = "rinuxcore_keyboard", issue = "none")]
pub mod keyboard;
#[unstable(feature = "rinuxcore_task", issue = "none")]
pub mod simple_executor;

/// Task, includes a future and a task ID
#[unstable(feature = "rinuxcore_task", issue = "none")]
pub struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    /// Create a new task
    #[unstable(feature = "rinuxcore_task", issue = "none")]
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }

    fn int(&self) -> u64 {
        self.id.0
    }
}
use std3::fmt::{Debug, Formatter, Error as FmtError};
impl Debug for Task {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "Task {{ id: {} }}", self.id.0)
    }
}
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.int() == other.int()
    }
}
impl Eq for Task {}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std3::cmp::Ordering> {
        self.int().partial_cmp(&other.int())
    }
    fn ge(&self, other: &Self) -> bool {
        self.int() >= other.int()
    }
    fn gt(&self, other: &Self) -> bool {
        self.int() > other.int()
    }
    fn le(&self, other: &Self) -> bool {
        self.int() <= other.int()
    }
    fn lt(&self, other: &Self) -> bool {
        self.int() < other.int()
    }
}
impl Ord for Task {
    fn cmp(&self, other: &Self) -> std3::cmp::Ordering {
        self.int().cmp(&other.int())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}
