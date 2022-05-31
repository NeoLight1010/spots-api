CREATE TABLE group_users (
    group_id INT,
    user_id INT,

    PRIMARY KEY (group_id, user_id),

    CONSTRAINT fk_group FOREIGN KEY(group_id) REFERENCES groups(id),
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id)
)
