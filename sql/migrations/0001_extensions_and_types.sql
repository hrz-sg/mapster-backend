-- Extensions
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Enums
CREATE TYPE user_typ AS ENUM ('Sys', 'User');
CREATE TYPE comment_entity_type AS ENUM ('Post');
CREATE TYPE event_status AS ENUM ('Planning', 'Active', 'Completed', 'Cancelled');
CREATE TYPE chat_type AS ENUM ('Group', 'Direct');
CREATE TYPE message_type AS ENUM ('Text', 'Post', 'Journey');
