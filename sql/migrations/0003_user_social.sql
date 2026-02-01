CREATE TABLE user_follow (
    follower_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    following_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (follower_id, following_id)
);

CREATE INDEX idx_user_follow_following ON user_follow(following_id);

CREATE TABLE user_stats (
    owner_id VARCHAR(25) PRIMARY KEY REFERENCES "user"(id) ON DELETE CASCADE,
    posts_count BIGINT NOT NULL DEFAULT 0,
    followers_count BIGINT NOT NULL DEFAULT 0,
    following_count BIGINT NOT NULL DEFAULT 0
);
