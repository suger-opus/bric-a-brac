# Metadata database

> PostgreSQL database

## Tables

**USER**
| NAME          | TYPE                          | CONSTRAINTS   |
| :------------ | :---------------------------- | :------------ |
| user_id       | ???                           | ???           |
| created_at    | ???                           | ???           |
| email         | ???                           | ???           |
| username      | ???                           | UNIQUE        |
| accesses      | ???                           | ???           |
| bookmarks     | ???                           | ???           |
| cheers        | ???                           | ???           |

**ACCESS**
| NAME          | TYPE                          | CONSTRAINTS   |
| :--------     | :---------------------------- | :------------ |
| user_id       | ???                           | ???           |
| graph_id      | ???                           | ???           |
| role          | ENUM (admin,writer,reader)    | ???           |

**BOOKMARK**
| NAME          | TYPE                          | CONSTRAINTS   |
| :--------     | :---------------------------- | :------------ |
| user_id       | ???                           | ???           |
| graph_id      | ???                           | ???           |
| created_at    | ???                           | ???           |

**CHEER**
| NAME          | TYPE                          | CONSTRAINTS   |
| :--------     | :---------------------------- | :------------ |
| user_id       | ???                           | ???           |
| graph_id      | ???                           | ???           |
| created_at    | ???                           | ???           |

**GRAPH**
| NAME          | TYPE                          | CONSTRAINTS   |
| :--------     | :---------------------------- | :------------ |
| graph_id      | ???                           | ???           |
| created_at    | ???                           | ???           |
| updated_at    | ???                           | ???           |
| name          | ???                           | UNIQUE        |
| description   | ???                           | ???           |
| is_public     | BOOLEAN                       | ???           |
| reddit        | JSON                          | ???           |
| nb_nodes      | ???                           | ???           |
| nb_edges      | ???                           | ???           |
| schema        | SCHEMA                        | ???           |
| accesses      | ACCESS[]                      | ???           |
| bookmarks     | BOOKMARK[]                    | ???           |
| cheers        | CHEER[]                       | ???           |

**SCHEMA**
> Think about removing this table
| NAME          | TYPE                          | CONSTRAINTS   |
| :--------     | :---------------------------- | :------------ |
| schema_id     | ???                           | ???           |
| nodes         | NODE[]                        | ???           |
| edges         | EDGE[]                        | ???           |
| graph_id      | ???                           | ???           |

**NODE**
| NAME          | TYPE                          | CONSTRAINTS   |
| :--------     | :---------------------------- | :------------ |
| node_id       | ???                           | ???           |
| name          | ???                           | ???           |
| color         | ???                           | ???           |
| properties    | PROPERTY[]                    | ???           |
| schema_id     | ???                           | ???           |

**EDGE**
| NAME          | TYPE                          | CONSTRAINTS   |
| :--------     | :---------------------------- | :------------ |
| edge_id       | ???                           | ???           |
| name          | ???                           | ???           |
| color         | ???                           | ???           |
| properties    | PROPERTY[]                    | ???           |
| schema_id     | ???                           | ???           |

**PROPERTY**
| NAME          | TYPE                          | CONSTRAINTS   |
| :--------     | :---------------------------- | :------------ |
| property_id   | ???                           | ???           |
| name          | ???                           | ???           |
| metadata      | JSON                          | ???           |
| node_id       | ???                           | NULL          |
| edge_id       | ???                           | NULL          |
> property's name should be unique in the node/edge schema

Here is the list of all the handled properties types:
```json
[
    "integer",
    "float",
    "string",
    "boolean",
    "date",
    "time",
    // "datetime",
    // "localdatetime",
    // "duration",
    // "2d_point",
    // "3d_point",
    "range",
    "select",
    "multi-select",
]
```

Here is the structure of `metadata`:
```json
{
    "property_type": "integer", // or any other property type
    "details": {
        "min": null, // default value, required for range
        "max": null, // default value, required for range
        "options": null, // default value required for select/multi-select
        "required": false, // default value
        "default_value": null, // default value, required if required is false
    }
}
```

## Types

**User**
```rs
struct User {}
```

**Access**
```rs
struct Access {}
```

**Bookmark**
```rs
struct Bookmark {}
```

**Cheer**
```rs
struct Cheer {}
```

**Graph**
```rs
struct Graph {}
```

**Schema**
```rs
struct Schema {}
```

**Node**
```rs
struct Node {}
```

**Edge**
```rs
struct Edge {}
```
