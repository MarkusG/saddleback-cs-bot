CREATE USER saddlebot;
GRANT SELECT ON member TO saddlebot;
GRANT INSERT ON member TO saddlebot;
GRANT UPDATE ON member TO saddlebot;

CREATE TABLE member (
	id           BIGINT PRIMARY KEY,
	is_professor BOOLEAN NOT NULL DEFAULT false,
	first_name   TEXT,
	last_name    TEXT,
	courses      TEXT[]
);
