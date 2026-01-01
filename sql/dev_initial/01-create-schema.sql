---- Base app schema

-- User
CREATE TYPE user_typ AS ENUM ('Sys', 'User');

CREATE TABLE "user" (
    id VARCHAR(30) PRIMARY KEY,
    typ user_typ NOT NULL DEFAULT 'User',
    username VARCHAR(128) NOT NULL UNIQUE,
    email VARCHAR(256) NOT NULL UNIQUE,
    avatar_url TEXT,
    bio TEXT,
    location TEXT,

    -- Auth
    pwd VARCHAR(256),
    pwd_salt UUID NOT NULL DEFAULT gen_random_uuid(),

    -- Token
    token_salt UUID NOT NULL DEFAULT gen_random_uuid(),
    reset_token TEXT,
    reset_token_expires_at TIMESTAMPTZ,

    -- Email verification
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    email_verification_token VARCHAR(255),
    email_verification_expires_at TIMESTAMPTZ,

    -- Timestamps / Audit
    cid VARCHAR(30) REFERENCES "user"(id),
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Followers & Followings
CREATE TABLE user_follow (
    follower_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    following_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (follower_id, following_id)
);

-- Stats
CREATE TABLE user_stats (
    user_id VARCHAR(30) PRIMARY KEY REFERENCES "user"(id) ON DELETE CASCADE,
    posts_count BIGINT NOT NULL DEFAULT 0,
    followers_count BIGINT NOT NULL DEFAULT 0,
    following_count BIGINT NOT NULL DEFAULT 0
);

-- Post
CREATE TABLE post (
    id VARCHAR(30) PRIMARY KEY,
    user_id VARCHAR(30) NOT NULL REFERENCES "user"(id),
    title VARCHAR(256) NOT NULL,
    description TEXT NOT NULL,
    is_published BOOLEAN NOT NULL DEFAULT FALSE,
    cover_media_url TEXT,
    thumbnail_url TEXT,
    media_count INT NOT NULL DEFAULT 0,
    has_video BOOLEAN NOT NULL DEFAULT FALSE,
    like_count BIGINT NOT NULL DEFAULT 0,
    comment_count BIGINT NOT NULL DEFAULT 0,
    saved_count BIGINT NOT NULL DEFAULT 0,

    -- Timestamps / Audit
    cid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Post Likes
CREATE TABLE post_like (
    post_id VARCHAR(30) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    user_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (post_id, user_id)
);

-- PostMedia
CREATE TABLE post_media (
    id VARCHAR(30) PRIMARY KEY,
    post_id VARCHAR(30) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    media_url TEXT NOT NULL,
    media_type VARCHAR(16) NOT NULL,
    mime_type VARCHAR(128) NOT NULL,
    width INT,
    height INT,
    file_size BIGINT,
    duration INT,
    sort_order INT NOT NULL DEFAULT 0,

    -- Timestamps / Audit
    cid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Comments
CREATE TYPE comment_entity_typ AS ENUM ('Post');

CREATE TABLE comment (
    id VARCHAR(30) PRIMARY KEY,
    user_id VARCHAR(30) NOT NULL REFERENCES "user"(id),
    entity_type comment_entity_typ NOT NULL,
    entity_id VARCHAR(30) NOT NULL,
    parent_id VARCHAR(30) REFERENCES comment(id) ON DELETE CASCADE,
    text TEXT NOT NULL,

    -- Timestamps / Audit
    cid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- CommentMedia
CREATE TABLE comment_media (
    id VARCHAR(30) PRIMARY KEY,
    comment_id VARCHAR(30) NOT NULL REFERENCES comment(id) ON DELETE CASCADE,
    media_url TEXT NOT NULL,
    media_type VARCHAR(16) NOT NULL CHECK (media_type = 'image'),
    mime_type VARCHAR(128) NOT NULL,
    width INT,
    height INT,
    file_size BIGINT,
    sort_order INT NOT NULL DEFAULT 0,
    is_blocked BOOLEAN NOT NULL DEFAULT FALSE,
    moderation_reason TEXT,

    -- Timestamps / Audit
    cid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Journey
CREATE TABLE journey (
    id VARCHAR(30) PRIMARY KEY,
    user_id VARCHAR(30) NOT NULL REFERENCES "user"(id),
    title VARCHAR(256) NOT NULL,
    description TEXT,
    is_published BOOLEAN NOT NULL DEFAULT FALSE,
    like_count BIGINT NOT NULL DEFAULT 0,
    saved_count BIGINT NOT NULL DEFAULT 0,

    -- Timestamps / Audit
    cid VARCHAR(30) REFERENCES "user"(id),
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) REFERENCES "user"(id),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE journey_post (
    journey_id VARCHAR(30) NOT NULL REFERENCES journey(id) ON DELETE CASCADE,
    post_id VARCHAR(30) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    sort_order INT NOT NULL,
    PRIMARY KEY (journey_id, post_id),
    UNIQUE (journey_id, sort_order),

    -- Timestamps
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- User saved content
CREATE TYPE saved_content_type AS ENUM ('Post', 'Journey');

CREATE TABLE saved_content(
    id VARCHAR(30) PRIMARY KEY,
    user_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    content_type saved_content_type NOT NULL, -- 'post' or 'journey'

    -- Original content
    original_post_id VARCHAR(30) REFERENCES post(id) ON DELETE SET NULL,
    original_journey_id VARCHAR(30) REFERENCES journey(id) ON DELETE SET NULL,

    -- User settings
    custom_title VARCHAR(256),
    is_favorite BOOLEAN NOT NULL DEFAULT FALSE,

    -- Timestamps / Audit
    cid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Saved content constraints
ALTER TABLE saved_content ADD CONSTRAINT chk_saved_content_type 
CHECK (
    (content_type = 'Post' AND original_post_id IS NOT NULL AND original_journey_id IS NULL) OR
    (content_type = 'Journey' AND original_journey_id IS NOT NULL AND original_post_id IS NULL)
);

-- User collection
CREATE TYPE collection_type AS ENUM ('Posts', 'Journeys');

CREATE TABLE user_collection(
    id VARCHAR(30) PRIMARY KEY,
    user_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    title VARCHAR(128) NOT NULL,
    sort_order INT DEFAULT 0,
    is_default BOOLEAN NOT NULL DEFAULT FALSE, -- for "all posts" & "journeys"

    collection_type collection_type NOT NULL,

    cid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE collection_item(
    id VARCHAR(30) PRIMARY KEY,
    collection_id VARCHAR(30) NOT NULL REFERENCES user_collection(id) ON DELETE CASCADE,
    saved_content_id VARCHAR(30) NOT NULL REFERENCES saved_content(id) ON DELETE CASCADE,

    sort_order INT DEFAULT 0,
    
    cid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) NOT NULL REFERENCES "user"(id),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE(collection_id, saved_content_id)
);