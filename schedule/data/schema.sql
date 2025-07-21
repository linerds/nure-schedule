PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;

CREATE TABLE IF NOT EXISTS Groups (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL -- not unqiue

    -- TODO: need to be fetched separately
    -- direction_id INTEGER,
    -- speciality_id INTEGER
) WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS Teachers (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
    -- short_name TEXT NOT NULL -- Compute this instead

    -- TODO: need to be fetched separately
    -- department_id INTEGER NOT NULL
) WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS Subjects (
    id INTEGER PRIMARY KEY,
    abbr TEXT UNIQUE NOT NULL,
    name TEXT UNIQUE NOT NULL
) WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS Auditoriums (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL

    -- TODO: need to be fetched separately, probably may be derived from name
    -- floor INTEGER NOT NULL,
    -- building TEXT NOT NULL,
    -- has_power INTEGER NOT NULL -- CHECK (has_power IN (0, 1))
) WITHOUT ROWID;


-- CREATE TABLE IF NOT EXISTS EventKind (
--     id INTEGER PRIMARY KEY,
--     abbr TEXT UNIQUE NOT NULL,
--     name TEXT UNIQUE NOT NULL
-- );

-- INSERT INTO EventKind (id, abbr, name) VALUES
-- (0, 'Лк', 'Лекція'),
-- (1, 'Пз', 'Практичне заняття'),
-- (2, 'Лб', 'Лабороторна робота'),
-- (3, 'Конс', 'Консультація'),
-- (4, 'Зал', 'Залік'),
-- (5, 'Екз', 'Екзамен'),
-- (6, 'КП/КР', 'Контрольний пункт / Курсова робота');

CREATE TABLE IF NOT EXISTS Events (
    id INTEGER PRIMARY KEY,
    subject_id INTEGER NOT NULL REFERENCES Subjects(id) ON DELETE CASCADE,
    auditorium_id INTEGER NOT NULL REFERENCES Auditoriums(id) ON DELETE CASCADE,
    -- kind TEXT CHECK (kind IN ('Лк', 'Пз', 'Лб', 'Конс', 'Зал', 'Екз', 'КП/КР')) NOT NULL,
    kind INTEGER NOT NULL, -- CHECK (kind >= 0 AND kind <= 6), -- REFERENCES EventKind(id) ON DELETE CASCADE,
    count INTEGER NOT NULL, -- sequence number of the particular class
    starts_at INTEGER NOT NULL
    -- ends_at INTEGER NOT NULL -- = starts_at + 1.5 hours
) WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS EventGroups (
    event_id INTEGER REFERENCES Events(id) ON DELETE CASCADE,
    group_id INTEGER REFERENCES Groups(id) ON DELETE CASCADE,
    PRIMARY KEY (event_id, group_id)
) WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS EventTeachers (
    event_id INTEGER REFERENCES Events(id) ON DELETE CASCADE,
    teacher_id INTEGER REFERENCES Teachers(id) ON DELETE CASCADE,
    PRIMARY KEY (event_id, teacher_id)
) WITHOUT ROWID;


-- TODO: Tracking info freshness
CREATE TABLE IF NOT EXISTS Fetches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fetched_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS GroupFetches (
    group_id INTEGER REFERENCES Groups(id) ON DELETE CASCADE,
    fetch_id INTEGER REFERENCES Fetches(id) ON DELETE CASCADE,
    PRIMARY KEY (group_id, fetch_id)
) WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS TeacherFetches (
    teacher_id INTEGER REFERENCES Teachers(id) ON DELETE CASCADE,
    fetch_id INTEGER REFERENCES Fetches(id) ON DELETE CASCADE,
    PRIMARY KEY (teacher_id, fetch_id)
) WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS AuditoriumFetches (
    auditorium_id INTEGER REFERENCES Auditoriums(id) ON DELETE CASCADE,
    fetch_id INTEGER REFERENCES Fetches(id) ON DELETE CASCADE,
    PRIMARY KEY (auditorium_id, fetch_id)
) WITHOUT ROWID;
