CREATE TABLE recommended_place (
    id VARCHAR(25) PRIMARY KEY,
    location_id VARCHAR(25) NOT NULL REFERENCES location(id),
    name VARCHAR(256) NOT NULL,
    category VARCHAR(50) NOT NULL, 
    description TEXT,
    rating DECIMAL(3, 2),
    price_level INT,
    recommended_by_user_id VARCHAR(25) REFERENCES "user"(id),
    
    -- External IDs from Map Provider
    poi_id VARCHAR(100),
    
    -- Statistics
    save_count INT DEFAULT 0,
    view_count INT DEFAULT 0,
    
    ctime TIMESTAMPTZ DEFAULT now(),
    mtime TIMESTAMPTZ DEFAULT now()
);