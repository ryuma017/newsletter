-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '0c7078ea-83d6-4b72-a1be-57238d7e3ffc',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$GW+L5YTS2DtDchOjG/4paw$aWvqqSugPJKTcgaIvZRbev9vzNOv+tuK559Tx8xaYHE'
);