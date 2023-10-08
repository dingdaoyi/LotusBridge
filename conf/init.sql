create table plugin_config
(
    id                 INTEGER primary key autoincrement,
    description        TEXT,
    form_customization TEXT,
    plugin_type        TEXT,
    name               text default '' not null
);

create unique index plugin_config_name_index
    on plugin_config (name);


CREATE TABLE tb_export_config
(
    id            INTEGER PRIMARY KEY,
    name          TEXT                                                    NOT NULL,
    configuration TEXT                                                    NOT NULL,
    description   TEXT                                                    NOT NULL,
    plugin_id     INTEGER REFERENCES plugin_config (id) ON DELETE CASCADE NOT NULL,
    UNIQUE (name, plugin_id)
);


CREATE TABLE tb_device
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    name          TEXT UNIQUE NOT NULL,
    device_type   TEXT        NOT NULL,
    custom_data   TEXT,
    protocol_name TEXT        NOT NULL
);

CREATE TABLE tb_device_group
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    name      TEXT    NOT NULL,
    interval  INTEGER NOT NULL DEFAULT 20,
    device_id INTEGER REFERENCES tb_device (id) ON DELETE CASCADE,
    UNIQUE (device_id, name)
);

-- auto-generated definition
create table tb_point
(
    id          INTEGER
        primary key autoincrement,
    device_id   INTEGER
        constraint tb_point_tb_device_id_fk
            references tb_device
            on delete cascade,
    address     TEXT    not null,
    data_type   TEXT    not null,
    access_mode TEXT    not null,
    multiplier  REAL    not null,
    precision   INTEGER not null,
    description TEXT    not null,
    part_number TEXT,
    group_id    integer,
    identifier  text,
    unique (device_id, address)
);



CREATE TABLE tb_export_group
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    export_id  INTEGER REFERENCES tb_export_config (id) ON DELETE CASCADE,
    group_id INTEGER REFERENCES tb_device_group (id) ON DELETE CASCADE,
    UNIQUE (group_id, export_id)
);

