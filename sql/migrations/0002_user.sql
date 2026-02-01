CREATE TABLE "user" (
    id VARCHAR(25) PRIMARY KEY,
    typ user_typ NOT NULL DEFAULT 'User',
    username VARCHAR(128) NOT NULL UNIQUE,
    email VARCHAR(256) NOT NULL UNIQUE,
    avatar_url TEXT,
    bio TEXT,
    location TEXT,

    pwd VARCHAR(256),
    pwd_salt UUID NOT NULL DEFAULT gen_random_uuid(),

    token_salt UUID NOT NULL DEFAULT gen_random_uuid(),
    reset_token TEXT,
    reset_token_expires_at TIMESTAMPTZ,

    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    email_verification_token VARCHAR(256),
    email_verification_expires_at TIMESTAMPTZ,

    deleted_at TIMESTAMPTZ,
    scheduled_permanent_deletion_at TIMESTAMPTZ,

    cid VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);
