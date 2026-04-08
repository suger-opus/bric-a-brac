import type { EdgeSchema } from "@/types";
import ElementSchemaItem from "./element-schema-item";

const EdgeSchemaItem = ({ schema }: { schema: EdgeSchema; }) => (
  <ElementSchemaItem
    kind="edge"
    schemaKey={schema.key}
    label={schema.label}
    color={schema.color}
    description={schema.description}
  />
);

export default EdgeSchemaItem;
