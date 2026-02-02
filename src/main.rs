use axum::{Router, routing::get};
use sport_tracker::types::{goal::*, workout::*};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    create_goals();
    read_goals();

    // let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

fn read_goals() {
    let goals_string = r#"
- name: Squat
  target: !Weighted
    target_weight: 100.0
    target_reps: 5
    target_sets: null
- name: Push-up
  target: !BodyweightReps
    progression:
    - name: Diamond Push-up
      target:
        target_reps: 15
        target_sets: 3
    - name: Standard Push-up
      target:
        target_reps: 20
        target_sets: 5
- name: Handstand
  target: !BodyweightTime
    progression:
    - name: Handstand
      target:
        target_duration_seconds: 30
        target_sets: 3
    - name: Wall Handstand
      target:
        target_duration_seconds: 60
        target_sets: 3
    "#;

    let goals: Vec<Goal> = serde_yml::from_str(goals_string).unwrap();
    println!("{:#?}", goals);
}

fn create_goals() {
    let squat_goal = Goal {
        name: "Squat".to_string(),
        target: ExerciseTarget::Weighted {
            target_weight: 100.0,
            target_reps: Some(5),
            target_sets: None,
        },
    };
    let pushup_goal = Goal {
        name: "Push-up".to_string(),
        target: ExerciseTarget::BodyweightReps {
            progression: vec![
                BodyweightRepsProgression {
                    name: "Diamond Push-up".to_string(),
                    target: BodyweightRepsTarget {
                        target_reps: 15,
                        target_sets: Some(3),
                    },
                },
                BodyweightRepsProgression {
                    name: "Standard Push-up".to_string(),
                    target: BodyweightRepsTarget {
                        target_reps: 20,
                        target_sets: Some(5),
                    },
                },
            ],
        },
    };
    let handstand_goal = Goal {
        name: "Handstand".to_string(),
        target: ExerciseTarget::BodyweightTime {
            progression: vec![
                BodyweightTimeProgression {
                    name: "Handstand".to_string(),
                    target: BodyweightTimeTarget {
                        target_duration_seconds: 30,
                        target_sets: Some(3),
                    },
                },
                BodyweightTimeProgression {
                    name: "Wall Handstand".to_string(),
                    target: BodyweightTimeTarget {
                        target_duration_seconds: 60,
                        target_sets: Some(3),
                    },
                },
            ],
        },
    };

    let _goals: Vec<Goal> = vec![squat_goal, pushup_goal, handstand_goal];
}
