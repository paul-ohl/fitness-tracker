#![allow(unused)]

use std::sync::Arc;

use axum::{Json, body::Body, extract::State, response::IntoResponse};
use chrono::NaiveDate;
use serde::Deserialize;

use crate::{
    domain::types::workout::{
        BodyweightRepSet, BodyweightTimeSet, WeightedSet, Workout, WorkoutExercise, WorkoutSet,
    },
    state::AppState,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkoutCreateRequest {
    pub date: NaiveDate,
    pub mood: Option<u8>,
    pub exercises: Vec<ExerciseDoneCreateRequest>,
}

impl TryFrom<WorkoutCreateRequest> for Workout {
    type Error = String;

    fn try_from(value: WorkoutCreateRequest) -> Result<Self, Self::Error> {
        let exercises = value
            .exercises
            .into_iter()
            .map(WorkoutExercise::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Workout {
            date: value.date,
            mood: value.mood,
            exercises,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseDoneCreateRequest {
    pub name: String,
    pub sets: ExerciseSetCreateRequest,
}

impl TryFrom<ExerciseDoneCreateRequest> for WorkoutExercise {
    type Error = String;

    fn try_from(value: ExerciseDoneCreateRequest) -> Result<Self, Self::Error> {
        let sets = value.sets.try_into()?;
        Ok(WorkoutExercise {
            name: value.name,
            sets,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExerciseSetCreateRequest {
    Weighted(Vec<WeightedSetCreateRequest>),
    BodyweightReps(Vec<BodyweightRepSetCreateRequest>),
    BodyweightTime(Vec<BodyweightTimeSetCreateRequest>),
}

impl TryFrom<ExerciseSetCreateRequest> for WorkoutSet {
    type Error = String;

    fn try_from(value: ExerciseSetCreateRequest) -> Result<Self, Self::Error> {
        match value {
            ExerciseSetCreateRequest::Weighted(weighted_sets) => {
                let sets = weighted_sets
                    .into_iter()
                    .map(WeightedSet::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(WorkoutSet::Weighted(sets))
            }
            ExerciseSetCreateRequest::BodyweightReps(bodyweight_reps_sets) => {
                let sets = bodyweight_reps_sets
                    .into_iter()
                    .map(BodyweightRepSet::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(WorkoutSet::BodyweightReps(sets))
            }
            ExerciseSetCreateRequest::BodyweightTime(bodyweight_time_sets) => {
                let sets = bodyweight_time_sets
                    .into_iter()
                    .map(BodyweightTimeSet::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(WorkoutSet::BodyweightTime(sets))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightedSetCreateRequest {
    pub reps: Option<u32>,
    pub weight: f32,
    pub failure: Option<bool>,
}

impl TryFrom<WeightedSetCreateRequest> for WeightedSet {
    type Error = String;

    fn try_from(value: WeightedSetCreateRequest) -> Result<Self, Self::Error> {
        if let Some(reps) = value.reps
            && reps == 0
        {
            return Err("Reps must be greater than 0".to_string());
        }
        if value.weight <= 0.0 {
            return Err("Weight must be greater than 0".to_string());
        }
        Ok(WeightedSet {
            reps: value.reps.unwrap_or(1),
            weight: value.weight,
            failure: value.failure.unwrap_or(false),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyweightRepSetCreateRequest {
    pub reps: u32,
    pub failure: Option<bool>,
}

impl TryFrom<BodyweightRepSetCreateRequest> for BodyweightRepSet {
    type Error = String;

    fn try_from(value: BodyweightRepSetCreateRequest) -> Result<Self, Self::Error> {
        if value.reps == 0 {
            return Err("Reps must be greater than 0".to_string());
        }
        Ok(BodyweightRepSet {
            reps: value.reps,
            failure: value.failure.unwrap_or(false),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyweightTimeSetCreateRequest {
    pub duration_seconds: u32,
}

impl TryFrom<BodyweightTimeSetCreateRequest> for BodyweightTimeSet {
    type Error = String;

    fn try_from(value: BodyweightTimeSetCreateRequest) -> Result<Self, Self::Error> {
        if value.duration_seconds == 0 {
            return Err("Duration must be greater than 0".to_string());
        }
        Ok(BodyweightTimeSet {
            duration_seconds: value.duration_seconds,
        })
    }
}

pub async fn create_workout(
    State(core_logic): State<Arc<AppState>>,
    Json(workout_create_req): Json<WorkoutCreateRequest>,
) -> Result<impl IntoResponse, String> {
    let workout: Workout = workout_create_req.try_into()?;
    Ok("Pouet".into_response())
}
