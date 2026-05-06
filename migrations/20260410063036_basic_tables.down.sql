--- Drop tables in reverse order of creation to prevent foreign key constraint errors
DROP TABLE IF EXISTS attendance;
DROP TABLE IF EXISTS lectures;
DROP TABLE IF EXISTS users;- Add down migration script here
