CREATE TYPE role_type AS ENUM
(
    'Owner',
    'Admin',
    'Editor',
    'Viewer',
    'None'
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
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP
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

CREATE TABLE nodes_schemas (
    node_schema_id      UUID PRIMARY KEY                NOT NULL,
    graph_id            UUID                            NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    label               VARCHAR(25)                     NOT NULL,
    key                 VARCHAR(8)                      UNIQUE NOT NULL,
    color               VARCHAR(7)                      NOT NULL,
    description         TEXT                            NOT NULL DEFAULT '',
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
    description         TEXT                            NOT NULL DEFAULT '',
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT edge_key_pattern                         CHECK (key ~ '^[a-zA-Z][a-zA-Z0-9]{7}$')
);

CREATE TABLE sessions (
    session_id          UUID PRIMARY KEY                NOT NULL,
    graph_id            UUID                            NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    user_id             UUID                            NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    status              VARCHAR(20)                     NOT NULL DEFAULT 'active',
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT session_status_check                     CHECK (status IN ('active', 'completed', 'error'))
);

CREATE INDEX idx_sessions_graph_id ON sessions(graph_id);

CREATE TABLE session_documents (
    document_id         UUID PRIMARY KEY                NOT NULL,
    session_id          UUID                            NOT NULL REFERENCES sessions(session_id) ON DELETE CASCADE,
    filename            TEXT                            NOT NULL,
    content_hash        VARCHAR(64)                     NOT NULL,
    content             TEXT                            NOT NULL,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE session_messages (
    message_id          UUID PRIMARY KEY                NOT NULL,
    session_id          UUID                            NOT NULL REFERENCES sessions(session_id) ON DELETE CASCADE,
    position            INTEGER                         NOT NULL,
    role                VARCHAR(20)                     NOT NULL,
    content             TEXT                            NOT NULL DEFAULT '',
    tool_calls          JSONB,
    tool_call_id        VARCHAR,
    document_id         UUID                            REFERENCES session_documents(document_id) ON DELETE SET NULL,
    chunk_index         INTEGER,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT message_role_check                       CHECK (role IN ('system', 'user', 'assistant', 'tool')),
    CONSTRAINT unique_session_position                  UNIQUE (session_id, position)
);
