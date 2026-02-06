CREATE TABLE exercise (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  exercise_type TEXT NOT NULL CHECK (exercise_type IN ('weighted', 'bodyweight_reps', 'bodyweight_time')),
  -- Progress tracking: if an exercise is a progression of another, these fields will reference the related exercises
  progression_name TEXT,
  progression_order INTEGER CHECK (progression_order IS NULL OR progression_order > 0),
  -- Goals to reach
  goal_reps INTEGER CHECK (goal_reps IS NULL OR goal_reps > 0),
  goal_weight REAL CHECK (goal_weight IS NULL OR goal_weight > 0),
  goal_duration_seconds INTEGER CHECK (goal_duration_seconds IS NULL OR goal_duration_seconds > 0),
  UNIQUE (progression_name, progression_order)
);

CREATE INDEX idx_exercise_progression ON exercise(progression_name, progression_order);

CREATE TABLE workout_template (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
);

CREATE TABLE workout_template_exercise (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  workout_template_id INTEGER NOT NULL REFERENCES workout_template(id) ON DELETE CASCADE,
  exercise_id INTEGER NOT NULL REFERENCES exercise(id) ON DELETE CASCADE,
  UNIQUE (workout_template_id, exercise_id)
);

CREATE INDEX idx_workout_template_exercise_template_id ON workout_template_exercise(workout_template_id);
CREATE INDEX idx_workout_template_exercise_exercise_id ON workout_template_exercise(exercise_id);

CREATE TABLE workout (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  date DATE NOT NULL,
  mood INTEGER CHECK (mood >= 1 AND mood <= 10)
);

CREATE INDEX idx_workout_date ON workout(date);

CREATE TABLE workout_set (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  workout_id INTEGER NOT NULL REFERENCES workout(id) ON DELETE CASCADE,
  exercise_id INTEGER NOT NULL REFERENCES exercise(id) ON DELETE CASCADE,
  set_order INTEGER NOT NULL CHECK (set_order > 0),
  reps INTEGER CHECK (reps IS NULL OR reps > 0),
  weight REAL CHECK (weight IS NULL OR weight > 0),
  duration_seconds INTEGER CHECK (duration_seconds IS NULL OR duration_seconds > 0),
  failure INTEGER NOT NULL DEFAULT 0 CHECK (failure IN (0, 1))
);

CREATE INDEX idx_workout_set_workout_id ON workout_set(workout_id);
CREATE INDEX idx_workout_set_exercise_id ON workout_set(exercise_id);
CREATE INDEX idx_workout_set_workout_exercise ON workout_set(workout_id, exercise_id);
