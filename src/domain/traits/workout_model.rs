use crate::domain::types::workout::{NewWorkout, Workout};

#[derive(Debug)]
pub enum WorkoutModelError {
    NotFound,
    DatabaseError(String),
}

#[async_trait::async_trait]
pub trait WorkoutModel: Send + Sync {
    async fn create_workout(&mut self, workout: NewWorkout) -> Result<u64, WorkoutModelError>;
    async fn get_workout(&self, workout_id: u64) -> Result<Workout, WorkoutModelError>;
    async fn get_all_workouts(&self) -> Result<Vec<Workout>, WorkoutModelError>;
    async fn update_workout(&mut self, workout: Workout) -> Result<(), WorkoutModelError>;
    async fn delete_workout(&mut self, workout_id: u64) -> Result<(), WorkoutModelError>;
}
