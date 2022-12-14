use crate::mp3_stream_decoder::Mp3StreamDecoder;

use anyhow::Result;
use std::{
    sync::mpsc::{self, Sender},
    thread,
};

pub struct Play {
    send: Sender<PlayMessage>,
    vol: u8,
}

enum PlayMessage {
    Play { listen_url: String, vol: u8 },
}

impl Play {
    #[allow(clippy::empty_loop)]
    pub fn try_new() -> Result<Self> {
        let (sender, reveiver) = mpsc::channel();

        thread::spawn(move || {
            let (current_url, current_vol) = loop {
                // receive the message from channel
                if let Ok(PlayMessage::Play { listen_url, vol }) = reveiver.recv() {
                    break (listen_url, vol);
                }
            };

            // rodio do something here
            let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
            // loop {
            let response = reqwest::blocking::get(&current_url).unwrap();
            let sink = rodio::Sink::try_new(&stream_handle).unwrap();
            let source = Mp3StreamDecoder::new(response).unwrap();

            sink.append(source);
            sink.set_volume(current_vol as f32 / 9_f32);

            loop {}
            // }
        });

        Ok(Play {
            send: sender,
            vol: 5,
        })
    }

    pub fn play(&self, listen_url: &str) {
        self.send
            .send(PlayMessage::Play {
                listen_url: listen_url.to_string(),
                vol: self.vol,
            })
            .unwrap();
    }
}
