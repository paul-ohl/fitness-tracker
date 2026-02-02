use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, response::Redirect, routing::get};

use crate::types::workout_plan::{ExercisePlanned, ExercisePlannedDetails::*, WorkoutPlan};

#[derive(Template, WebTemplate)]
#[template(path = "new_workout.html")]
struct NewWorkoutTemplate {
    workout_plan: WorkoutPlan,
}

async fn new_workout_page() -> NewWorkoutTemplate {
    NewWorkoutTemplate {
        workout_plan: workout_template(),
    }
}

pub fn new_workout_router() -> Router {
    Router::new().route("/new", get(new_workout_page).post(Redirect::to("/new")))
}

fn workout_template() -> WorkoutPlan {
    WorkoutPlan {
        date: chrono::NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
        exercises: vec![
            ExercisePlanned {
                name: "squat".to_string(),
                details: Weighted { value: 60.0 },
            },
            ExercisePlanned {
                name: "handstand pushup".to_string(),
                details: BodyweightReps { value: 2 },
            },
            ExercisePlanned {
                name: "handstand".to_string(),
                details: BodyweightTime { value: 12 },
            },
        ],
    }
}
