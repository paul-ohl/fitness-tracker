pub struct ExerciseProgression {
    pub progression: Vec<Exercise>,
}

pub struct Exercise {
    pub id: String,
    pub name: String,
    pub exercise_type: ExerciseType,
}

pub enum ExerciseType {
    Weighted { goal_weight: f32 },
    BodyweightReps { goal_reps: u32 },
    BodyweightTime { goal_duration_seconds: u32 },
}
