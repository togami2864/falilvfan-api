CREATE TABLE locations(
    id uuid PRIMARY KEY NOT NULL,
    location text UNIQUE NOT NULL,
    prefecture_id SERIAL UNIQUE NOT NULL,
    FOREIGN KEY(prefecture_id) REFERENCES prefectures(id)
);