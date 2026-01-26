"use client";

import NewEdgeSchemaDialogContent from "@/components/dialog-contents/new-edge-schema-dialog-content";
import NewElementSchemaButton from "./new-element-schema-button";

const NewNodeSchemaButton = () => {
  return (
    <NewElementSchemaButton kind="edge">
      <NewEdgeSchemaDialogContent isOpen={false} onClose={() => {}} />
    </NewElementSchemaButton>
  );
};

export default NewNodeSchemaButton;
