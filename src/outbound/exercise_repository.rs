#![allow(unused)]

use chrono::NaiveDate;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

use crate::domain::{
    traits::exercise_model::{ExerciseModel, ExerciseModelError},
    types::exercise::{Exercise, ExerciseProgression, ExerciseType},
};

#[derive(Debug, Clone)]
pub struct ExerciseRepository {
    db_pool: SqlitePool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
enum SqliteExerciseType {
    Weighted,
    BodyweightReps,
    BodyweightTime,
}

impl SqliteExerciseType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "weighted" => Some(SqliteExerciseType::Weighted),
            "bodyweight_reps" => Some(SqliteExerciseType::BodyweightReps),
            "bodyweight_time" => Some(SqliteExerciseType::BodyweightTime),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            SqliteExerciseType::Weighted => "weighted",
            SqliteExerciseType::BodyweightReps => "bodyweight_reps",
            SqliteExerciseType::BodyweightTime => "bodyweight_time",
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct SqliteExercise {
    pub id: u64,
    pub name: String,
    pub exercise_type: SqliteExerciseType,
    pub progression_name: Option<String>,
    pub progression_order: Option<u32>,
    pub goal_reps: Option<u32>,
    pub goal_weight: Option<f32>,
    pub goal_duration_seconds: Option<u32>,
}

fn exercise_goals_to_sqlite(
    exercise_type: &ExerciseType,
) -> (&'static str, Option<u32>, Option<f32>, Option<u32>) {
    match exercise_type {
        ExerciseType::Weighted { goal_weight } => ("weighted", None, Some(*goal_weight), None),
        ExerciseType::BodyweightReps { goal_reps } => {
            ("bodyweight_reps", Some(*goal_reps), None, None)
        }
        ExerciseType::BodyweightTime {
            goal_duration_seconds,
        } => ("bodyweight_time", None, None, Some(*goal_duration_seconds)),
    }
}

impl From<SqliteExercise> for Exercise {
    fn from(row: SqliteExercise) -> Self {
        let exercise_type = match row.exercise_type {
            SqliteExerciseType::Weighted => ExerciseType::Weighted {
                goal_weight: row.goal_weight.unwrap_or(0.0),
            },
            SqliteExerciseType::BodyweightReps => ExerciseType::BodyweightReps {
                goal_reps: row.goal_reps.unwrap_or(0),
            },
            SqliteExerciseType::BodyweightTime => ExerciseType::BodyweightTime {
                goal_duration_seconds: row.goal_duration_seconds.unwrap_or(0),
            },
        };

        Exercise {
            id: row.id,
            name: row.name,
            exercise_type,
        }
    }
}

pub enum RepositoryError {
    NotFound,
    DatabaseError(sqlx::Error),
}

impl ExerciseRepository {
    pub async fn new(db_url: String) -> std::result::Result<Self, sqlx::Error> {
        let db_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;
        Ok(Self { db_pool })
    }

    async fn get_repository_exercise_by_id(
        &self,
        id: u32,
    ) -> Result<SqliteExercise, ExerciseModelError> {
        sqlx::query_as(
            r#"
            SELECT id, name, exercise_type, progression_name, progression_order, goal_reps, goal_weight, goal_duration_seconds
            FROM exercise
            WHERE id = $1
            "#,
        )
        .bind(id as i64)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ExerciseModelError::NotFound,
            other => ExerciseModelError::DatabaseError(other.to_string()),
        })
    }
}

#[async_trait::async_trait]
impl ExerciseModel for ExerciseRepository {
    async fn get_exercise_by_id(&self, id: u32) -> Result<Exercise, ExerciseModelError> {
        self.get_repository_exercise_by_id(id).await.map(Into::into)
    }

