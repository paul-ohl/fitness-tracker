use std::{fs, path::PathBuf};

use serde::Deserialize;

use crate::domain::types::goal::Goal;

#[derive(Debug, Clone, Deserialize)]
struct GoalRead {
    pub name: String,
    pub target: ExerciseTarget,
}

#[derive(Debug, Clone, Deserialize)]
enum ExerciseTarget {
    Weighted {
        target_weight: f32,
        target_reps: Option<u32>,
        target_sets: Option<u32>,
    },
    BodyweightReps {
        progression: Vec<BodyweightRepsProgression>,
    },
    BodyweightTime {
        progression: Vec<BodyweightTimeProgression>,
    },
}

#[derive(Debug, Clone, Deserialize)]
struct BodyweightRepsProgression {
    pub name: String,
    pub target: BodyweightRepsTarget,
}

#[derive(Debug, Clone, Deserialize)]
struct BodyweightRepsTarget {
    pub target_reps: u32,
    pub target_sets: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
struct BodyweightTimeProgression {
    pub name: String,
    pub target: BodyweightTimeTarget,
}

#[derive(Debug, Clone, Deserialize)]
struct BodyweightTimeTarget {
    pub target_duration_seconds: u32,
    pub target_sets: Option<u32>,
}

impl From<GoalRead> for Goal {
    fn from(value: GoalRead) -> Self {
        let target = match value.target {
            ExerciseTarget::Weighted {
                target_weight,
                target_reps,
                target_sets,
            } => crate::domain::types::goal::ExerciseTarget::Weighted {
                target_weight,
                target_reps,
                target_sets,
            },
            ExerciseTarget::BodyweightReps { progression } => {
                crate::domain::types::goal::ExerciseTarget::BodyweightReps {
                    progression: progression
                        .into_iter()
                        .map(|p| crate::domain::types::goal::BodyweightRepsProgression {
                            name: p.name,
                            target: crate::domain::types::goal::BodyweightRepsTarget {
                                target_reps: p.target.target_reps,
                                target_sets: p.target.target_sets,
                            },
                        })
                        .collect(),
                }
            }
            ExerciseTarget::BodyweightTime { progression } => {
                crate::domain::types::goal::ExerciseTarget::BodyweightTime {
                    progression: progression
                        .into_iter()
                        .map(|p| crate::domain::types::goal::BodyweightTimeProgression {
                            name: p.name,
                            target: crate::domain::types::goal::BodyweightTimeTarget {
                                target_duration_seconds: p.target.target_duration_seconds,
                                target_sets: p.target.target_sets,
                            },
                        })
                        .collect(),
                }
            }
        };
        Goal {
            name: value.name,
            target,
        }
    }
}

pub fn read_goals(path_to_config: PathBuf) -> Vec<Goal> {
    let data = fs::read_to_string(path_to_config).expect("Could not read goal file");
    let goal_read: Vec<GoalRead> =
        serde_yml::from_str(&data).expect("Could not convert yml to data structure");
    goal_read.into_iter().map(|g| g.into()).collect()
}
