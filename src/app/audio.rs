use cpal::traits::{DeviceTrait, HostTrait};
use std::sync::mpsc::{Receiver, Sender};

pub struct AudioOutput {
    sender: Sender<core::AudioFrame>,
    sample_rate: cpal::SampleRate,
    frame_counter: u64,
    stream: cpal::Stream,
}

impl core::AudioTerminal for AudioOutput {
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
    StreamError(cpal::BuildStreamError),
}

impl From<cpal::DefaultStreamConfigError> for AudioError {
    fn from(error: cpal::DefaultStreamConfigError) -> Self {
        Self::ConfigError(error)
    }
}

impl From<cpal::BuildStreamError> for AudioError {
    fn from(error: cpal::BuildStreamError) -> Self {
        Self::StreamError(error)
    }
}

impl AudioOutput {
    pub fn new() -> Result<AudioOutput, AudioError> {
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
            stream: match supported_config.sample_format() {
                F32 => create_stream::<f32>(device, supported_config, receiver),
                I16 => create_stream::<i16>(device, supported_config, receiver),
                U16 => create_stream::<u16>(device, supported_config, receiver),
            }?,
        })
    }
}

fn sample_frame<T: cpal::Sample>(source: u16) -> T {
    cpal::Sample::from::<f32>(&((source as f32) / (65536f32 / 2f32)))
}

fn create_stream<T: cpal::Sample>(
    device: cpal::Device,
    supported_config: cpal::SupportedStreamConfig,
    receiver: Receiver<core::AudioFrame>,
) -> Result<cpal::Stream, AudioError> {
    let channels = supported_config.channels() as usize;
    Ok(device.build_output_stream(
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
    )?)
}
