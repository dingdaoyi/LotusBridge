create table plugin_config
(
    id                 INTEGER primary key autoincrement,
    description        TEXT ,
    form_customization TEXT,
    plugin_type        TEXT,
    name               text default '' not null
);

create unique index plugin_config_name_index
    on plugin_config (name);


CREATE TABLE tb_device
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT UNIQUE NOT NULL,
    device_type TEXT        NOT NULL,
    custom_data TEXT,
    protocol_name TEXT     NOT NULL
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
