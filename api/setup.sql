CREATE TABLE users (
    id BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    username VARCHAR(20) NOT NULL UNIQUE,
    bcrypted_password CHAR(60) NOT NULL,
    creation_time_utc BIGINT UNSIGNED NOT NULL,
    UNIQUE (username)
);

CREATE TABLE posts (
    id BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    creator BIGINT UNSIGNED NOT NULL,
    creation_time_utc BIGINT UNSIGNED NOT NULL,
    last_modified_utc BIGINT UNSIGNED,
    title VARCHAR(255) NOT NULL,
    post_type VARCHAR(15) NOT NULL,
    content TEXT NOT NULL,
    UNIQUE (title)
);

CREATE TABLE admins (
    id BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    bcrypted_password CHAR(60) NOT NULL
);
