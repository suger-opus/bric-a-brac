"use client";

import { Dialog, DialogTrigger } from "@/components/ui/dialog";
import { Item, ItemActions, ItemContent, ItemTitle } from "@/components/ui/item";
import { PlusIcon } from "lucide-react";
import React, { useState } from "react";

type NewElementSchemaButtonProps = {
  kind: "node" | "edge";
  children: React.ReactElement<{ isOpen: boolean; onClose: () => void; }>;
};

const NewElementSchemaButton = ({ kind, children }: NewElementSchemaButtonProps) => {
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  return (
    <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
      <DialogTrigger asChild>
        <Item
          variant="outline"
          className="hover:bg-gray-200 cursor-pointer py-2 pl-3 bg-gray-100"
        >
          <ItemContent>
            <ItemTitle>New {kind[0].toUpperCase() + kind.slice(1)} Type</ItemTitle>
          </ItemContent>
          <ItemActions>
            <PlusIcon size={14} />
          </ItemActions>
        </Item>
      </DialogTrigger>
      {React.cloneElement(children, {
        isOpen: isDialogOpen,
        onClose: () => setIsDialogOpen(false)
      })}
    </Dialog>
  );
};

export default NewElementSchemaButton;
