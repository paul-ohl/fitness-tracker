use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, response::Redirect, routing::get};

use crate::types::workout_plan::{
    ExercisesPlanned::{BodyweightReps, BodyweightTime, Weighted},
    WorkoutPlan,
};

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
        exercises: vec![
            Weighted {
                name: "squat".to_string(),
                value: 60.0,
            },
            BodyweightReps {
                name: "handstand pushup".to_string(),
                value: 2,
            },
            BodyweightTime {
                name: "handstand".to_string(),
                value: 12,
            },
        ],
    }
}
