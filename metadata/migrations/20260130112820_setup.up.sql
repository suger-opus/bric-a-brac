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
    username            VARCHAR(100)                    NOT NULL UNIQUE,
    email               VARCHAR(100)                    NOT NULL UNIQUE,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE graphs
(
    graph_id            UUID PRIMARY KEY                NOT NULL,
    user_id             UUID                            NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    name                VARCHAR(100)                    NOT NULL,
    description         TEXT                            NOT NULL,
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
    ROLE                role_type                       NOT NULL DEFAULT 'None',
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

CREATE TABLE node_schemas (
    node_schema_id      UUID PRIMARY KEY                NOT NULL,
    graph_id            UUID                            NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    label               VARCHAR(100)                    NOT NULL,
    formatted_label     VARCHAR(100)                    NOT NULL,
    color               VARCHAR(7)                      NOT NULL,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT node_formatted_label_pattern             CHECK (formatted_label ~ '^([A-Z][a-z]*_)*[A-Z][a-z]*$'),
    CONSTRAINT unique_node_formatted_label              UNIQUE (graph_id, formatted_label)
);

CREATE TABLE edge_schemas (
    edge_schema_id      UUID PRIMARY KEY                NOT NULL,
    graph_id            UUID                            NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    label               VARCHAR(100)                    NOT NULL,
    formatted_label     VARCHAR(100)                    NOT NULL,
    color               VARCHAR(7)                      NOT NULL,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT edge_formatted_label_pattern             CHECK (formatted_label ~ '^([A-Z][a-z]*_)*[A-Z][a-z]*$'),
    CONSTRAINT unique_edge_formatted_label              UNIQUE (graph_id, formatted_label)
);

CREATE OR REPLACE FUNCTION check_schema_formatted_label_uniqueness()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM node_schemas
        WHERE graph_id = NEW.graph_id
        AND (formatted_label = NEW.formatted_label)
        AND node_schema_id != COALESCE(NEW.node_schema_id, '00000000-0000-0000-0000-000000000000'::UUID)
    ) OR EXISTS (
        SELECT 1 FROM edge_schemas
        WHERE graph_id = NEW.graph_id
        AND (formatted_label = NEW.formatted_label)
        AND edge_schema_id != COALESCE(NEW.edge_schema_id, '00000000-0000-0000-0000-000000000000'::UUID)
    ) THEN
        RAISE EXCEPTION 'Label "%" or formatted label "%" already exists in this graph', NEW.label, NEW.formatted_label;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_node_schema_formatted_label_uniqueness
    BEFORE INSERT OR UPDATE ON node_schemas
    FOR EACH ROW EXECUTE FUNCTION check_schema_formatted_label_uniqueness();

CREATE TRIGGER check_edge_schema_formatted_label_uniqueness
    BEFORE INSERT OR UPDATE ON edge_schemas
    FOR EACH ROW EXECUTE FUNCTION check_schema_formatted_label_uniqueness();

CREATE TABLE properties (
    property_id         UUID PRIMARY KEY                NOT NULL,
    node_schema_id      UUID                            REFERENCES node_schemas(node_schema_id) ON DELETE CASCADE,
    edge_schema_id      UUID                            REFERENCES edge_schemas(edge_schema_id) ON DELETE CASCADE,
    label               VARCHAR(100)                    NOT NULL,
    formatted_label     VARCHAR(100)                    NOT NULL,
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
    CONSTRAINT property_formatted_label_pattern         CHECK (formatted_label ~ '^([A-Z][a-z]*_)*[A-Z][a-z]*$'),
    CONSTRAINT unique_node_property_formatted_label     UNIQUE (node_schema_id, formatted_label),
    CONSTRAINT unique_edge_property_formatted_label     UNIQUE (edge_schema_id, formatted_label)
);
