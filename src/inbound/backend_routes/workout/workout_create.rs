#![allow(unused)]

use std::sync::Arc;

use axum::{
    Json,
    body::{self, Body},
    extract::State,
    response::IntoResponse,
};
use chrono::NaiveDate;
use serde::Deserialize;

use crate::{
    domain::types::workout::{
        NewBodyweightRepSet, NewBodyweightTimeSet, NewWeightedSet, NewWorkout, NewWorkoutExercise,
        NewWorkoutSet,
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

impl TryFrom<WorkoutCreateRequest> for NewWorkout {
    type Error = String;

    fn try_from(value: WorkoutCreateRequest) -> Result<Self, Self::Error> {
        let exercises = value
            .exercises
            .into_iter()
            .map(NewWorkoutExercise::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(NewWorkout {
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
    pub sets: Vec<ExerciseSetCreateRequest>,
}

impl TryFrom<ExerciseDoneCreateRequest> for NewWorkoutExercise {
    type Error = String;

    fn try_from(value: ExerciseDoneCreateRequest) -> Result<Self, Self::Error> {
        let sets = value
            .sets
            .into_iter()
            .map(NewWorkoutSet::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(NewWorkoutExercise {
            exercise_id: todo!(),
            sets,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExerciseSetCreateRequest {
    Weighted(WeightedSetCreateRequest),
    BodyweightReps(BodyweightRepSetCreateRequest),
    BodyweightTime(BodyweightTimeSetCreateRequest),
}

impl TryFrom<ExerciseSetCreateRequest> for NewWorkoutSet {
    type Error = String;

    fn try_from(value: ExerciseSetCreateRequest) -> Result<Self, Self::Error> {
        match value {
            ExerciseSetCreateRequest::Weighted(weighted_set) => {
                Ok(NewWorkoutSet::Weighted(weighted_set.try_into()?))
            }

            ExerciseSetCreateRequest::BodyweightReps(bodyweight_reps_sets) => Ok(
                NewWorkoutSet::BodyweightReps(bodyweight_reps_sets.try_into()?),
            ),
            ExerciseSetCreateRequest::BodyweightTime(bodyweight_time_sets) => Ok(
                NewWorkoutSet::BodyweightTime(bodyweight_time_sets.try_into()?),
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightedSetCreateRequest {
    pub reps: Option<u16>,
    pub weight: f32,
    pub failure: Option<bool>,
}

impl TryFrom<WeightedSetCreateRequest> for NewWeightedSet {
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
        Ok(NewWeightedSet {
            reps: value.reps.unwrap_or(1),
            weight: value.weight,
            failure: value.failure.unwrap_or(false),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyweightRepSetCreateRequest {
    pub reps: u16,
    pub failure: Option<bool>,
}

impl TryFrom<BodyweightRepSetCreateRequest> for NewBodyweightRepSet {
    type Error = String;

    fn try_from(value: BodyweightRepSetCreateRequest) -> Result<Self, Self::Error> {
        if value.reps == 0 {
            return Err("Reps must be greater than 0".to_string());
        }
        Ok(NewBodyweightRepSet {
            reps: value.reps,
            failure: value.failure.unwrap_or(false),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyweightTimeSetCreateRequest {
    pub duration_seconds: u16,
    pub failure: Option<bool>,
}

impl TryFrom<BodyweightTimeSetCreateRequest> for NewBodyweightTimeSet {
    type Error = String;

    fn try_from(value: BodyweightTimeSetCreateRequest) -> Result<Self, Self::Error> {
        if value.duration_seconds == 0 {
            return Err("Duration must be greater than 0".to_string());
        }
        Ok(NewBodyweightTimeSet {
            duration_seconds: value.duration_seconds,
            failure: value.failure.unwrap_or(false),
        })
    }
}

pub async fn create_workout(
    State(core_logic): State<Arc<AppState>>,
    Json(workout_create_req): Json<WorkoutCreateRequest>,
) -> Result<impl IntoResponse, String> {
    let workout: NewWorkout = workout_create_req.try_into()?;
    Ok("Pouet".into_response())
}
