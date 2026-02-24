CREATE TYPE role_type AS ENUM
(
    'Owner',
    'Admin',
    'Editor',
    'Viewer',
    'None'
);

CREATE TYPE property_type AS ENUM
(
    'Number',
    'String',
    'Boolean',
    'Select'
);

CREATE TABLE users
(
    user_id             UUID PRIMARY KEY                NOT NULL,
    email               VARCHAR                         NOT NULL UNIQUE,
    username            VARCHAR(50)                     NOT NULL UNIQUE,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE graphs
(
    graph_id            UUID PRIMARY KEY                NOT NULL,
    name                VARCHAR(100)                    NOT NULL,
    description         VARCHAR(10000)                  NOT NULL,
    is_public           BOOLEAN                         NOT NULL DEFAULT FALSE,
    reddit              JSONB                           NOT NULL DEFAULT '{}'::JSONB,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    nb_data_nodes       INTEGER                         NOT NULL DEFAULT 0,
    nb_data_edges       INTEGER                         NOT NULL DEFAULT 0
);

CREATE TABLE accesses
(
    user_id             UUID                            NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    graph_id            UUID                            NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    role                role_type                       NOT NULL DEFAULT 'None',
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, graph_id)
);

CREATE TABLE bookmarks
(
    user_id             UUID                            NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    graph_id            UUID                            NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, graph_id)
);

CREATE TABLE cheers
(
    user_id             UUID                            NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    graph_id            UUID                            NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, graph_id)
);

CREATE TABLE nodes_schemas (
    node_schema_id      UUID PRIMARY KEY                NOT NULL,
    graph_id            UUID                            NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    label               VARCHAR(25)                     NOT NULL,
    key                 VARCHAR(8)                      UNIQUE NOT NULL,
    color               VARCHAR(7)                      NOT NULL,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT node_key_pattern                         CHECK (key ~ '^[a-zA-Z][a-zA-Z0-9]{7}$')
);

CREATE TABLE edges_schemas (
    edge_schema_id      UUID PRIMARY KEY                NOT NULL,
    graph_id            UUID                            NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    label               VARCHAR(25)                     NOT NULL,
    key                 VARCHAR(8)                      UNIQUE NOT NULL,
    color               VARCHAR(7)                      NOT NULL,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT edge_key_pattern                         CHECK (key ~ '^[a-zA-Z][a-zA-Z0-9]{7}$')
);

CREATE TABLE properties_schemas (
    property_schema_id  UUID PRIMARY KEY                NOT NULL,
    node_schema_id      UUID                            REFERENCES nodes_schemas(node_schema_id) ON DELETE CASCADE,
    edge_schema_id      UUID                            REFERENCES edges_schemas(edge_schema_id) ON DELETE CASCADE,
    label               VARCHAR(25)                     NOT NULL,
    key                 VARCHAR(8)                      UNIQUE NOT NULL,
    property_type       property_type                   NOT NULL,
    metadata            JSONB                           NOT NULL DEFAULT '{}'::JSONB,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT property_belongs_to_one CHECK (
        (
            node_schema_id IS NOT NULL
            AND edge_schema_id IS NULL
        )
        OR
        (
            node_schema_id IS NULL
            AND edge_schema_id IS NOT NULL
        )
    ),
    CONSTRAINT property_key_pattern                     CHECK (key ~ '^[a-zA-Z][a-zA-Z0-9]{7}$')
);
