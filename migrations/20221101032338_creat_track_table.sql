CREATE TABLE tracks(
    id uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    name text NOT NULL,
    track_number INTEGER NOT NULL,
    duration_ms INTERVAL NOT NULL,
    album_id uuid NOT NULL,
    youtube_url text NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(album_id) REFERENCES albums(album_id)
);