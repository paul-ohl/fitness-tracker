use crate::domain::types::exercise::Exercise;

pub struct WorkoutTemplate {
    pub id: u64,
    pub name: String,
    pub exercises: Vec<Exercise>,
}

pub struct NewWorkoutTemplate {
    pub name: String,
    pub exercise_ids: Vec<u64>,
}
