#[derive(Debug, Clone)]
pub struct Goal {
    pub name: String,
    pub target: ExerciseTarget,
}

#[derive(Debug, Clone)]
pub enum ExerciseTarget {
    Weighted {
        target_weight: f32,
        target_reps: Option<u32>,
        target_sets: Option<u32>,
    },
    BodyweightReps {
        progression: Vec<BodyweightRepsProgression>,
    },
    BodyweightTime {
        progression: Vec<BodyweightTimeProgression>,
    },
}

#[derive(Debug, Clone)]
pub struct BodyweightRepsProgression {
    pub name: String,
    pub target: BodyweightRepsTarget,
}

#[derive(Debug, Clone)]
pub struct BodyweightRepsTarget {
    pub target_reps: u32,
    pub target_sets: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct BodyweightTimeProgression {
    pub name: String,
    pub target: BodyweightTimeTarget,
}

#[derive(Debug, Clone)]
pub struct BodyweightTimeTarget {
    pub target_duration_seconds: u32,
    pub target_sets: Option<u32>,
}
