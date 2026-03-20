import type { NodeSchema } from "@/types";
import ElementSchemaItem from "./element-schema-item";

const NodeSchemaItem = ({ schema }: { schema: NodeSchema }) => (
  <ElementSchemaItem kind="node" label={schema.label} color={schema.color} description={schema.description} />
);

export default NodeSchemaItem;
