PRAGMA foreign_keys = ON;

CREATE TABLE Groups (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL -- not unqiue
    -- direction_id INTEGER, -- TODO
    -- speciality_id INTEGER -- TODO
) WITHOUT ROWID;

CREATE TABLE Teachers (
    id INTEGER PRIMARY KEY,
    full_name TEXT NOT NULL,
    short_name TEXT NOT NULL
    -- department_id INTEGER NOT NULL -- TODO
) WITHOUT ROWID;

CREATE TABLE Subjects (
    id INTEGER PRIMARY KEY,
    abbr TEXT UNIQUE NOT NULL,
    name TEXT UNIQUE NOT NULL
) WITHOUT ROWID;

CREATE TABLE Auditoriums (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,

    floor INTEGER NOT NULL,
    has_power INTEGER NOT NULL, -- CHECK (has_power IN (0, 1))
    building TEXT NOT NULL
) WITHOUT ROWID;


CREATE TABLE EventKind (
    -- ROWID
    abbr TEXT UNIQUE NOT NULL,
    name TEXT UNIQUE NOT NULL
);

INSERT INTO EventKind (abbr, name) VALUES
('Лк', 'Лекція'),
('Пз', 'Практичне заняття'),
('Лб', 'Лабороторна робота'),
('Конс', 'Консультація'),
('Зал', 'Залік'),
('Екз', 'Екзамен'),
('КП/КР', 'Контрольний пункт / Курсова робота');

CREATE TABLE Events (
    id INTEGER PRIMARY KEY,
    -- kind TEXT CHECK (kind IN ('Лк', 'Пз', 'Лб', 'Конс', 'Зал', 'Екз', 'КП/КР')) NOT NULL,
    kind INTEGER NOT NULL REFERENCES EventKind(rowid) ON DELETE CASCADE,
    subject_id INTEGER NOT NULL REFERENCES Subjects(id) ON DELETE CASCADE,
    auditorium_id INTEGER NOT NULL REFERENCES Auditoriums(id) ON DELETE CASCADE,
    count INTEGER NOT NULL, -- sequence number of the particular class
    starts_at TEXT NOT NULL,
    ends_at TEXT NOT NULL
) WITHOUT ROWID;

CREATE TABLE EventGroups (
    event_id INTEGER REFERENCES Events(id) ON DELETE CASCADE,
    group_id INTEGER REFERENCES Groups(id) ON DELETE CASCADE,
    PRIMARY KEY (event_id, group_id)
) WITHOUT ROWID;

CREATE TABLE EventTeachers (
    event_id INTEGER REFERENCES Events(id) ON DELETE CASCADE,
    teacher_id INTEGER REFERENCES Teachers(id) ON DELETE CASCADE,
    PRIMARY KEY (event_id, teacher_id)
) WITHOUT ROWID;


CREATE TABLE Fetches (
    fetched_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE GroupFetches (
    group_id INTEGER REFERENCES Groups(id) ON DELETE CASCADE,
    fetch_id INTEGER REFERENCES Fetches(rowid) ON DELETE CASCADE,
    PRIMARY KEY (group_id, fetch_id)
) WITHOUT ROWID;

CREATE TABLE TeacherFetches (
    teacher_id INTEGER REFERENCES Teachers(id) ON DELETE CASCADE,
    fetch_id INTEGER REFERENCES Fetches(rowid) ON DELETE CASCADE,
    PRIMARY KEY (teacher_id, fetch_id)
) WITHOUT ROWID;

CREATE TABLE AuditoriumFetches (
    auditorium_id INTEGER REFERENCES Auditoriums(id) ON DELETE CASCADE,
    fetch_id INTEGER REFERENCES Fetches(rowid) ON DELETE CASCADE,
    PRIMARY KEY (auditorium_id, fetch_id)
) WITHOUT ROWID;
