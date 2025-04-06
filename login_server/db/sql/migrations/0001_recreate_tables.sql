CREATE TABLE "user" (
    id UUID UNIQUE PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    
    -- Auth
    password VARCHAR(255) NOT NULL,
    encryption_salt UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),

    CONSTRAINT email_and_password_unique_together UNIQUE (email, password)
);

-- An index is a way to speed up queries
CREATE INDEX username_index ON "user" (name);