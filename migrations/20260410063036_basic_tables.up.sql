-- Create the users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id VARCHAR(100) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    password TEXT NOT NULL,
    role VARCHAR(50) NOT NULL
);

-- Create the lectures table
CREATE TABLE lectures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date DATE NOT NULL
);

-- Create the attendance table (One-to-Many from lectures)
CREATE TABLE attendance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lecture_id UUID NOT NULL REFERENCES lectures(id) ON DELETE CASCADE,
    
    -- Links to the unique student_id in the users table
    student_id VARCHAR(100) NOT NULL REFERENCES users(student_id) ON DELETE CASCADE,
    
    date DATE NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
