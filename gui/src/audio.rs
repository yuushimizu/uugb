use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::{Receiver, Sender};

pub struct Connection {
    sender: Sender<core::AudioFrame>,
    sample_rate: cpal::SampleRate,
    frame_counter: u64,
    _stream: cpal::Stream,
}

impl core::AudioTerminal for Connection {
    fn output(&mut self, frame: core::AudioFrame) {
        self.frame_counter += self.sample_rate.0 as u64;
        while self.frame_counter >= core::AUDIO_SAMPLE_RATE {
            self.frame_counter -= core::AUDIO_SAMPLE_RATE;
            _ = self.sender.send(frame);
        }
    }
}

#[derive(Debug)]
pub enum AudioError {
    NoDevice,
    ConfigError(cpal::DefaultStreamConfigError),
    BuildStreamError(cpal::BuildStreamError),
    PlayStreamError(cpal::PlayStreamError),
}

impl From<cpal::DefaultStreamConfigError> for AudioError {
    fn from(error: cpal::DefaultStreamConfigError) -> Self {
        Self::ConfigError(error)
    }
}

impl From<cpal::BuildStreamError> for AudioError {
    fn from(error: cpal::BuildStreamError) -> Self {
        Self::BuildStreamError(error)
    }
}

impl From<cpal::PlayStreamError> for AudioError {
    fn from(error: cpal::PlayStreamError) -> Self {
        Self::PlayStreamError(error)
    }
}

impl Connection {
    pub fn new() -> Result<Self, AudioError> {
        use cpal::SampleFormat::*;
        let device = cpal::default_host()
            .default_output_device()
            .ok_or(AudioError::NoDevice)?;
        let supported_config = device.default_output_config()?;
        let (sender, receiver) = std::sync::mpsc::channel();
        Ok(Self {
            sender,
            sample_rate: supported_config.sample_rate(),
            frame_counter: 0,
            _stream: match supported_config.sample_format() {
                F32 => create_stream::<f32>(device, supported_config, receiver),
                I16 => create_stream::<i16>(device, supported_config, receiver),
                U16 => create_stream::<u16>(device, supported_config, receiver),
            }?,
        })
    }
}

fn create_stream<T: cpal::Sample>(
    device: cpal::Device,
    supported_config: cpal::SupportedStreamConfig,
    receiver: Receiver<core::AudioFrame>,
) -> Result<cpal::Stream, AudioError> {
    let channels = supported_config.channels() as usize;
    let sample_frame = move |source| {
        cpal::Sample::from::<f32>(
            &(source as f32 / (core::MAX_AUDIO_FRAME_VOLUME as f32 / 2.0) - 1.0),
        )
    };
    let stream = device.build_output_stream(
        &supported_config.config(),
        move |data: &mut [T], _| {
            for frame in data.chunks_mut(channels) {
                if let Ok(core::AudioFrame { left, right }) = receiver.try_recv() {
                    if channels >= 2 {
                        frame[0] = sample_frame(left);
                        frame[1] = sample_frame(right);
                    }
                } else {
                    break;
                }
            }
        },
        |_| {},
    )?;
    stream.play()?;
    Ok(stream)
}

pub enum AudioOutput {
    Connected(Connection),
    None,
}

impl Default for AudioOutput {
    fn default() -> Self {
        Connection::new().map_or_else(
            |error| {
                log::warn!("Could not create the audio connection: {:?}", error);
                Self::None
            },
            |connection| Self::Connected(connection),
        )
    }
}

impl core::AudioTerminal for AudioOutput {
    fn output(&mut self, frame: core::AudioFrame) {
        match self {
            Self::Connected(connection) => connection.output(frame),
            Self::None => {}
        }
    }
}
