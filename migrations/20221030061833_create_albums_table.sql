CREATE TABLE albums(
    album_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    album_name text UNIQUE NOT NULL,
    spotify_id text UNIQUE NOT NULL,
    is_single boolean NOT NULL,
    release_date date NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
)