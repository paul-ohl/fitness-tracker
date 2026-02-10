use askama::Template;
use askama_web::WebTemplate;

use crate::domain::types::{
    exercise::{Exercise, ExerciseType},
    workout_template::WorkoutTemplate,
};

#[derive(Template, WebTemplate)]
#[template(path = "new_workout.html")]
pub struct NewWorkoutTemplate {
    workout_plan: WorkoutTemplate,
}

pub async fn new_workout_page() -> NewWorkoutTemplate {
    NewWorkoutTemplate {
        workout_plan: workout_template(),
    }
}

fn workout_template() -> WorkoutTemplate {
    WorkoutTemplate {
        name: "Legs and Handstand".to_string(),
        exercises: vec![
            Exercise {
                id: 1,
                name: "squat".to_string(),
                exercise_type: ExerciseType::Weighted { goal_weight: 100.0 },
            },
            Exercise {
                id: 2,
                name: "handstand pushup".to_string(),
                exercise_type: ExerciseType::BodyweightReps { goal_reps: 2 },
            },
            Exercise {
                id: 3,
                name: "handstand".to_string(),
                exercise_type: ExerciseType::BodyweightTime {
                    goal_duration_seconds: 12,
                },
            },
        ],
    }
}
