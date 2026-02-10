use std::sync::Arc;

use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::domain::{
    traits::{
        exercise_model::ExerciseModel,
        workout_template_model::{WorkoutTemplateModel, WorkoutTemplateModelError},
    },
    types::workout_template::{NewWorkoutTemplate, WorkoutTemplate},
};

#[derive(Clone)]
pub struct WorkoutTemplateRepository {
    db_pool: SqlitePool,
    exercise_model: Arc<dyn ExerciseModel>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct SqliteWorkoutTemplate {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct SqliteWorkoutTemplateExercise {
    pub workout_template_id: u64,
    pub exercise_id: u64,
}

impl WorkoutTemplateRepository {
    pub fn new(
        db_pool: SqlitePool,
        exercise_model: Arc<dyn ExerciseModel>,
    ) -> Self {
        Self {
            db_pool,
            exercise_model,
        }
    }

    async fn insert_template_exercises(
        &self,
        template_id: u64,
        exercise_ids: &[u64],
    ) -> Result<(), WorkoutTemplateModelError> {
        if exercise_ids.is_empty() {
            return Ok(());
        }
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO workout_template_exercise (workout_template_id, exercise_id) ",
        );
        query_builder.push_values(exercise_ids.iter(), |mut b, &exercise_id| {
            b.push_bind(template_id as i64).push_bind(exercise_id as i64);
        });
        let query = query_builder.build();
        query
            .execute(&self.db_pool)
            .await
            .map_err(|e| WorkoutTemplateModelError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl WorkoutTemplateModel for WorkoutTemplateRepository {
    async fn create_workout_template(
        &mut self,
        template: NewWorkoutTemplate,
    ) -> Result<u64, WorkoutTemplateModelError> {
        let result = sqlx::query("INSERT INTO workout_template (name) VALUES ($1)")
            .bind(&template.name)
            .execute(&self.db_pool)
            .await
            .map_err(|e| WorkoutTemplateModelError::DatabaseError(e.to_string()))?;
        let template_id = result.last_insert_rowid() as u64;
        self.insert_template_exercises(template_id, &template.exercise_ids)
            .await?;
        Ok(template_id)
    }

    async fn get_workout_template(
        &self,
        template_id: u64,
    ) -> Result<WorkoutTemplate, WorkoutTemplateModelError> {
        let row: SqliteWorkoutTemplate = sqlx::query_as(
            "SELECT id, name FROM workout_template WHERE id = $1",
        )
        .bind(template_id as i64)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => WorkoutTemplateModelError::NotFound,
            other => WorkoutTemplateModelError::DatabaseError(other.to_string()),
        })?;

        let links: Vec<SqliteWorkoutTemplateExercise> = sqlx::query_as(
            r#"
            SELECT workout_template_id, exercise_id
            FROM workout_template_exercise
            WHERE workout_template_id = $1
            ORDER BY id
            "#,
        )
        .bind(template_id as i64)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| WorkoutTemplateModelError::DatabaseError(e.to_string()))?;

        let mut exercises = Vec::new();
        for link in links {
            let exercise = self
                .exercise_model
                .get_exercise_by_id(link.exercise_id)
                .await
                .map_err(|e| WorkoutTemplateModelError::DatabaseError(format!("{:?}", e)))?;
            exercises.push(exercise);
        }

        Ok(WorkoutTemplate {
            id: row.id,
            name: row.name,
            exercises,
        })
    }

    async fn get_all_workout_templates(
        &self,
    ) -> Result<Vec<WorkoutTemplate>, WorkoutTemplateModelError> {
        let rows: Vec<SqliteWorkoutTemplate> =
            sqlx::query_as("SELECT id, name FROM workout_template")
                .fetch_all(&self.db_pool)
                .await
                .map_err(|e| WorkoutTemplateModelError::DatabaseError(e.to_string()))?;

        let mut templates = Vec::new();
        for row in rows {
            templates.push(self.get_workout_template(row.id).await?);
        }
        Ok(templates)
    }

    async fn update_workout_template(
        &mut self,
        template: WorkoutTemplate,
    ) -> Result<(), WorkoutTemplateModelError> {
        let result = sqlx::query("UPDATE workout_template SET name = $1 WHERE id = $2")
            .bind(&template.name)
            .bind(template.id as i64)
            .execute(&self.db_pool)
            .await
            .map_err(|e| WorkoutTemplateModelError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(WorkoutTemplateModelError::NotFound);
        }

        sqlx::query("DELETE FROM workout_template_exercise WHERE workout_template_id = $1")
            .bind(template.id as i64)
            .execute(&self.db_pool)
            .await
            .map_err(|e| WorkoutTemplateModelError::DatabaseError(e.to_string()))?;

        let exercise_ids: Vec<u64> = template.exercises.iter().map(|e| e.id).collect();
        self.insert_template_exercises(template.id, &exercise_ids)
            .await?;
        Ok(())
    }

    async fn delete_workout_template(
        &mut self,
        template_id: u64,
    ) -> Result<(), WorkoutTemplateModelError> {
        let result = sqlx::query("DELETE FROM workout_template WHERE id = $1")
            .bind(template_id as i64)
            .execute(&self.db_pool)
            .await
            .map_err(|e| WorkoutTemplateModelError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(WorkoutTemplateModelError::NotFound);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::traits::workout_template_model::WorkoutTemplateModel,
        outbound::exercise_repository::ExerciseRepository,
    };
    use std::sync::Arc;

    fn make_repo(pool: sqlx::SqlitePool) -> (WorkoutTemplateRepository, Arc<ExerciseRepository>) {
        let exercise_repo = Arc::new(ExerciseRepository::from_pool(pool.clone()));
        let template_repo = WorkoutTemplateRepository::new(pool, exercise_repo.clone());
        (template_repo, exercise_repo)
    }

    #[sqlx::test]
    async fn get_workout_template_returns_template_when_exists(pool: sqlx::SqlitePool) {
        sqlx::query("INSERT INTO workout_template (name) VALUES ('Legs')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO exercise (name, exercise_type, goal_weight) VALUES ('squat', 'weighted', 60.0)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO workout_template_exercise (workout_template_id, exercise_id) VALUES (1, 1)")
            .execute(&pool)
            .await
            .unwrap();

        let (repo, _) = make_repo(pool);
        let template = repo.get_workout_template(1).await.expect("fetch should succeed");

        assert_eq!(template.id, 1);
        assert_eq!(template.name, "Legs");
        assert_eq!(template.exercises.len(), 1);
        assert_eq!(template.exercises[0].name, "squat");
    }

    #[sqlx::test]
    async fn get_workout_template_returns_not_found_when_missing(pool: sqlx::SqlitePool) {
        let (repo, _) = make_repo(pool);
        let result = repo.get_workout_template(999).await;
        assert!(matches!(result, Err(WorkoutTemplateModelError::NotFound)));
    }

    #[sqlx::test]
    async fn get_all_workout_templates_returns_empty_when_none(pool: sqlx::SqlitePool) {
        let (repo, _) = make_repo(pool);
        let templates = repo.get_all_workout_templates().await.expect("should succeed");
        assert!(templates.is_empty());
    }

    #[sqlx::test]
    async fn create_workout_template_inserts_template_and_exercises(pool: sqlx::SqlitePool) {
        sqlx::query("INSERT INTO exercise (name, exercise_type, goal_reps) VALUES ('pushup', 'bodyweight_reps', 10)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO exercise (name, exercise_type, goal_weight) VALUES ('squat', 'weighted', 60.0)")
            .execute(&pool)
            .await
            .unwrap();

        let (mut repo, _) = make_repo(pool);
        let template = NewWorkoutTemplate {
            name: "Full body".to_string(),
            exercise_ids: vec![1, 2],
        };

        let id = WorkoutTemplateModel::create_workout_template(&mut repo, template)
            .await
            .expect("create should succeed");

        assert_eq!(id, 1);
        let fetched = repo.get_workout_template(1).await.expect("should exist");
        assert_eq!(fetched.name, "Full body");
        assert_eq!(fetched.exercises.len(), 2);
    }

    #[sqlx::test]
    async fn update_workout_template_modifies_existing(pool: sqlx::SqlitePool) {
        sqlx::query("INSERT INTO workout_template (name) VALUES ('Legs')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO exercise (name, exercise_type, goal_weight) VALUES ('squat', 'weighted', 60.0)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO workout_template_exercise (workout_template_id, exercise_id) VALUES (1, 1)")
            .execute(&pool)
            .await
            .unwrap();

        let (mut repo, _) = make_repo(pool);
        let template = WorkoutTemplate {
            id: 1,
            name: "Lower body".to_string(),
            exercises: vec![crate::domain::types::exercise::Exercise {
                id: 1,
                name: "squat".to_string(),
                exercise_type: crate::domain::types::exercise::ExerciseType::Weighted {
                    goal_weight: 60.0,
                },
            }],
        };

        WorkoutTemplateModel::update_workout_template(&mut repo, template)
            .await
            .expect("update should succeed");

        let fetched = repo.get_workout_template(1).await.expect("should exist");
        assert_eq!(fetched.name, "Lower body");
    }

    #[sqlx::test]
    async fn update_workout_template_returns_not_found_when_missing(pool: sqlx::SqlitePool) {
        let (mut repo, _) = make_repo(pool);
        let template = WorkoutTemplate {
            id: 999,
            name: "phantom".to_string(),
            exercises: vec![],
        };

        let result = WorkoutTemplateModel::update_workout_template(&mut repo, template).await;
        assert!(matches!(result, Err(WorkoutTemplateModelError::NotFound)));
    }

    #[sqlx::test]
    async fn delete_workout_template_removes_existing(pool: sqlx::SqlitePool) {
        sqlx::query("INSERT INTO workout_template (name) VALUES ('Legs')")
            .execute(&pool)
            .await
            .unwrap();

        let (mut repo, _) = make_repo(pool);
        WorkoutTemplateModel::delete_workout_template(&mut repo, 1)
            .await
            .expect("delete should succeed");

        let result = repo.get_workout_template(1).await;
        assert!(matches!(result, Err(WorkoutTemplateModelError::NotFound)));
    }

    #[sqlx::test]
    async fn delete_workout_template_returns_not_found_when_missing(pool: sqlx::SqlitePool) {
        let (mut repo, _) = make_repo(pool);
        let result = WorkoutTemplateModel::delete_workout_template(&mut repo, 999).await;
        assert!(matches!(result, Err(WorkoutTemplateModelError::NotFound)));
    }
}
