CREATE TABLE rust_foos (
  id SERIAL,
  description VARCHAR (20),
  PRIMARY KEY (id)
);

INSERT INTO rust_foos (description) VALUES ('A FIRST FOO'), ('A SECOND FOO');

