CREATE TABLE lives(
    live_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    location_id uuid NOT NULL,
    date date NOT NULL,
    is_festival boolean NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(location_id) REFERENCES locations(id)
)