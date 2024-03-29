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

//! Keyboard utilities

use crate::{print, print_err, vga_buffer::print_ok};
use conquer_once::spin::OnceCell;
use std3::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::{
    stream::{Stream, StreamExt},
    task::AtomicWaker,
};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if queue.push(scancode).is_err() {
            print_err!("[ERR] scancode queue full; dropping keyboard input\n");
        } else {
            WAKER.wake();
        }
    } else {
        print_err!("[ERR] scancode queue uninitialized\n");
    }
}

/// Keyboard presses stream
#[allow(clippy::new_without_default)]
#[unstable(feature = "rinuxcore_keyboard", issue = "none")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    /// Create a new scancode stream
    #[unstable(feature = "rinuxcore_keyboard", issue = "none")]
    pub fn new() -> Self {
        match SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100)) {
            Ok(_) => {
                unsafe {
                    if !crate::CONFIG.quiet_boot {
                        print_ok!("[OK] Scancode initialized\n");
                    };
                };
            }
            Err(_) => {
                print_err!("[ERR] ScancodeStream::new should only be called once");
                panic!("ScancodeStream::new should only be called once");
            }
        }
        ScancodeStream { _private: () }
    }
}

#[unstable(feature = "rinuxcore_keyboard", issue = "none")]
impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

/// Used if you want to see the keys pressed beeing printed to the screen
#[unstable(feature = "rinuxcore_keyboard", issue = "none")]
pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = new_keyboard();

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        print!("{}", character);
                    }
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }
}

/// Builds a new Keyboard
const fn new_keyboard() -> Keyboard<layouts::Us104Key, ScancodeSet1> {
    Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore)
}

/// Used if you want to just load the keyboard driver (fixes a bug which caused a crash if a keyboard signal was sent before the keyboard driver was loaded)
#[unstable(feature = "rinuxcore_keyboard", issue = "none")]
pub async fn init() {
    let _ = ScancodeStream::new();
    let _ = new_keyboard();
}
