"use client";

import { useElementSchemaForm } from "@/hooks/use-element-schema-form";
import NewElementSchemaDialogContent from "./new-element-schema-dialog-content";

type NewEdgeSchemaDialogContentProps = {
  isOpen: boolean;
  onClose: () => void;
};

const NewEdgeSchemaDialogContent = ({ isOpen, onClose }: NewEdgeSchemaDialogContentProps) => {
  const { label, formattedLabel, color, properties, submitEdge } = useElementSchemaForm();

  return (
    <NewElementSchemaDialogContent
      kind="edge"
      isOpen={isOpen}
      onClose={onClose}
      onSubmit={submitEdge}
      label={label}
      formattedLabel={formattedLabel}
      color={color}
      properties={properties}
    />
  );
};

export default NewEdgeSchemaDialogContent;
