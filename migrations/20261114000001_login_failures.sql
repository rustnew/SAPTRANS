CREATE TABLE member_login_failures (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    email VARCHAR(320) NOT NULL,
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX member_login_failures_email_time_idx ON member_login_failures(email, attempted_at DESC);
