CREATE TABLE exercise (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  exercise_type TEXT NOT NULL CHECK (exercise_type IN ('weighted', 'bodyweight_reps', 'bodyweight_time')),
  progress_from INTEGER REFERENCES exercise(id) ON DELETE SET NULL,
  progress_to INTEGER REFERENCES exercise(id) ON DELETE SET NULL
);

CREATE TABLE goal (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  exercise_id INTEGER NOT NULL REFERENCES exercise(id) ON DELETE CASCADE,
  reps INTEGER,
  weight REAL,
  duration_seconds INTEGER,
  UNIQUE (exercise_id)
);

CREATE TABLE workout_template (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  description TEXT
);

CREATE TABLE workout_template_exercise (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  workout_template_id INTEGER NOT NULL REFERENCES workout_template(id) ON DELETE CASCADE,
  exercise_id INTEGER NOT NULL REFERENCES exercise(id) ON DELETE CASCADE
  UNIQUE (workout_template_id, exercise_id)
);

CREATE TABLE workout (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  date DATE NOT NULL,
  mood INTEGER CHECK (mood >= 0 AND mood <= 255)
);

CREATE TABLE set (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  workout_id INTEGER NOT NULL REFERENCES workout(id) ON DELETE CASCADE,
  exercise_id INTEGER NOT NULL REFERENCES exercise(id) ON DELETE CASCADE,
  set_order INTEGER NOT NULL,
  reps INTEGER,
  weight REAL,
  duration_seconds INTEGER,
  failure INTEGER NOT NULL DEFAULT 0
);
