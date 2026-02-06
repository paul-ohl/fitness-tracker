use crate::domain::types::exercise::Exercise;

pub struct WorkoutTemplate {
    pub name: String,
    pub exercises: Vec<Exercise>,
}
