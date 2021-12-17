CREATE TABLE users (
    user_id BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    username VARCHAR(20) NOT NULL,
    bcrypted_password CHAR(60) NOT NULL,
    creation_time_utc BIGINT UNSIGNED NOT NULL
);
CREATE UNIQUE INDEX idx_username
ON users (username);

CREATE TABLE posts (
    post_id BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    creator BIGINT UNSIGNED NOT NULL,
    creation_time_utc BIGINT UNSIGNED NOT NULL,
    last_modified_utc BIGINT UNSIGNED,
    title VARCHAR(255) NOT NULL,
    post_type VARCHAR(15) NOT NULL,
    content TEXT NOT NULL,
    CONSTRAINT UC_title UNIQUE (title)
);

CREATE TABLE admins (
    admin_id BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    bcrypted_password CHAR(60) NOT NULL
);
