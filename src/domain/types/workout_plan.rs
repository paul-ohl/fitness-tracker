use chrono::NaiveDate;

pub struct WorkoutPlan {
    pub date: NaiveDate,
    pub name: String,
    pub exercises: Vec<ExercisePlanned>,
}

pub struct ExercisePlanned {
    pub name: String,
    pub details: ExercisePlannedDetails,
}

pub enum ExercisePlannedDetails {
    Weighted { value: f32 },
    BodyweightReps { value: i32 },
    BodyweightTime { value: i32 },
}
