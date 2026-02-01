-- Journey Collection
CREATE TABLE journey_collection (
    id VARCHAR(25) PRIMARY KEY,
    owner_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    title VARCHAR(128) NOT NULL DEFAULT 'Journeys',
    is_default BOOLEAN DEFAULT TRUE,
    ctime TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE journey_collection_item (
    id VARCHAR(25) PRIMARY KEY,
    collection_id VARCHAR(25) NOT NULL REFERENCES journey_collection(id) ON DELETE CASCADE,
    journey_id VARCHAR(25) NOT NULL REFERENCES journey(id) ON DELETE CASCADE,
    ctime TIMESTAMPTZ DEFAULT now(),
    UNIQUE(collection_id, journey_id)
);