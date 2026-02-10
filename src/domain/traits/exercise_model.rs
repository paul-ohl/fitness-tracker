use crate::domain::types::exercise::{Exercise, ExerciseProgression};

#[derive(Debug)]
pub enum ExerciseModelError {
    NotFound,
    DatabaseError(String),
}

#[async_trait::async_trait]
pub trait ExerciseModel: Send + Sync {
    async fn get_exercise_by_id(&self, id: u64) -> Result<Exercise, ExerciseModelError>;
    async fn get_all_exercises(&self) -> Result<Vec<Exercise>, ExerciseModelError>;
    async fn add_exercise(&mut self, exercise: Exercise) -> Result<(), ExerciseModelError>;
    async fn update_exercise(&mut self, exercise: Exercise) -> Result<(), ExerciseModelError>;
    async fn delete_exercise(&mut self, id: u64) -> Result<(), ExerciseModelError>;

    async fn get_all_exercise_progressions(
        &self,
    ) -> Result<Vec<ExerciseProgression>, ExerciseModelError>;
    async fn get_exercise_progression(
        &self,
        exercise_id: u64,
    ) -> Result<ExerciseProgression, ExerciseModelError>;
    async fn get_exercise_progression_from_name(
        &self,
        name: &str,
    ) -> Result<ExerciseProgression, ExerciseModelError>;
    async fn add_exercise_progression(
        &mut self,
        progression: ExerciseProgression,
    ) -> Result<(), ExerciseModelError>;
}
