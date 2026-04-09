CREATE TYPE role_type AS ENUM
(
    'owner',
    'admin',
    'editor',
    'viewer',
    'none'
);

CREATE TYPE session_status_type AS ENUM
(
    'active',
    'completed',
    'error'
);

CREATE TYPE session_message_role_type AS ENUM
(
    'system',
    'user',
    'assistant',
    'tool'
);

CREATE TABLE users
(
    user_id             UUID PRIMARY KEY                NOT NULL,
    email               VARCHAR                         NOT NULL UNIQUE,
    username            VARCHAR(50)                     NOT NULL UNIQUE,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT username_length                          CHECK (LENGTH(username) >= 3)
);

CREATE TABLE graphs
(
    graph_id            UUID PRIMARY KEY                NOT NULL,
    name                VARCHAR(100)                    NOT NULL,
    description         VARCHAR(1000)                   NOT NULL,
    is_public           BOOLEAN                         NOT NULL DEFAULT FALSE,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT graph_name_length                        CHECK (LENGTH(name) >= 3)
);

CREATE TABLE accesses
(
    user_id             UUID                            NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    graph_id            UUID                            NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    role                role_type                       NOT NULL DEFAULT 'none',
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
    description         VARCHAR(1000)                   NOT NULL,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT node_key_pattern                         CHECK (key ~ '^[a-zA-Z][a-zA-Z0-9]{7}$'),
    CONSTRAINT node_label_length                        CHECK (LENGTH(label) >= 3),
    CONSTRAINT node_color_pattern                       CHECK (color ~ '^#[0-9A-Fa-f]{6}$')
);

CREATE TABLE edges_schemas (
    edge_schema_id      UUID PRIMARY KEY                NOT NULL,
    graph_id            UUID                            NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    label               VARCHAR(25)                     NOT NULL,
    key                 VARCHAR(8)                      UNIQUE NOT NULL,
    color               VARCHAR(7)                      NOT NULL,
    description         VARCHAR(1000)                   NOT NULL,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT edge_key_pattern                         CHECK (key ~ '^[a-zA-Z][a-zA-Z0-9]{7}$'),
    CONSTRAINT edge_label_length                        CHECK (LENGTH(label) >= 3),
    CONSTRAINT edge_color_pattern                       CHECK (color ~ '^#[0-9A-Fa-f]{6}$')
);

CREATE TABLE sessions (
    session_id          UUID PRIMARY KEY                NOT NULL,
    graph_id            UUID                            NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    user_id             UUID                            NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    status              session_status_type             NOT NULL DEFAULT 'active',
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP
);

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
    document_id         UUID                            REFERENCES session_documents(document_id) ON DELETE SET NULL,
    position            INTEGER                         NOT NULL,
    role                session_message_role_type       NOT NULL,
    content             TEXT                            NOT NULL,
    tool_calls          TEXT,
    tool_call_id        VARCHAR,
    chunk_index         INTEGER,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_session_position                  UNIQUE (session_id, position)
);
