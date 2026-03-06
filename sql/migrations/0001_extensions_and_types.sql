-- Extensions
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Enums
CREATE TYPE user_typ AS ENUM ('Sys', 'User');
CREATE TYPE comment_entity_type AS ENUM ('Post');
CREATE TYPE event_status AS ENUM ('Planning', 'Active', 'Completed', 'Cancelled');
CREATE TYPE chat_type AS ENUM ('Group', 'Direct');
CREATE TYPE message_type AS ENUM ('Text', 'Post', 'Journey');
CREATE TYPE post_status AS ENUM ( 'Draft', 'Published');
CREATE TYPE journey_status AS ENUM ( 'Draft', 'Published');
CREATE TYPE media_status AS ENUM ('Draft', 'Published');
CREATE TYPE media_variant_type AS ENUM ('Thumb', 'Medium', 'Full', 'Preview', 'P360', 'P720', 'P1080');
CREATE TYPE media_type AS ENUM ('Image','Video');
