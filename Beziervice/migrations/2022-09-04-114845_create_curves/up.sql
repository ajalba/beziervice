CREATE TABLE curves (
  id SERIAL PRIMARY KEY,
  curve_name TEXT NOT NULL,
  expression_x TEXT NOT NULL,
  expression_y TEXT NOT NULL,
  expression_z TEXT NOT NULL,
  points_x float8 [] NOT NULL,
  points_y float8 [] NOT NULL,
  points_z float8 [] NOT NULL
)