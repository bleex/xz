-- Your SQL goes here
create table links (
  id SERIAL PRIMARY KEY, 
  src VARCHAR NOT NULL, 
  dst VARCHAR NOT NULL DEFAULT ''
);
