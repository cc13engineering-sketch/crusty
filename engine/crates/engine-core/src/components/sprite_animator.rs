use super::SchemaInfo;

/// Defines a named animation clip: a sequence of sprite frame indices with timing.
#[derive(Clone, Debug)]
pub struct AnimationClip {
    pub name: String,
    /// Frame indices into the sprite sheet.
    pub frames: Vec<u32>,
    /// Duration of each frame in seconds.
    pub frame_duration: f64,
    /// Whether this clip loops.
    pub looping: bool,
}

impl AnimationClip {
    pub fn new(name: &str, frames: Vec<u32>, frame_duration: f64, looping: bool) -> Self {
        Self {
            name: name.to_string(),
            frames,
            frame_duration,
            looping,
        }
    }

    pub fn total_duration(&self) -> f64 {
        self.frames.len() as f64 * self.frame_duration
    }
}

/// Per-entity sprite animation controller.
/// Manages multiple named clips, tracks current clip, frame index, and elapsed time.
#[derive(Clone, Debug)]
pub struct SpriteAnimator {
    pub clips: Vec<AnimationClip>,
    pub current_clip: String,
    pub current_frame_index: usize,
    pub elapsed: f64,
    pub playing: bool,
    /// The sprite sheet tile index to display (output of the animator).
    pub current_tile: u32,
    /// Set true on the frame a clip finishes (non-looping only).
    pub just_finished: bool,
    /// Speed multiplier (1.0 = normal).
    pub speed: f64,
}

impl SpriteAnimator {
    pub fn new(initial_clip: &str) -> Self {
        Self {
            clips: Vec::new(),
            current_clip: initial_clip.to_string(),
            current_frame_index: 0,
            elapsed: 0.0,
            playing: true,
            current_tile: 0,
            just_finished: false,
            speed: 1.0,
        }
    }

    pub fn add_clip(&mut self, clip: AnimationClip) {
        self.clips.push(clip);
    }

    pub fn with_clip(mut self, clip: AnimationClip) -> Self {
        self.add_clip(clip);
        self
    }

    /// Switch to a named clip. Resets elapsed time and frame index.
    pub fn play(&mut self, clip_name: &str) {
        if self.current_clip != clip_name {
            self.current_clip = clip_name.to_string();
            self.current_frame_index = 0;
            self.elapsed = 0.0;
            self.playing = true;
            self.just_finished = false;
        }
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }

    pub fn resume(&mut self) {
        self.playing = true;
    }

    /// Get the current AnimationClip, if any.
    pub fn get_clip(&self) -> Option<&AnimationClip> {
        self.clips.iter().find(|c| c.name == self.current_clip)
    }

    /// Returns true if the current clip is playing and hasn't finished.
    pub fn is_playing(&self) -> bool {
        self.playing && !self.just_finished
    }

    /// Get the clip names available.
    pub fn clip_names(&self) -> Vec<&str> {
        self.clips.iter().map(|c| c.name.as_str()).collect()
    }
}

impl SchemaInfo for SpriteAnimator {
    fn schema_name() -> &'static str { "SpriteAnimator" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "description": "Sprite animation controller with named clips",
            "properties": {
                "current_clip": { "type": "string" },
                "clips": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": { "type": "string" },
                            "frames": { "type": "array", "items": { "type": "integer" } },
                            "frame_duration": { "type": "number" },
                            "looping": { "type": "boolean" }
                        }
                    }
                },
                "speed": { "type": "number", "default": 1.0 }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_animator() {
        let anim = SpriteAnimator::new("idle");
        assert_eq!(anim.current_clip, "idle");
        assert!(anim.playing);
        assert_eq!(anim.current_frame_index, 0);
        assert_eq!(anim.speed, 1.0);
    }

    #[test]
    fn add_and_get_clip() {
        let mut anim = SpriteAnimator::new("walk");
        anim.add_clip(AnimationClip::new("walk", vec![0, 1, 2, 3], 0.1, true));
        assert!(anim.get_clip().is_some());
        assert_eq!(anim.get_clip().unwrap().frames.len(), 4);
    }

    #[test]
    fn with_clip_builder() {
        let anim = SpriteAnimator::new("run")
            .with_clip(AnimationClip::new("run", vec![4, 5, 6, 7], 0.08, true))
            .with_clip(AnimationClip::new("idle", vec![0], 1.0, true));
        assert_eq!(anim.clips.len(), 2);
    }

    #[test]
    fn play_switches_clip() {
        let mut anim = SpriteAnimator::new("idle")
            .with_clip(AnimationClip::new("idle", vec![0], 1.0, true))
            .with_clip(AnimationClip::new("walk", vec![1, 2, 3, 4], 0.1, true));
        anim.elapsed = 0.5;
        anim.current_frame_index = 2;

        anim.play("walk");
        assert_eq!(anim.current_clip, "walk");
        assert_eq!(anim.current_frame_index, 0);
        assert_eq!(anim.elapsed, 0.0);
    }

    #[test]
    fn play_same_clip_no_reset() {
        let mut anim = SpriteAnimator::new("idle")
            .with_clip(AnimationClip::new("idle", vec![0, 1], 0.5, true));
        anim.elapsed = 0.3;
        anim.current_frame_index = 1;

        anim.play("idle"); // same clip — no reset
        assert_eq!(anim.elapsed, 0.3);
        assert_eq!(anim.current_frame_index, 1);
    }

    #[test]
    fn stop_and_resume() {
        let mut anim = SpriteAnimator::new("idle");
        assert!(anim.playing);
        anim.stop();
        assert!(!anim.playing);
        anim.resume();
        assert!(anim.playing);
    }

    #[test]
    fn clip_names_lists_all() {
        let anim = SpriteAnimator::new("idle")
            .with_clip(AnimationClip::new("idle", vec![0], 1.0, true))
            .with_clip(AnimationClip::new("walk", vec![1, 2], 0.1, true))
            .with_clip(AnimationClip::new("attack", vec![3, 4, 5], 0.05, false));
        let names = anim.clip_names();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"idle"));
        assert!(names.contains(&"walk"));
        assert!(names.contains(&"attack"));
    }

    #[test]
    fn total_duration() {
        let clip = AnimationClip::new("test", vec![0, 1, 2, 3], 0.25, true);
        assert!((clip.total_duration() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn get_clip_returns_none_for_missing() {
        let anim = SpriteAnimator::new("nonexistent");
        assert!(anim.get_clip().is_none());
    }

    #[test]
    fn is_playing_reflects_state() {
        let mut anim = SpriteAnimator::new("idle");
        assert!(anim.is_playing());
        anim.just_finished = true;
        assert!(!anim.is_playing());
        anim.just_finished = false;
        anim.stop();
        assert!(!anim.is_playing());
    }
}
