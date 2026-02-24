"use client";

import { useGraph } from "@/contexts/graph-context";
import { useElementSchemaForm } from "@/hooks/use-element-schema-form";
import NewElementSchemaDialogContent from "./new-element-schema-content";

const NewNodeSchemaContent = () => {
  const { label, color, properties, submitNode } = useElementSchemaForm();
  const { setAction } = useGraph();

  const onSubmit = async () => {
    await submitNode();
    setAction(null);
  };

  return (
    <NewElementSchemaDialogContent
      kind="node"
      onSubmit={onSubmit}
      label={label}
      color={color}
      properties={properties}
    />
  );
};

export default NewNodeSchemaContent;
