CREATE TABLE IF NOT EXISTS autoreplies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    trigger TEXT NOT NULL,
    reply TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_autoreplies_trigger ON autoreplies(trigger);
