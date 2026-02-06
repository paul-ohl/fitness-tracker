use askama::Template;
use askama_web::WebTemplate;

use crate::domain::types::workout_plan::{
    ExercisePlannedDetails::*, WorkoutTemplate, WorkoutTemplateExercise,
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
        date: chrono::NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
        name: "Legs and Handstand".to_string(),
        exercises: vec![
            WorkoutTemplateExercise {
                name: "squat".to_string(),
                details: Weighted { value: 60.0 },
            },
            WorkoutTemplateExercise {
                name: "handstand pushup".to_string(),
                details: BodyweightReps { value: 2 },
            },
            WorkoutTemplateExercise {
                name: "handstand".to_string(),
                details: BodyweightTime { value: 12 },
            },
        ],
    }
}
