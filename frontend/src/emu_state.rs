use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
};

use aliusnes::{cart::Cart, emu::Emu};

#[allow(dead_code)]
pub enum Message {
    Pause,
    Stop,
}

pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub buffer: [u16; 61184],
}

#[allow(dead_code)]
pub struct EmuState {
    message_tx: mpsc::Sender<Message>,
    pub frame_rx: rtrb::Consumer<Frame>,
    emu_thread: thread::JoinHandle<()>,
}

impl EmuState {
    pub fn new(cart: Cart) -> Self {
        let (message_tx, message_rx) = mpsc::channel::<Message>();
        let (frame_tx, frame_rx) = rtrb::RingBuffer::<Frame>::new(5);
        Self {
            message_tx,
            frame_rx,
            emu_thread: thread::spawn(move || Self::run(cart, frame_tx, message_rx)),
        }
    }

    pub fn send_message(&self, msg: Message) {
        self.message_tx.send(msg).expect("Error on sending message");
    }

    fn run(cart: Cart, mut frame_tx: rtrb::Producer<Frame>, message_rx: mpsc::Receiver<Message>) {
        let paused = AtomicBool::new(false);
        let mut emu = Emu::new(cart);
        'main: loop {
            for msg in message_rx.try_iter() {
                match msg {
                    Message::Pause => paused.store(true, Ordering::Relaxed),
                    Message::Stop => break 'main,
                }
            }

            if !paused.load(Ordering::Relaxed) {
                emu.step();
            }

            if emu.frame_ready() {
                let mut frame = Frame {
                    width: emu.frame_width(),
                    height: emu.frame_height(),
                    buffer: [0; 61184],
                };
                frame.buffer.copy_from_slice(emu.frame());

                let _ = frame_tx.push(frame);
            }
        }
    }
}
