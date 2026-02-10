use crate::domain::types::workout_template::{NewWorkoutTemplate, WorkoutTemplate};

#[derive(Debug)]
pub enum WorkoutTemplateModelError {
    NotFound,
    DatabaseError(String),
}

#[async_trait::async_trait]
pub trait WorkoutTemplateModel: Send + Sync {
    async fn create_workout_template(
        &mut self,
        template: NewWorkoutTemplate,
    ) -> Result<u64, WorkoutTemplateModelError>;
    async fn get_workout_template(
        &self,
        template_id: u64,
    ) -> Result<WorkoutTemplate, WorkoutTemplateModelError>;
    async fn get_all_workout_templates(
        &self,
    ) -> Result<Vec<WorkoutTemplate>, WorkoutTemplateModelError>;
    async fn update_workout_template(
        &mut self,
        template: WorkoutTemplate,
    ) -> Result<(), WorkoutTemplateModelError>;
    async fn delete_workout_template(
        &mut self,
        template_id: u64,
    ) -> Result<(), WorkoutTemplateModelError>;
}
