"use client";

import DraftElementSchemaItem from "@/components/graph/items/draft-element-schema";
import { Button } from "@/components/ui/button";
import { Field, FieldError, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useGraph } from "@/contexts/graph-context";
import { useGraphSchemaForm } from "@/hooks/use-graph-schema-form";
import { ApiProvider } from "@/lib/api/provider";
import { CheckIcon, ChevronRightIcon } from "lucide-react";
import { useEffect, useEffectEvent, useMemo, useState } from "react";
import { useDropzone } from "react-dropzone";

const steps = ["1. Select File", "2. Generate Schema", "3. Insert Data"];

const checkFileSize = (file: File) => {
  const maxSizeInBytes = 100 * 1024; // 100KB
  if (file.size > maxSizeInBytes) {
    return {
      code: "file-too-large",
      message: `File size should not exceed ${maxSizeInBytes / 1024}KB`
    };
  }
  return null;
};

const GenerateSchemaContent = () => {
  const { graphService } = ApiProvider;
  const { metadata } = useGraph();
  const [currentStep, setCurrentStep] = useState(0);
  const [isGenerating, setIsGenerating] = useState(false);
  const { acceptedFiles, getRootProps, getInputProps, isFocused, isDragAccept, isDragReject } =
    useDropzone({
      maxFiles: 1,
      accept: {
        "text/plain": [".txt"],
        "text/csv": [".csv"]
      },
      validator: checkFileSize
    });
  const {
    nodesSchemas,
    edgesSchemas,
    loadGraphSchema,
    validateGraphSchema,
    graphSchemaErrors,
    submitGraphSchema
  } = useGraphSchemaForm();

  const dropzoneStyle = useMemo(() => ({
    flex: 1,
    display: "flex",
    flexDirection: "column" as const,
    alignItems: "center",
    padding: "20px",
    borderWidth: 2,
    borderRadius: 2,
    borderColor: "#eeeeee",
    borderStyle: "dashed",
    backgroundColor: "#fafafa",
    color: "#bdbdbd",
    outline: "none",
    transition: "border .24s ease-in-out",
    ...(isFocused
      ? {
        borderColor: "#2196f3"
      }
      : {}),
    ...(isDragAccept
      ? {
        borderColor: "#00e676"
      }
      : {}),
    ...(isDragReject
      ? {
        borderColor: "#ff1744"
      }
      : {})
  }), [
    isFocused,
    isDragAccept,
    isDragReject
  ]);

  const generateSchema = async () => {
    if (acceptedFiles.length === 1) {
      const file = acceptedFiles[0];
      const fileType = file.type;
      try {
        setIsGenerating(true);
        const res = await graphService.generateSchema(metadata!.graph_id, file, fileType);
        loadGraphSchema(res);
      } catch (error) {
        console.error("Error generating schema:", error);
      } finally {
        setIsGenerating(false);
      }
    }
  };

  const handlePreviousPage = () => {
    setCurrentStep((prev) => Math.max(0, prev - 1));
  };

  const handleNextPage = async () => {
    if (currentStep === 0) {
      if (acceptedFiles.length === 1) {
        setCurrentStep(1);
        await generateSchema();
      }
    } else if (currentStep === 1) {
      if (validateGraphSchema()) {
        setCurrentStep(2);
        await submitGraphSchema();
      }
    } else if (currentStep === 2) {
      if (true) {
        try {
        } catch (error) {
          console.error(error);
        }
      }
    }
  };

  const resetState = useEffectEvent(() => {
    setCurrentStep(0);
  });

  useEffect(() => {
    resetState();
  }, []);

  return (
    <div className="h-full flex flex-col justify-between">
      <Tabs value={steps[currentStep]} className="overflow-hidden">
        <TabsList className="w-full mb-2">
          <TabsTrigger value={steps[0]} disabled={currentStep !== 0}>
            {steps[0]} {currentStep > 0
              ? <CheckIcon className="ml-auto" />
              : <ChevronRightIcon className="ml-auto" />}
          </TabsTrigger>
          <TabsTrigger value={steps[1]} disabled={currentStep !== 1}>
            {steps[1]} {currentStep > 1
              ? <CheckIcon className="ml-auto" />
              : <ChevronRightIcon className="ml-auto" />}
          </TabsTrigger>
          <TabsTrigger value={steps[2]} disabled={currentStep !== 2}>
            {steps[2]} <ChevronRightIcon className="ml-auto" />
          </TabsTrigger>
        </TabsList>
        <div className="no-scrollbar px-1 overflow-y-auto">
          <TabsContent value={steps[0]}>
            <FieldGroup>
              <Field>
                <div {...getRootProps({ style: dropzoneStyle })}>
                  <input {...getInputProps()} />
                  <p>Drag & drop some files here, or click to select files</p>
                  <p>Only .txt and .csv files are accepted - Maximum file size: 100KB</p>
                </div>
                {acceptedFiles.map((file) => (
                  <li key={file.path}>{file.name} - {(file.size / 1024).toFixed(2)} KB</li>
                ))}
              </Field>
            </FieldGroup>
          </TabsContent>
          <TabsContent value={steps[1]}>
            <FieldGroup>
              {isGenerating && <p>Generating schema...</p>}
              {!isGenerating && (
                <Field>
                  <FieldLabel>Nodes</FieldLabel>
                  {nodesSchemas.map((node, index) => (
                    <div key={index} className="space-y-1">
                      <DraftElementSchemaItem
                        kind="node"
                        label={node.value.label}
                        color={node.value.color}
                        properties={node.value.properties}
                      />
                      <FieldError>{graphSchemaErrors[node.id]}</FieldError>
                    </div>
                  ))}
                  <FieldLabel>Edges</FieldLabel>
                  {edgesSchemas.map((edge, index) => (
                    <div key={index} className="space-y-1">
                      <DraftElementSchemaItem
                        kind="edge"
                        label={edge.value.label}
                        color={edge.value.color}
                        properties={edge.value.properties}
                      />
                      <FieldError>{graphSchemaErrors[edge.id]}</FieldError>
                    </div>
                  ))}
                </Field>
              )}
            </FieldGroup>
          </TabsContent>
          <TabsContent value={steps[2]}>
            <FieldGroup>
            </FieldGroup>
          </TabsContent>
        </div>
      </Tabs>
      <div className="mt-4 flex flex-col w-full items-end space-y-3">
        <div className="flex space-x-2">
          {currentStep === 0
            ? (
              <Button variant="outline">
                Cancel
              </Button>
            )
            : (
              <Button variant="outline" onClick={handlePreviousPage}>
                Back
              </Button>
            )}
          <Button onClick={handleNextPage}>
            {currentStep === steps.length - 1 ? "Submit" : "Next"}
          </Button>
        </div>
      </div>
    </div>
  );
};

export default GenerateSchemaContent;
