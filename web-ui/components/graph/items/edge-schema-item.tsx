import type { EdgeSchema } from "@/types";
import ElementSchemaItem from "./element-schema-item";

const EdgeSchemaItem = ({ schema }: { schema: EdgeSchema }) => (
  <ElementSchemaItem kind="edge" label={schema.label} color={schema.color} description={schema.description} />
);

export default EdgeSchemaItem;