    async fn get_all_exercises(&self) -> Result<Vec<Exercise>, ExerciseModelError> {
        let rows: Vec<SqliteExercise> = sqlx::query_as(
            r#"
            SELECT id, name, exercise_type, progression_name, progression_order, goal_reps, goal_weight, goal_duration_seconds
            FROM exercise
            ORDER BY name
            "#,
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| {
            ExerciseModelError::DatabaseError(e.to_string())
        })?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn add_exercise(&mut self, exercise: Exercise) -> Result<(), ExerciseModelError> {
        let (exercise_type, goal_reps, goal_weight, goal_duration_seconds) =
            exercise_goals_to_sqlite(&exercise.exercise_type);

        sqlx::query(
            r#"
            INSERT INTO exercise (name, exercise_type, goal_reps, goal_weight, goal_duration_seconds)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(&exercise.name)
        .bind(exercise_type)
        .bind(goal_reps)
        .bind(goal_weight)
        .bind(goal_duration_seconds)
        .execute(&self.db_pool)
        .await
        .map_err(|e| ExerciseModelError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn update_exercise(&mut self, exercise: Exercise) -> Result<(), ExerciseModelError> {
        let (exercise_type, goal_reps, goal_weight, goal_duration_seconds) =
            exercise_goals_to_sqlite(&exercise.exercise_type);

        let result = sqlx::query(
            r#"
            UPDATE exercise
            SET name = $1, exercise_type = $2, goal_reps = $3, goal_weight = $4, goal_duration_seconds = $5
            WHERE id = $6
            "#,
        )
        .bind(&exercise.name)
        .bind(exercise_type)
        .bind(goal_reps)
        .bind(goal_weight)
        .bind(goal_duration_seconds)
        .bind(exercise.id as i64)
        .execute(&self.db_pool)
        .await
        .map_err(|e| ExerciseModelError::DatabaseError( e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ExerciseModelError::NotFound);
        }
        Ok(())
    }

    async fn delete_exercise(&mut self, id: u32) -> Result<(), ExerciseModelError> {
        let result = sqlx::query("DELETE FROM exercise WHERE id = $1")
            .bind(id as i64)
            .execute(&self.db_pool)
            .await
            .map_err(|e| ExerciseModelError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ExerciseModelError::NotFound);
        }
        Ok(())
    }

    async fn get_all_exercise_progressions(
        &self,
    ) -> Result<Vec<ExerciseProgression>, ExerciseModelError> {
        let exercises: Vec<SqliteExercise> = sqlx::query_as(
            r#"
            SELECT id, name, exercise_type, progression_name, progression_order, goal_reps, goal_weight, goal_duration_seconds
            FROM exercise
            WHERE progression_name IS NOT NULL
            ORDER BY progression_name, progression_order
            "#,
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| {
            ExerciseModelError::DatabaseError(e.to_string())
        })?;

        let result = exercises
            .into_iter()
            .fold(std::collections::HashMap::new(), |mut acc, exercise| {
                acc.entry(exercise.progression_name.clone().unwrap())
                    .or_insert_with(Vec::new)
                    .push(exercise.into());
                acc
            })
            .into_iter()
            .map(|(name, exercises)| ExerciseProgression {
                name,
                progression: exercises,
            })
            .collect::<Vec<_>>();

        Ok(result)
    }

    async fn get_exercise_progression(
        &self,
        exercise_id: u32,
    ) -> Result<ExerciseProgression, ExerciseModelError> {
        match self
            .get_repository_exercise_by_id(exercise_id)
            .await?
            .progression_name
        {
            Some(name) => self.get_exercise_progression_from_name(&name).await,
            None => Err(ExerciseModelError::NotFound),
        }
    }

    async fn get_exercise_progression_from_name(
        &self,
        name: &str,
    ) -> Result<ExerciseProgression, ExerciseModelError> {
        let exercises: Vec<SqliteExercise> = sqlx::query_as(
            r#"
            SELECT id, name, exercise_type, progression_name, progression_order, goal_reps, goal_weight, goal_duration_seconds
            FROM exercise
            WHERE progression_name = $1
            ORDER BY progression_order
            "#,
        )
        .bind(name)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| {
            ExerciseModelError::DatabaseError(e.to_string())
        })?;

        if exercises.is_empty() {
            return Err(ExerciseModelError::NotFound);
        }

        Ok(ExerciseProgression {
            name: name.to_string(),
            progression: exercises.into_iter().map(Into::into).collect(),
        })
    }
}
