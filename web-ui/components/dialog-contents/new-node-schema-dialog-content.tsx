"use client";

import { useElementSchemaForm } from "@/hooks/use-element-schema-form";
import NewElementSchemaDialogContent from "./new-element-schema-dialog-content";

type NewNodeSchemaDialogContentProps = {
  isOpen: boolean;
  onClose: () => void;
};

const NewNodeSchemaDialogContent = ({ isOpen, onClose }: NewNodeSchemaDialogContentProps) => {
  const { label, formattedLabel, color, properties, submitNode } = useElementSchemaForm();

  return (
    <NewElementSchemaDialogContent
      kind="node"
      isOpen={isOpen}
      onClose={onClose}
      onSubmit={submitNode}
      label={label}
      formattedLabel={formattedLabel}
      color={color}
      properties={properties}
    />
  );
};

export default NewNodeSchemaDialogContent;
