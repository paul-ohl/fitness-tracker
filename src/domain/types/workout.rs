use chrono::NaiveDate;

use crate::domain::types::exercise::Exercise;

pub struct Workout {
    pub id: u32,
    pub date: NaiveDate,
    /// Mood on a scale of 1-10, where 1 is very bad and 10 is very good
    pub mood: Option<u8>,
    pub exercises: Vec<WorkoutExercise>,
}

pub struct WorkoutExercise {
    pub exercise: Exercise,
    pub sets: Vec<WorkoutSet>,
}

pub enum WorkoutSet {
    Weighted(WeightedSet),
    BodyweightReps(BodyweightRepSet),
    BodyweightTime(BodyweightTimeSet),
}

pub struct WeightedSet {
    pub id: u32,
    pub reps: u32,
    pub weight: f32,
    pub failure: bool,
}

pub struct BodyweightRepSet {
    pub id: u32,
    pub reps: u32,
    pub failure: bool,
}

pub struct BodyweightTimeSet {
    pub id: u32,
    pub duration_seconds: u32,
    pub failure: bool,
}
