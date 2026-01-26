"use client";

import NewNodeSchemaDialogContent from "@/components/dialog-contents/new-node-schema-dialog-content";
import NewElementSchemaButton from "./new-element-schema-button";

const NewNodeSchemaButton = () => {
  return (
    <NewElementSchemaButton kind="node">
      <NewNodeSchemaDialogContent isOpen={false} onClose={() => {}} />
    </NewElementSchemaButton>
  );
};

export default NewNodeSchemaButton;
