#!/usr/bin/env/ zx

const psql = await $`psql --version`;
if (psql.exitCode !== 0) {
  console.error("Error: psql is not installed.");
  await $`exit 1`;
}

const sqlx = await $`sqlx --version`;
if (sqlx.exitCode !== 0) {
  console.error("Error: sqlx is not installed.");
  await $`exit 1`;
}

const DB_USER = "postgres";
const DB_PASSWORD = "password";
const DB_NAME = "falilvfan";
const DB_PORT = 5432;

await $`docker run \
-e POSTGRES_USER=${DB_USER} \
-e POSTGRES_PASSWORD=${DB_PASSWORD} \
-e POSTGRES_DB=${DB_NAME} \
-p "${DB_PORT}":5432 \
-d postgres \
postgres -N 1000
`;

let isRunning = false;
while (isRunning) {
  const res =
    await $`PGPASSWORD="${DB_PASSWORD}" psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\\q'`;
  isRunning = res.exitCode === 0 ? true : false;
  if (!isRunning) {
    console.log("Postgres is still unavailable - sleeping");
    await sleep(1000);
  }
}

console.log(`Postgres is up and running on port ${DB_PORT}`);

const DATABASE_URL = `postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}`;
await $`sqlx database create --database-url ${DATABASE_URL}`;
