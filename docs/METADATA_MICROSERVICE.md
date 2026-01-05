# Metadata micro-service

> Rest API

## Middlewares

### Auth (AUT)

> Retrieves auth credentials

Request changes:
- ??? (explain cookie / token thing)
- refresh token when token expired

Exit conditions: None (never exits, always forwards)

### Access (ACC)

> Checks user access to targeted ressource
> Always ran after `Auth` middleware 

Request changes: None

Exit conditions:
- User isn't authenticated (**401** response status) 
- Graph doesn't exist (**404** response status)
    - graph `/graphs/:graph_id` doesn't exist
    - (existance of node `:node_id` or edge `:edge_id` are not checked here)
- User isn't allowed (**403** response status):
    - endpoint is `/graphs/:graph_id` or `/graphs/:graph_id/schema` or `/graphs/:graph_id/data`
        - (GET) the graph `:graph_id` is private and the user requesting has no access to it
    - endpoint is `/graphs/:graph_id`
        - (PATCH, DELETE) the user requesting has no admin access to the graph `:graph_id`
    - endpoint is `/graphs/:graph_id/schema/*` or `/graphs/:graph_id/data/*`
        - (POST, PATCH, DELETE) the user requesting has no admin/writer access to the graph `:graph_id`
    - endpoint is `/graphs/:graph_id/accesses` or `/graphs/:graph_id/accesses/*`
        - (GET, POST, PATCH, DELETE) the user requesting has no admin access to the graph `:graph_id`

### Validation (VAL)

> Checks body from schema when inserting/updating data
> Always ran after `Auth` and `Access` middlewares

Request changes: None

Exit conditions:
- Data doesn't follow schema (**422** response status)
    - endpoint `/graphs/:graph_id/data/nodes` or `/graphs/:graph_id/data/nodes/:node_id`
        - (POST, PATCH) the data doesn't follow the schema of `schema_node_id` (in body)
    - endpoint `/graphs/:graph_id/data/edges` or `/graphs/:graph_id/data/edges/:edge_id`
        - (POST, PATCH) the data doesn't follow the schema of `schema_edge_id` (in body)

## Endpoints

> All endpoints can return a **500** response status

