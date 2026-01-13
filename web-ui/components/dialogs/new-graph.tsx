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
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
  InputGroupText,
  InputGroupTextarea
} from "@/components/ui/input-group";
import { Label } from "@/components/ui/label";
import { CheckIcon } from "lucide-react";
import { useState } from "react";

const NewGraphContent = () => {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");

  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>Create a New Graph</DialogTitle>
        <DialogDescription>
          This is the creation of a new graph.
        </DialogDescription>
      </DialogHeader>
      <div className="space-y-2">
        <div className="space-y-2">
          <InputGroup>
            <InputGroupInput
              id="new-graph-name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="Napoleon battles"
            />
            <InputGroupAddon align="block-start">
              <Label htmlFor="new-graph-name" className="text-foreground">Name</Label>
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
        <div className="space-y-2">
          <InputGroup>
            <InputGroupTextarea
              id="new-graph-description"
              placeholder="This is about Napoleon's battles"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
            />
            <InputGroupAddon align="block-start">
              <Label htmlFor="new-graph-description" className="text-foreground">Description</Label>
              <div className="ml-auto">
                <CheckIcon size={16} />
              </div>
            </InputGroupAddon>
            <InputGroupAddon align="block-end">
              <InputGroupText className="text-muted-foreground text-xs">
                120 characters left
              </InputGroupText>
            </InputGroupAddon>
          </InputGroup>
          <Field>
            {description.length > 0 && description.length < 5 && (
              <FieldError>Description must be at least 5 characters long.</FieldError>
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

export default NewGraphContent;
