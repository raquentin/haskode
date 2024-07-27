CREATE TABLE problems (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    difficulty problem_difficulty NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
