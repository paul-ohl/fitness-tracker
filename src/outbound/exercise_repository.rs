use sqlx::{QueryBuilder, SqlitePool, sqlite::Sqlite, sqlite::SqlitePoolOptions};

use crate::domain::{
    traits::exercise_model::{ExerciseModel, ExerciseModelError},
    types::exercise::{Exercise, ExerciseProgression, ExerciseType},
};

#[derive(Debug, Clone)]
pub struct ExerciseRepository {
    db_pool: SqlitePool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
enum SqliteExerciseType {
    Weighted,
    #[sqlx(rename = "bodyweight_reps")]
    BodyweightReps,
    #[sqlx(rename = "bodyweight_time")]
    BodyweightTime,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct SqliteExercise {
    pub id: u64,
    pub name: String,
    pub exercise_type: SqliteExerciseType,
    pub progression_name: Option<String>,
    pub progression_order: Option<u8>,
    pub goal_reps: Option<u16>,
    pub goal_weight: Option<f32>,
    pub goal_duration_seconds: Option<u16>,
}

struct SqliteExerciseInsert {
    pub name: String,
    pub exercise_type: SqliteExerciseType,
    pub progression_name: Option<String>,
    pub progression_order: Option<u8>,
    pub goal_reps: Option<u16>,
    pub goal_weight: Option<f32>,
    pub goal_duration_seconds: Option<u16>,
}

struct SqliteExerciseUpdate {
    pub id: u64,
    pub name: String,
    pub exercise_type: SqliteExerciseType,
    pub progression_name: Option<String>,
    pub progression_order: Option<u8>,
    pub goal_reps: Option<u16>,
    pub goal_weight: Option<f32>,
    pub goal_duration_seconds: Option<u16>,
}

fn exercise_goals_to_sqlite(
    exercise_type: &ExerciseType,
) -> (SqliteExerciseType, Option<u16>, Option<f32>, Option<u16>) {
    match exercise_type {
        ExerciseType::Weighted { goal_weight } => {
            (SqliteExerciseType::Weighted, None, Some(*goal_weight), None)
        }
        ExerciseType::BodyweightReps { goal_reps } => (
            SqliteExerciseType::BodyweightReps,
            Some(*goal_reps),
            None,
            None,
        ),
        ExerciseType::BodyweightTime {
            goal_duration_seconds,
        } => (
            SqliteExerciseType::BodyweightTime,
            None,
            None,
            Some(*goal_duration_seconds),
        ),
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

fn from_exercise_to_sqlite_insert(
    exercise: Exercise,
    progression_name: Option<String>,
    progression_order: Option<u8>,
) -> SqliteExerciseInsert {
    let (exercise_type, goal_reps, goal_weight, goal_duration_seconds) =
        exercise_goals_to_sqlite(&exercise.exercise_type);

    SqliteExerciseInsert {
        name: exercise.name,
        exercise_type,
        progression_name,
        progression_order,
        goal_reps,
        goal_weight,
        goal_duration_seconds,
    }
}

fn from_exercise_to_sqlite_update(
    exercise: Exercise,
    progression_name: Option<String>,
    progression_order: Option<u8>,
) -> SqliteExerciseUpdate {
    let (exercise_type, goal_reps, goal_weight, goal_duration_seconds) =
        exercise_goals_to_sqlite(&exercise.exercise_type);

    SqliteExerciseUpdate {
        id: exercise.id,
        name: exercise.name,
        exercise_type,
        progression_name,
        progression_order,
        goal_reps,
        goal_weight,
        goal_duration_seconds,
    }
}

impl ExerciseRepository {
    pub async fn new(db_url: String) -> std::result::Result<Self, sqlx::Error> {
        let db_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;
        Ok(Self { db_pool })
    }

    #[cfg(test)]
    pub fn from_pool(db_pool: SqlitePool) -> Self {
        Self { db_pool }
    }

    async fn get_repository_exercise_by_id(
        &self,
        id: u64,
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

    async fn add_exercise(
        &mut self,
        exercise: SqliteExerciseInsert,
    ) -> Result<(), ExerciseModelError> {
        sqlx::query(
            r#"
            INSERT INTO exercise (name, exercise_type, progression_name, progression_order, goal_reps, goal_weight, goal_duration_seconds)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(&exercise.name)
        .bind(&exercise.exercise_type)
        .bind(&exercise.progression_name)
        .bind(&exercise.progression_order)
        .bind(&exercise.goal_reps)
        .bind(&exercise.goal_weight)
        .bind(&exercise.goal_duration_seconds)
        .execute(&self.db_pool)
        .await
        .map_err(|e| ExerciseModelError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn add_multiple_exercises(
        &mut self,
        exercises: Vec<SqliteExerciseInsert>,
    ) -> Result<(), ExerciseModelError> {
        if exercises.is_empty() {
            return Ok(());
        }
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO exercise (name, exercise_type, progression_name, progression_order, goal_reps, goal_weight, goal_duration_seconds) ",
        );
        query_builder.push_values(exercises.iter(), |mut b, exercise| {
            b.push_bind(&exercise.name)
                .push_bind(&exercise.exercise_type)
                .push_bind(&exercise.progression_name)
                .push_bind(&exercise.progression_order)
                .push_bind(&exercise.goal_reps)
                .push_bind(&exercise.goal_weight)
                .push_bind(&exercise.goal_duration_seconds);
        });
        let query = query_builder.build();
        query
            .execute(&self.db_pool)
            .await
            .map_err(|e| ExerciseModelError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn update_multiple_exercises(
        &mut self,
        exercises: Vec<SqliteExerciseUpdate>,
    ) -> Result<(), ExerciseModelError> {
        for exercise in exercises {
            sqlx::query(
                r#"
                UPDATE exercise
                SET name = $1, exercise_type = $2, progression_name = $3, progression_order = $4, goal_reps = $5, goal_weight = $6, goal_duration_seconds = $7
                WHERE id = $8
                "#,
            )
            .bind(&exercise.name)
            .bind(&exercise.exercise_type)
            .bind(&exercise.progression_name)
            .bind(exercise.progression_order.map(|o| o as i64))
            .bind(&exercise.goal_reps)
            .bind(&exercise.goal_weight)
            .bind(&exercise.goal_duration_seconds)
            .bind(exercise.id as i64)
            .execute(&self.db_pool)
            .await
            .map_err(|e| ExerciseModelError::DatabaseError(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete_multiple_exercises_by_progression_name(
        &mut self,
        progression_name: &str,
    ) -> Result<(), ExerciseModelError> {
        let result = sqlx::query("DELETE FROM exercise WHERE progression_name = $1")
            .bind(progression_name)
            .execute(&self.db_pool)
            .await
            .map_err(|e| ExerciseModelError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ExerciseModelError::NotFound);
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl ExerciseModel for ExerciseRepository {
    async fn get_exercise_by_id(&self, id: u64) -> Result<Exercise, ExerciseModelError> {
        self.get_repository_exercise_by_id(id).await.map(Into::into)
    }

    async fn get_all_exercises(&self) -> Result<Vec<Exercise>, ExerciseModelError> {
        let rows: Vec<SqliteExercise> = sqlx::query_as(
            r#"
            SELECT id, name, exercise_type, progression_name, progression_order, goal_reps, goal_weight, goal_duration_seconds
            FROM exercise
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
        self.add_exercise(from_exercise_to_sqlite_insert(exercise, None, None))
            .await
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

    async fn delete_exercise(&mut self, id: u64) -> Result<(), ExerciseModelError> {
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
        exercise_id: u64,
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
        .map_err(|e| ExerciseModelError::DatabaseError(e.to_string()))?;

        if exercises.is_empty() {
            return Err(ExerciseModelError::NotFound);
        }

        Ok(ExerciseProgression {
            name: name.to_string(),
            progression: exercises.into_iter().map(Into::into).collect(),
        })
    }

    async fn add_exercise_progression(
        &mut self,
        progression: ExerciseProgression,
    ) -> Result<(), ExerciseModelError> {
        let exercises: Vec<SqliteExerciseInsert> = progression
            .progression
            .into_iter()
            .enumerate()
            .map(|(index, exercise)| {
                from_exercise_to_sqlite_insert(
                    exercise,
                    Some(progression.name.clone()),
                    Some((index + 1) as u8),
                )
            })
            .collect();
        self.add_multiple_exercises(exercises).await
    }

    async fn update_exercise_progression(
        &mut self,
        progression: ExerciseProgression,
    ) -> Result<(), ExerciseModelError> {
        let exercises: Vec<SqliteExerciseUpdate> = progression
            .progression
            .into_iter()
            .enumerate()
            .map(|(index, exercise)| {
                from_exercise_to_sqlite_update(
                    exercise,
                    Some(progression.name.clone()),
                    Some((index + 1) as u8),
                )
            })
            .collect();
        self.update_multiple_exercises(exercises).await
    }

    async fn delete_exercise_progression(
        &mut self,
        progression: ExerciseProgression,
    ) -> Result<(), ExerciseModelError> {
        self.delete_multiple_exercises_by_progression_name(&progression.name)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::traits::exercise_model::ExerciseModel;

    fn make_repo(pool: sqlx::SqlitePool) -> ExerciseRepository {
        ExerciseRepository { db_pool: pool }
    }

    #[sqlx::test]
    async fn get_exercise_by_id_returns_exercise_when_exists(pool: sqlx::SqlitePool) {
        sqlx::query("INSERT INTO exercise (name, exercise_type, goal_weight) VALUES ($1, $2, $3)")
            .bind("squat")
            .bind("weighted")
            .bind(60.0)
            .execute(&pool)
            .await
            .unwrap();

        let repo = make_repo(pool);
        let exercise = repo
            .get_exercise_by_id(1)
            .await
            .expect("fetch should succeed");

        assert_eq!(exercise.id, 1);
        assert_eq!(exercise.name, "squat");
        match &exercise.exercise_type {
            ExerciseType::Weighted { goal_weight } => assert_eq!(*goal_weight, 60.0),
            _ => panic!("expected Weighted exercise type"),
        }
    }

    #[sqlx::test]
    async fn get_exercise_by_id_returns_not_found_when_missing(pool: sqlx::SqlitePool) {
        let repo = make_repo(pool);
        let result = repo.get_exercise_by_id(999).await;
        assert!(matches!(result, Err(ExerciseModelError::NotFound)));
    }

    #[sqlx::test]
    async fn get_all_exercises_returns_empty_when_no_exercises(pool: sqlx::SqlitePool) {
        let repo = make_repo(pool);
        let exercises = repo.get_all_exercises().await.expect("should succeed");
        assert!(exercises.is_empty());
    }

    #[sqlx::test]
    async fn get_all_exercises_returns_all_exercises(pool: sqlx::SqlitePool) {
        sqlx::query(
            "INSERT INTO exercise (name, exercise_type, goal_reps) VALUES ('pushup', 'bodyweight_reps', 10)",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO exercise (name, exercise_type, goal_weight) VALUES ('squat', 'weighted', 60.0)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let repo = make_repo(pool);
        let exercises = repo.get_all_exercises().await.expect("should succeed");

        assert_eq!(exercises.len(), 2);
        let names: std::collections::HashSet<_> =
            exercises.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains("pushup"));
        assert!(names.contains("squat"));
    }

    #[sqlx::test]
    async fn add_exercise_inserts_weighted_exercise(pool: sqlx::SqlitePool) {
        let mut repo = make_repo(pool);
        let exercise = Exercise {
            id: 0,
            name: "bench press".to_string(),
            exercise_type: ExerciseType::Weighted { goal_weight: 80.0 },
        };

        ExerciseModel::add_exercise(&mut repo, exercise)
            .await
            .expect("add should succeed");

        let fetched = repo.get_exercise_by_id(1).await.expect("should exist");
        assert_eq!(fetched.name, "bench press");
        match &fetched.exercise_type {
            ExerciseType::Weighted { goal_weight } => assert_eq!(*goal_weight, 80.0),
            _ => panic!("expected Weighted"),
        }
    }

    #[sqlx::test]
    async fn add_exercise_inserts_bodyweight_reps_exercise(pool: sqlx::SqlitePool) {
        let mut repo = make_repo(pool);
        let exercise = Exercise {
            id: 0,
            name: "pullup".to_string(),
            exercise_type: ExerciseType::BodyweightReps { goal_reps: 8 },
        };

        ExerciseModel::add_exercise(&mut repo, exercise)
            .await
            .expect("add should succeed");

        let fetched = repo.get_exercise_by_id(1).await.expect("should exist");
        match &fetched.exercise_type {
            ExerciseType::BodyweightReps { goal_reps } => assert_eq!(*goal_reps, 8),
            _ => panic!("expected BodyweightReps"),
        }
    }

    #[sqlx::test]
    async fn add_exercise_inserts_bodyweight_time_exercise(pool: sqlx::SqlitePool) {
        let mut repo = make_repo(pool);
        let exercise = Exercise {
            id: 0,
            name: "plank".to_string(),
            exercise_type: ExerciseType::BodyweightTime {
                goal_duration_seconds: 60,
            },
        };

        ExerciseModel::add_exercise(&mut repo, exercise)
            .await
            .expect("add should succeed");

        let fetched = repo.get_exercise_by_id(1).await.expect("should exist");
        match &fetched.exercise_type {
            ExerciseType::BodyweightTime {
                goal_duration_seconds,
            } => {
                assert_eq!(*goal_duration_seconds, 60)
            }
            _ => panic!("expected BodyweightTime"),
        }
    }

    #[sqlx::test]
    async fn update_exercise_modifies_existing(pool: sqlx::SqlitePool) {
        sqlx::query(
            "INSERT INTO exercise (name, exercise_type, goal_weight) VALUES ('squat', 'weighted', 60.0)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let mut repo = make_repo(pool);
        let exercise = Exercise {
            id: 1,
            name: "front squat".to_string(),
            exercise_type: ExerciseType::Weighted { goal_weight: 70.0 },
        };

        repo.update_exercise(exercise)
            .await
            .expect("update should succeed");

        let fetched = repo.get_exercise_by_id(1).await.expect("should exist");
        assert_eq!(fetched.name, "front squat");
        match &fetched.exercise_type {
            ExerciseType::Weighted { goal_weight } => assert_eq!(*goal_weight, 70.0),
            _ => panic!("expected Weighted"),
        }
    }

    #[sqlx::test]
    async fn update_exercise_returns_not_found_when_missing(pool: sqlx::SqlitePool) {
        let mut repo = make_repo(pool);
        let exercise = Exercise {
            id: 999,
            name: "phantom".to_string(),
            exercise_type: ExerciseType::Weighted { goal_weight: 50.0 },
        };

        let result = repo.update_exercise(exercise).await;
        assert!(matches!(result, Err(ExerciseModelError::NotFound)));
    }

    #[sqlx::test]
    async fn delete_exercise_removes_existing(pool: sqlx::SqlitePool) {
        sqlx::query(
            "INSERT INTO exercise (name, exercise_type, goal_weight) VALUES ('squat', 'weighted', 60.0)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let mut repo = make_repo(pool);
        repo.delete_exercise(1)
            .await
            .expect("delete should succeed");

        let result = repo.get_exercise_by_id(1).await;
        assert!(matches!(result, Err(ExerciseModelError::NotFound)));
    }

    #[sqlx::test]
    async fn delete_exercise_returns_not_found_when_missing(pool: sqlx::SqlitePool) {
        let mut repo = make_repo(pool);
        let result = repo.delete_exercise(999).await;
        assert!(matches!(result, Err(ExerciseModelError::NotFound)));
    }

    #[sqlx::test]
    async fn get_all_exercise_progressions_returns_empty_when_none(pool: sqlx::SqlitePool) {
        sqlx::query(
            "INSERT INTO exercise (name, exercise_type, goal_reps) VALUES ('pushup', 'bodyweight_reps', 10)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let repo = make_repo(pool);
        let progressions = repo
            .get_all_exercise_progressions()
            .await
            .expect("should succeed");
        assert!(progressions.is_empty());
    }

    #[sqlx::test]
    async fn get_all_exercise_progressions_returns_progressions(pool: sqlx::SqlitePool) {
        sqlx::query(
            "INSERT INTO exercise (name, exercise_type, progression_name, progression_order, goal_reps) VALUES
             ('pushup incline', 'bodyweight_reps', 'pushup', 1, 10),
             ('pushup knees', 'bodyweight_reps', 'pushup', 2, 15),
             ('pushup', 'bodyweight_reps', 'pushup', 3, 20)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let repo = make_repo(pool);
        let progressions = repo
            .get_all_exercise_progressions()
            .await
            .expect("should succeed");

        assert_eq!(progressions.len(), 1);
        assert_eq!(progressions[0].name, "pushup");
        assert_eq!(progressions[0].progression.len(), 3);
        assert_eq!(progressions[0].progression[0].name, "pushup incline");
        assert_eq!(progressions[0].progression[1].name, "pushup knees");
        assert_eq!(progressions[0].progression[2].name, "pushup");
    }

    #[sqlx::test]
    async fn get_exercise_progression_returns_progression_when_exists(pool: sqlx::SqlitePool) {
        sqlx::query(
            "INSERT INTO exercise (id, name, exercise_type, progression_name, progression_order, goal_reps) VALUES
             (1, 'pushup incline', 'bodyweight_reps', 'pushup', 1, 10),
             (2, 'pushup', 'bodyweight_reps', 'pushup', 2, 20)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let repo = make_repo(pool);
        let progression = repo
            .get_exercise_progression(1)
            .await
            .expect("should succeed");

        assert_eq!(progression.name, "pushup");
        assert_eq!(progression.progression.len(), 2);
    }

    #[sqlx::test]
    async fn get_exercise_progression_returns_not_found_when_no_progression(
        pool: sqlx::SqlitePool,
    ) {
        sqlx::query(
            "INSERT INTO exercise (name, exercise_type, goal_weight) VALUES ('squat', 'weighted', 60.0)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let repo = make_repo(pool);
        let result = repo.get_exercise_progression(1).await;
        assert!(matches!(result, Err(ExerciseModelError::NotFound)));
    }

    #[sqlx::test]
    async fn get_exercise_progression_returns_not_found_when_exercise_missing(
        pool: sqlx::SqlitePool,
    ) {
        let repo = make_repo(pool);
        let result = repo.get_exercise_progression(999).await;
        assert!(matches!(result, Err(ExerciseModelError::NotFound)));
    }

    #[sqlx::test]
    async fn get_exercise_progression_from_name_returns_progression(pool: sqlx::SqlitePool) {
        sqlx::query(
            "INSERT INTO exercise (name, exercise_type, progression_name, progression_order, goal_reps) VALUES
             ('pushup incline', 'bodyweight_reps', 'pushup', 1, 10),
             ('pushup', 'bodyweight_reps', 'pushup', 2, 20)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let repo = make_repo(pool);
        let progression = repo
            .get_exercise_progression_from_name("pushup")
            .await
            .expect("should succeed");

        assert_eq!(progression.name, "pushup");
        assert_eq!(progression.progression.len(), 2);
    }

    #[sqlx::test]
    async fn get_exercise_progression_from_name_returns_not_found_when_missing(
        pool: sqlx::SqlitePool,
    ) {
        let repo = make_repo(pool);
        let result = repo.get_exercise_progression_from_name("nonexistent").await;
        assert!(matches!(result, Err(ExerciseModelError::NotFound)));
    }
}
