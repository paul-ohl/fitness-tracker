use chrono::NaiveDate;

use crate::domain::types::exercise::Exercise;

pub struct Workout {
    pub id: u64,
    pub date: NaiveDate,
    /// Mood on a scale of 1-10, where 1 is very bad and 10 is very good
    pub mood: Option<u8>,
    pub exercises: Vec<WorkoutExercise>,
}

pub struct NewWorkout {
    pub date: NaiveDate,
    /// Mood on a scale of 1-10, where 1 is very bad and 10 is very good
    pub mood: Option<u8>,
    pub exercises: Vec<NewWorkoutExercise>,
}

pub struct WorkoutExercise {
    pub exercise: Exercise,
    pub sets: Vec<WorkoutSet>,
}

pub struct NewWorkoutExercise {
    pub exercise_id: u64,
    pub sets: Vec<NewWorkoutSet>,
}

pub enum WorkoutSet {
    Weighted(WeightedSet),
    BodyweightReps(BodyweightRepSet),
    BodyweightTime(BodyweightTimeSet),
}

pub enum NewWorkoutSet {
    Weighted(NewWeightedSet),
    BodyweightReps(NewBodyweightRepSet),
    BodyweightTime(NewBodyweightTimeSet),
}

pub struct WeightedSet {
    pub id: u64,
    pub reps: u16,
    pub weight: f32,
    pub failure: bool,
}

pub struct NewWeightedSet {
    pub reps: u16,
    pub weight: f32,
    pub failure: bool,
}

pub struct BodyweightRepSet {
    pub id: u64,
    pub reps: u16,
    pub failure: bool,
}

pub struct NewBodyweightRepSet {
    pub reps: u16,
    pub failure: bool,
}

pub struct BodyweightTimeSet {
    pub id: u64,
    pub duration_seconds: u16,
    pub failure: bool,
}

pub struct NewBodyweightTimeSet {
    pub duration_seconds: u16,
    pub failure: bool,
}
