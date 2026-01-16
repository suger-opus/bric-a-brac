"use client";

import { Button } from "@/components/ui/button";
import {
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import { Field, FieldError } from "@/components/ui/field";
import { InputGroup, InputGroupAddon, InputGroupInput } from "@/components/ui/input-group";
import { Label } from "@/components/ui/label";
import { CheckIcon } from "lucide-react";
import { useState } from "react";

const NewNodeDialogContent = () => {
  const [name, setName] = useState("");

  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>Add a new node type</DialogTitle>
        <DialogDescription>
          This is the creation of a new node type.
        </DialogDescription>
      </DialogHeader>
      <div className="space-y-2">
        <div className="space-y-2">
          <InputGroup>
            <InputGroupInput
              id="new-node-name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="General"
            />
            <InputGroupAddon align="block-start">
              <Label htmlFor="new-node-name" className="text-foreground">Name</Label>
              <div className="ml-auto">
                <CheckIcon size={16} />
              </div>
            </InputGroupAddon>
          </InputGroup>
          <Field>
            {name.length > 0 && name.length < 5 && (
              <FieldError>Name must be at least 5 characters long.</FieldError>
            )}
          </Field>
        </div>
      </div>
      <DialogFooter>
        <Button type="submit" disabled>Create</Button>
      </DialogFooter>
    </DialogContent>
  );
};

export default NewNodeDialogContent;
