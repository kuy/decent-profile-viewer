use include_dir::{include_dir, Dir};
use once_cell::sync::Lazy;

use crate::lib::parser::{profile, Command, ProfileType, Prop, PumpType, Step, TransitionType};

static PROFILES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/profiles");

pub static PROFILES: Lazy<Vec<Preset>> = Lazy::new(|| {
    let mut items = vec![];
    for file in PROFILES_DIR.files() {
        let mut preset = Preset {
            name: file
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            ..Default::default()
        };

        let content = file.contents_utf8().unwrap();
        let (_, commands) = profile(content.as_bytes()).expect("Failed to parse");
        let profile = Profile(commands);

        // NOTE: Support only "Advanced Profile" now
        if !profile.is_profile_type(ProfileType::Settings2C) {
            continue;
        }

        let name = &preset.name;
        preset.title = profile
            .title()
            .unwrap_or_else(move || panic!("Failed to get `profile_title` {}", name));
        preset.notes = profile.notes().expect("Failed to get `profile_notes`");
        preset.data = profile
            .advanced_shot()
            .expect("Failed to get `advanced_shot`");

        items.push(preset);
    }
    items.sort_by(|a, b| a.title.cmp(&b.title));
    items
});

#[derive(Clone, Debug)]
pub struct Profile(pub Vec<Command>);

impl Profile {
    pub fn is_profile_type(&self, ty: ProfileType) -> bool {
        self.0.iter().any(|cmd| match cmd {
            Command::SettingsProfileType(pt) => pt == &ty,
            _ => false,
        })
    }

    pub fn title(&self) -> Option<String> {
        self.0.iter().find_map(|cmd| match cmd {
            Command::ProfileTitle(title) => Some(title.clone()),
            _ => None,
        })
    }

    pub fn notes(&self) -> Option<String> {
        self.0.iter().find_map(|cmd| match cmd {
            Command::ProfileNotes(note) => Some(note.clone()),
            _ => None,
        })
    }

    pub fn advanced_shot(&self) -> Option<String> {
        self.0.iter().find_map(|cmd| match cmd {
            Command::AdvancedShot(data) => Some(format!("{}\n", data.clone())),
            _ => None,
        })
    }
}

#[derive(Clone, Default)]
pub struct Preset {
    pub name: String,
    pub title: String,
    pub notes: String,
    pub data: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnalyzedProfile {
    pub temperature: PositionList,
    pub pressure: PositionList,
    pub flow: PositionList,
    pub elapsed_time: f64,
}

pub type PositionList = Vec<(f64, f64, f64, f64)>;

pub fn analyze(steps: &[Step]) -> AnalyzedProfile {
    let mut temperature_pos: PositionList = vec![];
    let mut last_temperature_pos: Option<(f64, f64, f64, f64)> = None;

    let mut pressure_pos: PositionList = vec![];
    let mut last_pressure_pos: Option<(f64, f64, f64, f64)> = None;

    let mut flow_pos: PositionList = vec![];
    let mut last_flow_pos: Option<(f64, f64, f64, f64)> = None;

    let mut elapsed_time = 0f64;
    let mut prev_pump = None;
    let mut prev_exit_flow: Option<f32> = None;

    for step in steps.iter() {
        let duration = step.seconds() as f64;
        let transition = step.transition();
        let pump = step.pump();

        for prop in step.0.iter() {
            match prop {
                Prop::Temperature(t) => {
                    let t = *t as f64;
                    if let Some((.., prev_t)) = last_temperature_pos {
                        temperature_pos.push((elapsed_time, prev_t, elapsed_time, t));
                        temperature_pos.push((elapsed_time, t, elapsed_time + duration, t));
                    } else {
                        temperature_pos.push((elapsed_time, t, elapsed_time + duration, t));
                    }
                    last_temperature_pos = Some(*temperature_pos.last().unwrap());
                }
                Prop::Pressure(v) => {
                    if pump == PumpType::Pressure {
                        if let (Some(PumpType::Flow), Some((.., px, py))) =
                            (prev_pump, last_flow_pos)
                        {
                            flow_pos.push((px, py, px, 0.));
                            last_flow_pos = Some(*flow_pos.last().unwrap());
                        }

                        let v = *v as f64;
                        if let Some((.., prev_v)) = last_pressure_pos {
                            match transition {
                                TransitionType::Fast => {
                                    pressure_pos.push((elapsed_time, prev_v, elapsed_time, v));
                                    pressure_pos.push((
                                        elapsed_time,
                                        v,
                                        elapsed_time + duration,
                                        v,
                                    ));
                                }
                                TransitionType::Smooth => {
                                    pressure_pos.push((
                                        elapsed_time,
                                        prev_v,
                                        elapsed_time + duration,
                                        v,
                                    ));
                                }
                            }
                        } else {
                            pressure_pos.push((elapsed_time, 0., elapsed_time, v));
                            pressure_pos.push((elapsed_time, v, elapsed_time + duration, v));
                        }

                        last_pressure_pos = Some(*pressure_pos.last().unwrap());
                    }
                }
                Prop::Flow(v) => {
                    if pump == PumpType::Flow {
                        if let (Some(PumpType::Pressure), Some((.., px, py))) =
                            (prev_pump, last_pressure_pos)
                        {
                            pressure_pos.push((px, py, px, 0.));
                            last_pressure_pos = Some(*pressure_pos.last().unwrap());
                        }

                        let v = *v as f64;
                        if let Some((.., prev_v)) = last_flow_pos {
                            let mut prev_v = prev_v;
                            if let Some(f) = prev_exit_flow {
                                flow_pos.push((elapsed_time, prev_v, elapsed_time, f as f64));
                                prev_v = f as f64;
                            }

                            match transition {
                                TransitionType::Fast => {
                                    flow_pos.push((elapsed_time, prev_v, elapsed_time, v));
                                    flow_pos.push((elapsed_time, v, elapsed_time + duration, v));
                                }
                                TransitionType::Smooth => {
                                    flow_pos.push((
                                        elapsed_time,
                                        prev_v,
                                        elapsed_time + duration,
                                        v,
                                    ));
                                }
                            }
                        } else {
                            flow_pos.push((elapsed_time, 0., elapsed_time, v));
                            flow_pos.push((elapsed_time, v, elapsed_time + duration, v));
                        }

                        last_flow_pos = Some(*flow_pos.last().unwrap());
                    }
                }
                _ => (),
            }
        }

        elapsed_time += duration;
        prev_pump = Some(pump);
        prev_exit_flow = step.exit_flow();
    }

    AnalyzedProfile {
        temperature: temperature_pos,
        pressure: pressure_pos,
        flow: flow_pos,
        elapsed_time,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_is_profile_type() {
        let profile = Profile(vec![Command::SettingsProfileType(ProfileType::Settings2C)]);
        assert_eq!(profile.is_profile_type(ProfileType::Settings2C), true);
    }

    #[test]
    fn test_profile_is_profile_type_failed() {
        let profile = Profile(vec![Command::SettingsProfileType(ProfileType::Settings1)]);
        assert_eq!(profile.is_profile_type(ProfileType::Settings2C), false);

        let profile = Profile(vec![Command::Author("Trunk".into())]);
        assert_eq!(profile.is_profile_type(ProfileType::Settings2C), false);
    }
}
