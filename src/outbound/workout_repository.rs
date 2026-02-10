use std::sync::Arc;

use sqlx::SqlitePool;

use crate::domain::{
    traits::{
        exercise_model::ExerciseModel,
        workout_model::{WorkoutModel, WorkoutModelError},
    },
    types::workout::{
        BodyweightRepSet, BodyweightTimeSet, NewWorkout, NewWorkoutExercise, NewWorkoutSet,
        NewBodyweightRepSet, NewBodyweightTimeSet, NewWeightedSet, WeightedSet, Workout,
        WorkoutExercise, WorkoutSet,
    },
};

#[derive(Clone)]
pub struct WorkoutRepository {
    db_pool: SqlitePool,
    exercise_model: Arc<dyn ExerciseModel>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct SqliteWorkout {
    pub id: u64,
    pub date: chrono::NaiveDate,
    pub mood: Option<i64>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct SqliteWorkoutSet {
    pub id: u64,
    pub workout_id: u64,
    pub exercise_id: u64,
    pub set_order: u32,
    pub reps: Option<i64>,
    pub weight: Option<f64>,
    pub duration_seconds: Option<i64>,
    pub failure: i64,
}

impl WorkoutRepository {
    pub fn new(db_pool: SqlitePool, exercise_model: Arc<dyn ExerciseModel>) -> Self {
        Self {
            db_pool,
            exercise_model,
        }
    }

    async fn insert_workout(&self, date: chrono::NaiveDate, mood: Option<u8>) -> Result<u64, WorkoutModelError> {
        let result = sqlx::query(
            "INSERT INTO workout (date, mood) VALUES ($1, $2)",
        )
        .bind(date)
        .bind(mood.map(|m| m as i64))
        .execute(&self.db_pool)
        .await
        .map_err(|e| WorkoutModelError::DatabaseError(e.to_string()))?;
        Ok(result.last_insert_rowid() as u64)
    }

    async fn insert_workout_sets(
        &self,
        workout_id: u64,
        exercises: &[NewWorkoutExercise],
    ) -> Result<(), WorkoutModelError> {
        for (exercise_idx, exercise) in exercises.iter().enumerate() {
            for (set_idx, set) in exercise.sets.iter().enumerate() {
                let set_order = (exercise_idx * 1000 + set_idx + 1) as i64;
                let (reps, weight, duration_seconds, failure) = match set {
                    NewWorkoutSet::Weighted(s) => (
                        Some(s.reps as i64),
                        Some(s.weight as f64),
                        None,
                        s.failure as i64,
                    ),
                    NewWorkoutSet::BodyweightReps(s) => (
                        Some(s.reps as i64),
                        None,
                        None,
                        s.failure as i64,
                    ),
                    NewWorkoutSet::BodyweightTime(s) => (
                        None,
                        None,
                        Some(s.duration_seconds as i64),
                        s.failure as i64,
                    ),
                };
                sqlx::query(
                    r#"
                    INSERT INTO workout_set (workout_id, exercise_id, set_order, reps, weight, duration_seconds, failure)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#,
                )
                .bind(workout_id as i64)
                .bind(exercise.exercise_id as i64)
                .bind(set_order)
                .bind(reps)
                .bind(weight)
                .bind(duration_seconds)
                .bind(failure)
                .execute(&self.db_pool)
                .await
                .map_err(|e| WorkoutModelError::DatabaseError(e.to_string()))?;
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl WorkoutModel for WorkoutRepository {
    async fn create_workout(&mut self, workout: NewWorkout) -> Result<u64, WorkoutModelError> {
        let workout_id = self.insert_workout(workout.date, workout.mood).await?;
        self.insert_workout_sets(workout_id, &workout.exercises)
            .await?;
        Ok(workout_id)
    }

    async fn get_workout(&self, workout_id: u64) -> Result<Workout, WorkoutModelError> {
        let workout_row: SqliteWorkout = sqlx::query_as(
            "SELECT id, date, mood FROM workout WHERE id = $1",
        )
        .bind(workout_id as i64)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => WorkoutModelError::NotFound,
            other => WorkoutModelError::DatabaseError(other.to_string()),
        })?;

        let sets: Vec<SqliteWorkoutSet> = sqlx::query_as(
            r#"
            SELECT id, workout_id, exercise_id, set_order, reps, weight, duration_seconds, failure
            FROM workout_set
            WHERE workout_id = $1
            ORDER BY set_order
            "#,
        )
        .bind(workout_id as i64)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| WorkoutModelError::DatabaseError(e.to_string()))?;

        let mut exercises_by_id: std::collections::HashMap<u64, Vec<SqliteWorkoutSet>> =
            std::collections::HashMap::new();
        for set in sets {
            exercises_by_id
                .entry(set.exercise_id)
                .or_default()
                .push(set);
        }

        let mut workout_exercises = Vec::new();
        for (exercise_id, sets) in exercises_by_id {
            let exercise = self
                .exercise_model
                .get_exercise_by_id(exercise_id)
                .await
                .map_err(|e| WorkoutModelError::DatabaseError(format!("{:?}", e)))?;
            let workout_sets: Vec<WorkoutSet> = sets
                .into_iter()
                .map(|s| {
                    let failure = s.failure != 0;
                    if s.weight.is_some() {
                        WorkoutSet::Weighted(WeightedSet {
                            id: s.id,
                            reps: s.reps.unwrap_or(0) as u16,
                            weight: s.weight.unwrap_or(0.0) as f32,
                            failure,
                        })
                    } else if s.duration_seconds.is_some() {
                        WorkoutSet::BodyweightTime(BodyweightTimeSet {
                            id: s.id,
                            duration_seconds: s.duration_seconds.unwrap_or(0) as u16,
                            failure,
                        })
                    } else {
                        WorkoutSet::BodyweightReps(BodyweightRepSet {
                            id: s.id,
                            reps: s.reps.unwrap_or(0) as u16,
                            failure,
                        })
                    }
                })
                .collect();
            workout_exercises.push(WorkoutExercise {
                exercise,
                sets: workout_sets,
            });
        }

        Ok(Workout {
            id: workout_row.id,
            date: workout_row.date,
            mood: workout_row.mood.map(|m| m as u8),
            exercises: workout_exercises,
        })
    }

    async fn get_all_workouts(&self) -> Result<Vec<Workout>, WorkoutModelError> {
        let rows: Vec<SqliteWorkout> = sqlx::query_as("SELECT id, date, mood FROM workout")
            .fetch_all(&self.db_pool)
            .await
            .map_err(|e| WorkoutModelError::DatabaseError(e.to_string()))?;

        let mut workouts = Vec::new();
        for row in rows {
            workouts.push(self.get_workout(row.id).await?);
        }
        Ok(workouts)
    }

    async fn update_workout(&mut self, workout: Workout) -> Result<(), WorkoutModelError> {
        let result = sqlx::query("UPDATE workout SET date = $1, mood = $2 WHERE id = $3")
            .bind(workout.date)
            .bind(workout.mood.map(|m| m as i64))
            .bind(workout.id as i64)
            .execute(&self.db_pool)
            .await
            .map_err(|e| WorkoutModelError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(WorkoutModelError::NotFound);
        }

        sqlx::query("DELETE FROM workout_set WHERE workout_id = $1")
            .bind(workout.id as i64)
            .execute(&self.db_pool)
            .await
            .map_err(|e| WorkoutModelError::DatabaseError(e.to_string()))?;

        let new_exercises: Vec<NewWorkoutExercise> = workout
            .exercises
            .into_iter()
            .map(|we| {
                let sets: Vec<NewWorkoutSet> = we
                    .sets
                    .into_iter()
                    .map(|s| match s {
                        WorkoutSet::Weighted(ws) => {
                            NewWorkoutSet::Weighted(NewWeightedSet {
                                reps: ws.reps,
                                weight: ws.weight,
                                failure: ws.failure,
                            })
                        }
                        WorkoutSet::BodyweightReps(ws) => {
                            NewWorkoutSet::BodyweightReps(NewBodyweightRepSet {
                                reps: ws.reps,
                                failure: ws.failure,
                            })
                        }
                        WorkoutSet::BodyweightTime(ws) => {
                            NewWorkoutSet::BodyweightTime(NewBodyweightTimeSet {
                                duration_seconds: ws.duration_seconds,
                                failure: ws.failure,
                            })
                        }
                    })
                    .collect();
                NewWorkoutExercise {
                    exercise_id: we.exercise.id,
                    sets,
                }
            })
            .collect();
        self.insert_workout_sets(workout.id, &new_exercises)
            .await?;
        Ok(())
    }

    async fn delete_workout(&mut self, workout_id: u64) -> Result<(), WorkoutModelError> {
        let result = sqlx::query("DELETE FROM workout WHERE id = $1")
            .bind(workout_id as i64)
            .execute(&self.db_pool)
            .await
            .map_err(|e| WorkoutModelError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(WorkoutModelError::NotFound);
        }
        Ok(())
    }
}
