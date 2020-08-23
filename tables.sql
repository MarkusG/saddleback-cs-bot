CREATE USER saddlebot;
GRANT SELECT ON member TO saddlebot;
GRANT INSERT ON member TO saddlebot;
GRANT UPDATE ON member TO saddlebot;

CREATE TYPE course AS ENUM ('1A', '1B', '1C', '1D', '3A', '3B', '4A', 'CIMS140', 'CIMS150');

CREATE TABLE member (
	id           BIGINT PRIMARY KEY,
	is_professor BOOLEAN NOT NULL DEFAULT false,
	first_name   TEXT,
	last_name    TEXT,
	courses      course[]
);
