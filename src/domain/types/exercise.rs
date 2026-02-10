pub struct ExerciseProgression {
    pub name: String,
    pub progression: Vec<Exercise>,
}

pub struct Exercise {
    pub id: u64,
    pub name: String,
    pub exercise_type: ExerciseType,
}

pub struct NewExercise {
    pub name: String,
    pub exercise_type: ExerciseType,
}

pub enum ExerciseType {
    Weighted { goal_weight: f32 },
    BodyweightReps { goal_reps: u16 },
    BodyweightTime { goal_duration_seconds: u16 },
}
