use portaudio::{Continue, PortAudio, StreamParameters, DuplexStreamSettings,
                DuplexStreamCallbackArgs, Stream, NonBlocking, Duplex};
use portaudio::Error as PaError;

use std::error::Error;

const DEFAULT_SAMPLE_RATE: f64 = 44_100.0;
const DEFAULT_FRAME_COUNT: u32 = 256;
const DEFAULT_CHANNEL_COUNT: i32 = 2;
const DEFAULT_INTERLEAVED: bool = true;

type ParamResult = Result<StreamParameters<f32>, PaError>;
type PlayerStream = Stream<NonBlocking, Duplex<f32, f32>>;

#[derive(Clone, Debug)]
pub struct PlayerError(String);

impl ::std::fmt::Display for PlayerError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Error for PlayerError {
    fn description(&self) -> &str {
        match *self {
            PlayerError(s) => &*s,
        }
    }
}

pub type PlayerCallback = Fn(&[f32], &[f32]) -> ();

pub struct Player {
    pa: PortAudio,
    settings: DuplexStreamSettings<f32, f32>,
    stream: Option<PlayerStream>,
}

impl Player {
    pub fn default() -> Result<Self, PlayerError> {
        PortAudio::new()
            .and_then(|pa| {
                input_params(&pa, DEFAULT_CHANNEL_COUNT, DEFAULT_INTERLEAVED)
                    .and_then(|input_params| {
                        output_params(&pa, DEFAULT_CHANNEL_COUNT, DEFAULT_INTERLEAVED)
                            .map(|output_params| {
                                let settings = DuplexStreamSettings::new(
                                    input_params,
                                    output_params,
                                    DEFAULT_SAMPLE_RATE,
                                    DEFAULT_FRAME_COUNT,
                                );

                                Self {
                                    pa,
                                    settings,
                                    stream: None,
                                }
                            })
                    })
            })
            .map_err(|e| PlayerError(e.description().to_owned()))
    }

    pub fn play(&mut self, callback: &PlayerCallback) -> Result<&Self, PlayerError> {
        let stream_callback = move |DuplexStreamCallbackArgs {
                                   in_buffer,
                                   out_buffer,
                                   frames,
                                   ..
                               }| {
            callback(&in_buffer, &out_buffer);
            Continue
        };

        self.pa
            .open_non_blocking_stream(self.settings, stream_callback)
            .and_then(|stream| stream.start().map(|_| stream))
            .map(|stream| self.stream = Some(stream))
            .map(|_| &*self)
            .map_err(|e| PlayerError(e.description().to_owned()))
    }
}

fn input_params(pa: &PortAudio, channels: i32, interleaved: bool) -> ParamResult {
    pa.default_input_device().and_then(|default_input| {
        pa.device_info(default_input).map(|input_info| {
            StreamParameters::new(
                default_input,
                channels,
                interleaved,
                input_info.default_low_input_latency,
            )
        })
    })
}

fn output_params(pa: &PortAudio, channels: i32, interleaved: bool) -> ParamResult {
    pa.default_output_device().and_then(|default_output| {
        pa.device_info(default_output).map(|output_info| {
            StreamParameters::new(
                default_output,
                channels,
                interleaved,
                output_info.default_low_output_latency,
            )
        })
    })
}
