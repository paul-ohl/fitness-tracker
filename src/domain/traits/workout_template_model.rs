use crate::domain::types::workout_template::WorkoutTemplate;

pub trait WorkoutTemplateModel {
    fn create_workout_template(&self, template: &WorkoutTemplate) -> Result<(), String>;
    fn get_workout_template(&self, template_id: u64) -> Result<WorkoutTemplate, String>;
    fn get_all_workout_templates(&self) -> Result<Vec<WorkoutTemplate>, String>;
    fn update_workout_template(&self, template: &WorkoutTemplate) -> Result<(), String>;
    fn delete_workout_template(&self, template_id: u64) -> Result<(), String>;
}
