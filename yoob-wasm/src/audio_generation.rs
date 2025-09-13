use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct AudioGenerationRequest {
    pub text: String,
    pub voice: String,
    pub speed: f32,
    pub additional_params: Option<Value>,
}

impl From<AudioGenerationRequest> for yoob::audio_generation::AudioGenerationRequest {
    fn from(value: AudioGenerationRequest) -> Self {
        let AudioGenerationRequest {
            text,
            voice,
            speed,
            additional_params,
        } = value;

        Self {
            text,
            voice,
            speed,
            additional_params,
        }
    }
}

impl From<yoob::audio_generation::AudioGenerationRequest> for AudioGenerationRequest {
    fn from(value: yoob::audio_generation::AudioGenerationRequest) -> Self {
        let yoob::audio_generation::AudioGenerationRequest {
            text,
            voice,
            speed,
            additional_params,
        } = value;

        Self {
            text,
            voice,
            speed,
            additional_params,
        }
    }
}
