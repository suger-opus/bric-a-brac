"use client";

import { useElementSchemaForm } from "@/hooks/use-element-schema-form";
import NewElementSchemaDialogContent from "./new-element-schema-content";

const NewEdgeSchemaContent = () => {
  const { label, color, properties, submitEdge } = useElementSchemaForm();

  return (
    <NewElementSchemaDialogContent
      kind="edge"
      onSubmit={submitEdge}
      label={label}
      color={color}
      properties={properties}
    />
  );
};

export default NewEdgeSchemaContent;
