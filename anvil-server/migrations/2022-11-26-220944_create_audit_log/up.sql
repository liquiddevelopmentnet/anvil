CREATE TABLE audit_log (
    id SERIAL PRIMARY KEY,
    ip INET,
    action TEXT,
    description TEXT,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);