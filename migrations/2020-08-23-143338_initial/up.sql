-- Your SQL goes here

CREATE TABLE festivals (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    place TEXT NULL,
    site_url TEXT NULL,
    facebook_url TEXT NULL
);

CREATE TABLE events (
    id INTEGER NOT NULL PRIMARY KEY,
    festival_id INTEGER NOT NULL REFERENCES festivals(id),
    status INTEGER NOT NULL REFERENCES event_statuses(id),
    name TEXT NULL,
    description TEXT NULL,
    place TEXT NULL,
    site_url TEXT NULL,
    facebook_url TEXT NULL,
    register_url TEXT NULL,
    register_start_date INTEGER NULL,
    register_end_date INTEGER NULL,
    start_date INTEGER NOT NULL,
    end_date INTEGER NOT NULL
);

CREATE TABLE event_statuses (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

INSERT INTO event_statuses (id, name) VALUES
    (0, 'Unknown'), (1, 'Planned'), (2, 'Registered'),
    (3, 'Confirmed'), (4, 'Waitlist'), (5, 'Rejected'),
    (6, 'Canceled');