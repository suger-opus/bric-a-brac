DROP TABLE IF EXISTS properties;
DROP TABLE IF EXISTS edge_schemas;
DROP TABLE IF EXISTS node_schemas;
DROP TABLE IF EXISTS cheers;
DROP TABLE IF EXISTS bookmarks;
DROP TABLE IF EXISTS accesses;
DROP TABLE IF EXISTS graphs;
DROP TABLE IF EXISTS users;

DROP TRIGGER IF EXISTS check_node_schema_formatted_label_uniqueness ON node_schemas;
DROP TRIGGER IF EXISTS check_edge_schema_formatted_label_uniqueness ON edge_schemas;

DROP FUNCTION IF EXISTS check_schema_formatted_label_uniqueness();

DROP TYPE IF EXISTS property_type;
DROP TYPE IF EXISTS role_type;
