use crate::domain::traits::{
    exercise_model::ExerciseModel, workout_model::WorkoutModel,
    workout_template_model::WorkoutTemplateModel,
};

pub struct AppState {
    pub exercise_model: Box<dyn ExerciseModel>,
    pub workout_model: Box<dyn WorkoutModel>,
    pub workout_plan_model: Box<dyn WorkoutTemplateModel>,
}
