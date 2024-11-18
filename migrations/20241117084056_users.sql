-- 1. Create the `users` table if it doesn't exist
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- Unique identifier for each user
    name TEXT NOT NULL, -- User's name
    password TEXT NOT NULL -- Hashed password (currently plaintext)
);

-- 2. Insert a default Admin user with plaintext password
INSERT INTO
    users (name, password)
VALUES (
        'Admin',
        '$argon2id$v=19$m=19456,t=2,p=1$FPLq4LNUILUFJssUNFtk5Q$MYcXhclig7w+iXKDj30/eyX0T+iK6LLYJVLGLrO9s0Q'
    );

-- 3. Rename the existing `codes` table to prepare for migration
ALTER TABLE codes RENAME TO codes_old;

-- 4. Create a new `codes` table with the `user_id` column required
CREATE TABLE IF NOT EXISTS codes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    code VARCHAR(255) UNIQUE NOT NULL,
    user_id INTEGER NOT NULL DEFAULT 1, -- Foreign key to the `users` table, defaults to Admin's ID
    created_at DATETIME DEFAULT (
        STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')
    ) NOT NULL
);

-- 5. Copy data from the old `codes` table to the new one, setting the default `user_id`
INSERT INTO
    codes (id, code, user_id)
SELECT id, code, 1
FROM codes_old;

-- 6. Drop the old table
DROP TABLE codes_old;

-- 7. Add an index for the `user_id` column
CREATE INDEX IF NOT EXISTS idx_codes_user_id ON codes (user_id);