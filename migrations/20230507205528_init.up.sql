CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS test (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        fname VARCHAR(255) NOT NULL UNIQUE,
        lname VARCHAR(255) NOT NULL UNIQUE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );
