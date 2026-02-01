CREATE TABLE location (
    id VARCHAR(25) PRIMARY KEY,
    lat DECIMAL(10, 8) NOT NULL,
    lon DECIMAL(11, 8) NOT NULL,
    address TEXT NOT NULL,
    city VARCHAR(100) NOT NULL,
    province VARCHAR(100) NOT NULL,
    country_code CHAR(2) DEFAULT 'CN',
    display_name TEXT,

    gcj02_lat DECIMAL(10, 8),
    gcj02_lon DECIMAL(11, 8),

    ctime TIMESTAMPTZ DEFAULT now(),
    mtime TIMESTAMPTZ DEFAULT now()
);
