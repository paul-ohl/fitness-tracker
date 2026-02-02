pub struct WorkoutPlan {
    pub exercises: Vec<ExercisesPlanned>,
}

pub enum ExercisesPlanned {
    Weighted { name: String, value: f32 },
    BodyweightReps { name: String, value: i32 },
    BodyweightTime { name: String, value: i32 },
}
