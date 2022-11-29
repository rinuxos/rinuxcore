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

//! Executor for running tasks

use std3::__reexports::x86_64;
use super::{Task, TaskId};
use std3::{collections::BTreeMap, sync::Arc, task::Wake};
use std3::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;

/// Executor for running tasks.
/// Make sure you enable the feature:
/// ```rust
/// #![feature(rinuxcore_task)]
/// ```
/// Basic usage:
/// ```rust
/// use rinuxcore::{
///     println,
///     task::{executor::Executor, Task},
///     BootInfo
/// };
///
/// #[rinuxcore::main]
/// fn kernel_main(boot_info: &'static BootInfo) -> ! {
///     rinuxcore::init(&boot_info);
/// 
///     let mut executor = Executor::new();
///     executor.spawn(Task::new(main()));
///     executor.run()
/// }
///
/// async fn main() {
///     println!("Hello World");
/// }
/// ```
#[unstable(feature = "rinuxcore_task", issue = "none")]
#[derive(Debug)]
pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}
impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}
impl Executor {
    /// Create a new executor instance.
    /// Has a default size of 100 tasks.
    #[unstable(feature = "rinuxcore_task", issue = "none")]
    pub fn new() -> Self {
        Executor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(100)),
            waker_cache: BTreeMap::new(),
        }
    }

    /// Spawn a new task
    #[unstable(feature = "rinuxcore_task", issue = "none")]
    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task.id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        self.task_queue.push(task_id).expect("queue full");
    }

    /// Run all tasks in the executor
    #[unstable(feature = "rinuxcore_task", issue = "none")]
    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }

    /// Runs first task in the executor's queue.
    /// Useful when you want to run initailization tasks
    /// 
    /// Example:
    /// 
    /// ```rust
    /// #![no_std]
    /// #![no_main]
    /// #![feature(custom_test_frameworks)]
    /// #![test_runner(rinuxcore::test_runner)]
    /// #![reexport_test_harness_main = "test_main"]
    ///
    /// // Rinuxcore features
    /// #![feature(rinuxcore_task)]
    /// #![feature(rinuxcore_keyboard)]
    ///
    ///
    /// use rinuxcore::{
    ///     task::{executor::Executor, Task},
    ///     BootInfo,
    ///     println,
    ///     std3
    /// };
    ///
    /// #[rinuxcore::main]
    /// fn kernel_main(boot_info: &'static BootInfo) -> ! {
    ///     rinuxcore::init(&boot_info);// Initializes Rinux
    ///     let mut executor = Executor::new();// Creates new Task Executor
    ///
    ///     // Built-in keyboard driver, requires "rinuxcore_keyboard" feature
    ///     executor.spawn(Task::new(rinuxcore::task::keyboard::init()));
    ///     executor.run_first_task_in_queue();
    ///     executor.spawn(Task::new(main()));
    ///     executor.run()
    /// }
    ///
    /// async fn main() {
    ///     println!("Hello World");
    /// }
    ///
    /// #[panic_handler]
    /// fn panic(info: &std3::panic::PanicInfo) -> ! {
    ///     rinuxcore::print_err!("{}", info);
    ///     rinuxcore::hlt_loop();
    /// }
    /// ```
    #[unstable(feature = "rinuxcore_task", issue = "none")]
    pub fn run_first_task_in_queue(&mut self) -> () {
        let task_id = self.task_queue.pop().expect("queue empty");
        self.run_task(task_id);
    }


    fn run_task(&mut self, task_id: TaskId) -> () {
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;
        let task = match tasks.get_mut(&task_id) {
            Some(task) => task,
            None => return,
        };
        let waker = waker_cache
            .entry(task_id)
            .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
        let mut context = Context::from_waker(waker);
        match task.poll(&mut context) {
            Poll::Ready(()) => {
                tasks.remove(&task_id);
                waker_cache.remove(&task_id);
            }
            Poll::Pending => {}
        }
    }

    fn run_ready_tasks(&mut self) {
        while let Ok(task_id) = self.task_queue.pop() {
            self.run_task(task_id);
        }
    }

    fn sleep_if_idle(&self) {
        use x86_64::instructions::interrupts::{self, enable_and_hlt};
        interrupts::disable();
        if self.task_queue.is_empty() {
            enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }
}

struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    #[allow(clippy::new_ret_no_self)]
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            task_id,
            task_queue,
        }))
    }

    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("task_queue full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}
