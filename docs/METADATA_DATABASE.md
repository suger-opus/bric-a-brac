# Metadata database

> PostgreSQL database

## Tables

**USER**
| NAME          | TYPE                          | CONSTRAINTS   |
| :------------ | :---------------------------- | :------------ |
| user_id       | ???                           | ???           |
| created_at    | ???                           | ???           |
| email         | ???                           | ???           |
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

**CHEER**
| NAME          | TYPE                          | CONSTRAINTS   |
| :--------     | :---------------------------- | :------------ |
| user_id       | ???                           | ???           |
| graph_id      | ???                           | ???           |

**GRAPH**
| NAME          | TYPE                          | CONSTRAINTS   |
| :--------     | :---------------------------- | :------------ |
| graph_id      | ???                           | ???           |
| created_at    | ???                           | ???           |
| updated_at    | ???                           | ???           |
| name          | ???                           | ???           |
| description   | ???                           | ???           |
| is_public     | BOOLEAN                       | ???           |
| reddit        | JSON                          | ???           |
| schema        | SCHEMA                        | ???           |
| accesses      | ACCESS[]                      | ???           |
| bookmarks     | BOOKMARK[]                    | ???           |
| cheers        | CHEER[]                       | ???           |

**SCHEMA**
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
| properties    | JSON                          | ???           |
| schema_id     | ???                           | ???           |

**EDGE**
| NAME          | TYPE                          | CONSTRAINTS   |
| :--------     | :---------------------------- | :------------ |
| edge_id       | ???                           | ???           |
| name          | ???                           | ???           |
| properties    | JSON                          | ???           |
| schema_id     | ???                           | ???           |

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
