CREATE TABLE IF NOT EXISTS visit_counter (
  id          smallint PRIMARY KEY CHECK (id = 1),
  total       bigint   NOT NULL DEFAULT 0,
  updated_at  timestamptz NOT NULL DEFAULT now()
);

INSERT INTO visit_counter (id, total) VALUES (1, 0)
ON CONFLICT (id) DO NOTHING;

CREATE OR REPLACE FUNCTION increment_visit() RETURNS bigint
LANGUAGE sql AS $$
  INSERT INTO visit_counter (id, total)
  VALUES (1, 1)
  ON CONFLICT (id) DO UPDATE
    SET total = visit_counter.total + 1,
        updated_at = now()
  RETURNING total;
$$;

CREATE OR REPLACE VIEW visit_count AS
SELECT total, updated_at FROM visit_counter WHERE id = 1;