CREATE TABLE setlists(
    id uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    live_id uuid NOT NULL,
    track_id uuid NOT NULL,
    track_order INTEGER NOT NULL CHECK(track_order > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(track_id) REFERENCES tracks(id),
    FOREIGN KEY(live_id) REFERENCES lives(live_id)
);
