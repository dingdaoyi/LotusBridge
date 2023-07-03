CREATE TABLE IF NOT EXISTS plugin_config
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    description        TEXT,
    form_customization TEXT,
    plugin_type        TEXT
);

CREATE TABLE IF NOT EXISTS protocol_config
(
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    name             TEXT UNIQUE NOT NULL,
    path             TEXT,
    description      TEXT,
    plugin_config_id INTEGER,
    FOREIGN KEY (plugin_config_id) REFERENCES plugin_config (id)
);

CREATE TABLE tb_device
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT UNIQUE NOT NULL,
    device_type TEXT        NOT NULL,
    custom_data TEXT,
    protocol_id INTEGER     NOT NULL
);

CREATE TABLE tb_point
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    device_id   INTEGER REFERENCES tb_device (id) ON DELETE CASCADE,
    address     TEXT    NOT NULL,
    data_type   TEXT    NOT NULL,
    access_mode TEXT    NOT NULL,
    multiplier  REAL    NOT NULL,
    precision   INTEGER NOT NULL,
    description TEXT    NOT NULL,
    part_number TEXT,
    UNIQUE (device_id, address)
);
