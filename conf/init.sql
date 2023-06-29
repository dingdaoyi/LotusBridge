CREATE TABLE IF NOT EXISTS product_func (
                                            id INTEGER PRIMARY KEY,
                                            create_time DATETIME,
                                            update_time DATETIME,
                                            is_async INTEGER,
                                            data_type INTEGER,
                                            description TEXT,
                                            event_type INTEGER,
                                            func_status INTEGER,
                                            func_type INTEGER,
                                            has_rule_engine INTEGER,
                                            identifier TEXT,
                                            input_param TEXT,
                                            name TEXT,
                                            output_param TEXT,
                                            rule_engine TEXT,
                                            specs TEXT,
                                            is_wr_type INTEGER,
                                            product_type_id INTEGER,
                                            icon_id INTEGER,
                                            is_custom INTEGER,
                                            is_optional INTEGER,
                                            protected_service INTEGER
);


INSERT INTO "product_func" ("id", "create_time", "update_time", "is_async", "data_type", "description", "event_type", "func_status", "func_type", "has_rule_engine", "identifier", "input_param", "name", "output_param", "rule_engine", "specs", read_only, "product_type_id", "icon_id", custom, optional, "protected_service") VALUES (170661844276912128, '2021-07-15 15:13:43', '2022-07-04 14:37:16', 0, 8, '消音,异步操作,不等待结果返回', NULL, 1, 3, 0, 'silence', '[]', '消音', '[]', NULL, '{"name":"消音","identifier":"","dataType":0,"bool0":"","bool1":"","length":0,"unit":"","unitName":"","min":0,"max":0,"step":0.0,"enumMap":{}}', 0, 170644969589481472, 288841137368256512, 0, 1, 0);

INSERT INTO "product_func" ("id", "create_time", "update_time", "is_async", "data_type", "description", "event_type", "func_status", "func_type", "has_rule_engine", "identifier", "input_param", "name", "output_param", "rule_engine", "specs", read_only, "product_type_id", "icon_id", custom, optional, "protected_service") VALUES (208344170958888961, '2021-10-27 14:49:50', '2022-06-06 16:31:58', 0, 6, '温度', NULL, 1, 1, 0, 'temperature', '[]', '温度', '[]', NULL, '{"name":"温度","identifier":"","dataType":0,"bool0":"","bool1":"","length":0,"unit":"29","unitName":"","min":-200.0,"max":1000.0,"step":1.0,"enumMap":{}}', 0, 206207346073866240, 288815266401681408, 0, 1, 1);
