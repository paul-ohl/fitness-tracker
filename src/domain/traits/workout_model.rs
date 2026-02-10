use crate::domain::types::workout::Workout;

pub trait WorkoutModel {
    fn create_workout(&self, workout: &Workout) -> Result<(), String>;
    fn get_workout(&self, workout_id: u64) -> Result<Workout, String>;
    fn get_all_workouts(&self) -> Result<Vec<Workout>, String>;
    fn update_workout(&self, workout: &Workout) -> Result<(), String>;
    fn delete_workout(&self, workout_id: u64) -> Result<(), String>;
}
