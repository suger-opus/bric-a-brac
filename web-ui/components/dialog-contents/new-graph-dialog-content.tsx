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
import { Spinner } from "@/components/ui/spinner";
import { ApiProvider } from "@/lib/api/provider";
import { requestGraph } from "@/lib/api/schemas/request-schemas";
import { CheckIcon, XIcon } from "lucide-react";
import { useRouter } from "next/navigation";
import { useEffect, useEffectEvent, useState } from "react";
import * as v from "valibot";

type NewGraphDialogContentProps = {
  isOpen: boolean;
  onClose: () => void;
};

const NewGraphDialogContent = ({ isOpen, onClose }: NewGraphDialogContentProps) => {
  const { graphService } = ApiProvider;
  const router = useRouter();

  const [name, setName] = useState("");
  const [validationNameError, setValidationNameError] = useState<string | null>(null);
  const [description, setDescription] = useState("");
  const [validationDescriptionError, setValidationDescriptionError] = useState<string | null>(null);
  const [isCreateLoading, setIsCreateLoading] = useState(false);

  const resetState = useEffectEvent(() => {
    setName("");
    setDescription("");
    setValidationNameError(null);
    setValidationDescriptionError(null);
  });

  useEffect(() => {
    if (isOpen) {
      resetState();
    }
  }, [isOpen]);

  const createGraph = async () => {
    try {
      setIsCreateLoading(true);
      setValidationNameError(null);
      setValidationDescriptionError(null);
      const validation = v.safeParse(requestGraph, { name, description });
      if (validation.success) {
        const newGraph = await graphService.post(validation.output);
        onClose();
        router.push(`/graph?graph_id=${newGraph.graph_id}`);
      } else {
        setValidationNameError(
          validation.issues.find((issue) => issue.path?.some((p) => p.key === "name"))?.message
            || null
        );
        setValidationDescriptionError(
          validation.issues.find((issue) => issue.path?.some((p) => p.key === "description"))
            ?.message
            || null
        );
      }
    } catch (error) {
      console.error("Error during createGraph:", error);
    } finally {
      setIsCreateLoading(false);
    }
  };

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
                {validationNameError
                  ? <XIcon size={16} />
                  : isCreateLoading || validationDescriptionError
                  ? <CheckIcon size={16} />
                  : <></>}
              </div>
            </InputGroupAddon>
          </InputGroup>
          <Field>
            {validationNameError
              && <FieldError>{validationNameError}</FieldError>}
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
                {validationDescriptionError
                  ? <XIcon size={16} />
                  : isCreateLoading || validationNameError
                  ? <CheckIcon size={16} />
                  : <></>}
              </div>
            </InputGroupAddon>
            <InputGroupAddon align="block-end">
              <InputGroupText className="text-muted-foreground text-xs">
                {description.length} characters
              </InputGroupText>
            </InputGroupAddon>
          </InputGroup>
          <Field>
            {validationDescriptionError
              && <FieldError>{validationDescriptionError}</FieldError>}
          </Field>
        </div>
      </div>
      <DialogFooter>
        <Button
          type="submit"
          onClick={createGraph}
          disabled={isCreateLoading}
        >
          {isCreateLoading && <Spinner />}
          Create
        </Button>
      </DialogFooter>
    </DialogContent>
  );
};

export default NewGraphDialogContent;
