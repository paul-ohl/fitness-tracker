use chrono::NaiveDate;

pub struct Workout {
    pub date: NaiveDate,
    pub mood: Option<u8>,
    pub exercises: Vec<Exercise>,
}

pub struct Exercise {
    pub name: String,
    pub sets: ExerciseSet,
}

pub enum ExerciseSet {
    Weighted(Vec<WeightedSet>),
    BodyweightReps(Vec<BodyweightRepSet>),
    BodyweightTime(Vec<BodyweightTimeSet>),
}

pub struct WeightedSet {
    pub reps: u32,
    pub weight: f32,
    pub failure: bool,
}

pub struct BodyweightRepSet {
    pub reps: u32,
    pub failure: bool,
}

pub struct BodyweightTimeSet {
    pub duration_seconds: u32,
}
