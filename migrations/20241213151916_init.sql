CREATE TABLE category (
    code TEXT NOT NULL,
    label TEXT NOT NULL default '',
    UNIQUE (code)
);

CREATE TABLE category_pattern (
    pattern TEXT NOT NULL,
    category TEXT NOT NULL,
    unique (pattern),
    FOREIGN KEY (category) REFERENCES category (code)
);

CREATE TABLE transfer (
    date TEXT NOT NULL,
    label TEXT NOT NULL,
    value REAL NOT NULL,
    category TEXT,
    UNIQUE (date, label, value),
    FOREIGN KEY (category) REFERENCES category (code)
)
