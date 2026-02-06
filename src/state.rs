use crate::outbound::{WorkoutPlanRepository, WorkoutRepository};

pub struct AppState {
    pub workout_repository: WorkoutRepository,
    pub workout_plan_repository: WorkoutPlanRepository,
}