| Method    | Middlewares   | URL                                                                       | Request body      | Response status   | Response body     |
| :-------- | :------------ | :------------------------------------------------------------------------ | :---------------- | :---------------- | :---------------- |
| POST      |               | /auth/register?service={gmail,reddit}                                     | ???               | ???               | ???               |
| POST      |               | /auth/register?service={gmail,reddit}                                     | ???               | ???               | ???               |
| POST      |               | /auth/login?service={gmail,reddit}                                        | ???               | ???               | ???               |
| DELETE    |               | /auth/logout                                                              | None              | 204               | None              |
| GET       |               | /graphs/search?keyword=""                                                 | None              | 200               | `Graph[]`         |
| GET       | ACC           | /graphs/:graph_id                                                         | None              | 200               | `Graph`           |
| GET       | ACC           | /graphs/:graph_id/schema                                                  | None              | 200               | `Schema`          |
| GET       | ACC           | /graphs/:graph_id/data                                                    | None              | 200               | `Data`            |
| GET       | AUT,ACC       | /users/me                                                                 | None              | 200               | `User`            |
| DELETE    | AUT,ACC       | /users/me                                                                 | None              | 204               | None              |
| POST      | AUT,ACC       | /graphs                                                                   | `PostGraph`       | 201,400           | `Graph`           |
| GET       | AUT,ACC       | /graphs/filter?role=[reader,writer,admin]&bookmarked={t,f}&cheered={t,f}  | None              | 200               | `Graph[]`         |
| PATCH     | AUT,ACC       | /graphs/:graph_id                                                         | `PatchGraph`      | 200,400           | `Graph`           |
| DELETE    | AUT,ACC       | /graphs/:graph_id                                                         | None              | 204               | None              |
| POST      | AUT,ACC       | /graphs/:graph_id/schema/nodes                                            | `PostSchemaNode`  | 201,400           | `SchemaNode`      |
| PATCH     | AUT,ACC       | /graphs/:graph_id/schema/nodes/:node_id                                   | `PatchSchemaNode` | 200,400,404       | `SchemaNode`      |
| DELETE    | AUT,ACC       | /graphs/:graph_id/schema/nodes/:node_id                                   | None              | 204,404           | None              |
| POST      | AUT,ACC       | /graphs/:graph_id/schema/edges                                            | `PostSchemaEdge`  | 201,400           | `SchemaEdge`      |
| PATCH     | AUT,ACC       | /graphs/:graph_id/schema/edges/:edge_id                                   | `PatchSchemaEdge` | 200,400,404       | `SchemaEdge`      |
| DELETE    | AUT,ACC       | /graphs/:graph_id/schema/edges/:edge_id                                   | None              | 204,404           | None              |
| POST      | AUT,ACC,VAL   | /graphs/:graph_id/data/nodes                                              | `PostDataNode`    | 201,400           | `DataNode`        |
| PATCH     | AUT,ACC,VAL   | /graphs/:graph_id/data/nodes/:node_id                                     | `PatchDataNode`   | 200,400,404       | `DataNode`        |
| DELETE    | AUT,ACC       | /graphs/:graph_id/data/nodes/:node_id                                     | None              | 204,404           | None              |
| POST      | AUT,ACC,VAL   | /graphs/:graph_id/data/edges                                              | `PostDataEdge`    | 201,400           | `DataEdge`        |
| PATCH     | AUT,ACC,VAL   | /graphs/:graph_id/data/edges/:edge_id                                     | `PatchDataEdge`   | 200,400,404       | `DataEdge`        |
| DELETE    | AUT,ACC       | /graphs/:graph_id/data/edges/:edge_id                                     | None              | 204,404           | None              |
| GET       | AUT,ACC       | /graphs/:graph_id/accesses                                                | None              | 200               | `Access[]`        |
| POST      | AUT,ACC       | /graphs/:graph_id/accesses                                                | `PostAccess`      | 201,400           | `Access`          |
| PATCH     | AUT,ACC       | /graphs/:graph_id/accesses/:access_id                                     | `PatchAccess`     | 200,400,404       | `Access`          |
| DELETE    | AUT,ACC       | /graphs/:graph_id/accesses/:access_id                                     | None              | 204,404           | None              |
| POST      | AUT,ACC       | /graphs/:graph_id/bookmarks                                               | None              | 201               | None              |
| DELETE    | AUT,ACC       | /graphs/:graph_id/bookmarks                                               | None              | 204               | None              |
| POST      | AUT,ACC       | /graphs/:graph_id/cheers                                                  | None              | 201               | None              |
| DELETE    | AUT,ACC       | /graphs/:graph_id/cheers                                                  | None              | 204               | None              |

## Types

**User**:
```json
{
}
```

**Graph**:
```json
{
}
```

**PostGraph**:
```json
{
}
```

**PatchGraph**:
```json
{
}
```

**Schema**:
```json
{
}
```

**SchemaNode**:
```json
{
}
```

**PostSchemaNode**:
```json
{
}
```

**PatchSchemaNode**:
```json
{
}
```

**SchemaEdge**:
```json
{
}
```

**PostSchemaEdge**:
```json
{
}
```

**PatchSchemaEdge**:
```json
{
}
```

**Data**:
```json
{
}
```

**DataNode**:
```json
{
}
```

**PostDataNode**:
```json
{
}
```

**PatchDataNode**:
```json
{
}
```

**DataEdge**:
```json
{
}
```

**PostDataEdge**:
```json
{
}
```

**PatchDataEdge**:
```json
{
}
```

**Access**:
```json
{
}
```

**PostAccess**:
```json
{
}
```

**PatchAccess**:
```json
{
}
```
