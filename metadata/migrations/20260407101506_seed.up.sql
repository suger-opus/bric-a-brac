INSERT INTO users (user_id, email, username)
VALUES ('019d1f20-d9a3-7ff3-836f-9c6b24e5baaf', 'dev@bric-a-brac.local', 'dev')
ON CONFLICT (user_id) DO NOTHING;
