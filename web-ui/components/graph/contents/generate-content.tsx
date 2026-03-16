"use client";

import DraftElementDataItem from "@/components/graph/items/draft-element-data";
import DraftElementSchemaItem from "@/components/graph/items/draft-element-schema";
import { Button } from "@/components/ui/button";
import { Field, FieldError, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useGraph } from "@/contexts/graph-context";
import { useGraphSchemaForm } from "@/hooks/use-graph-schema-form";
import { CheckIcon, ChevronRightIcon } from "lucide-react";
import { useEffect, useEffectEvent, useMemo, useState } from "react";
import { useDropzone } from "react-dropzone";

const steps = ["1. Select File", "2. Generate Schema", "3. Generate Data"];

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

type GenerateContentProps = {
  onClose: () => void;
};

const GenerateContent = ({ onClose }: GenerateContentProps) => {
  const { metadata } = useGraph();
  const {
    graphSchema,
    savedGraphSchema,
    graphData,
    validateGraphSchema,
    validateGraphData,
    graphSchemaErrors,
    graphDataErrors,
    generateGraphSchema,
    generateGraphData,
    submitGraphSchema,
    submitGraphData
  } = useGraphSchemaForm(metadata!.graph_id);
  const [currentStep, setCurrentStep] = useState(0);
  const [isLoading, setIsLoading] = useState(false);
  const { acceptedFiles, getRootProps, getInputProps, isFocused, isDragAccept, isDragReject } =
    useDropzone({
      maxFiles: 1,
      accept: {
        "text/plain": [".txt"],
        "text/csv": [".csv"]
      },
      validator: checkFileSize
    });

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
        setIsLoading(true);
        await generateGraphSchema(file, fileType);
        return true;
      } catch (error) {
        console.error("Error generating schema:", error);
      } finally {
        setIsLoading(false);
      }
    }
    return false;
  };

  const submitSchema = async () => {
    try {
      setIsLoading(true);
      await submitGraphSchema();
      return true;
    } catch (error) {
      console.error("Error submitting schema:", error);
    } finally {
      setIsLoading(false);
    }
    return false;
  };

  const generateData = async () => {
    if (acceptedFiles.length === 1) {
      const file = acceptedFiles[0];
      const fileType = file.type;
      try {
        setIsLoading(true);
        await generateGraphData(file, fileType);
        return true;
      } catch (error) {
        console.error("Error generating data:", error);
      } finally {
        setIsLoading(false);
      }
    }
    return false;
  };

  const submitData = async () => {
    try {
      setIsLoading(true);
      await submitGraphData();
      return true;
    } catch (error) {
      console.error("Error submitting data:", error);
    } finally {
      setIsLoading(false);
    }
    return false;
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
      if (validateGraphSchema() && await submitSchema()) {
        setCurrentStep(2);
        await generateData();
      }
    } else if (currentStep === 2) {
      if (validateGraphData() && await submitData()) {
        onClose();
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
              {isLoading ? <p>Generating schema...</p> : (
                <Field>
                  <FieldLabel>Nodes</FieldLabel>
                  {graphSchema.nodes.map((node, index) => (
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
                  {graphSchema.edges.map((edge, index) => (
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
                  <Button variant="secondary" onClick={generateSchema}>
                    Re-generate Schema
                  </Button>
                </Field>
              )}
            </FieldGroup>
          </TabsContent>
          <TabsContent value={steps[2]}>
            <FieldGroup>
              {isLoading ? <p>Generating data...</p> : (
                <Field>
                  <FieldLabel>Nodes</FieldLabel>
                  {graphData.nodes.map((node, index) => {
                    const savedNode = savedGraphSchema.nodes.find((n) => n.key === node.value.key);
                    return (savedNode
                      ? (
                        <div key={index} className="space-y-1">
                          <DraftElementDataItem
                            kind="node"
                            label={savedNode.label}
                            color={savedNode.color}
                            propertiesSchemas={savedNode.properties}
                            propertiesData={node.value.properties}
                          />
                          <FieldError>{graphDataErrors[node.id]}</FieldError>
                        </div>
                      )
                      : <></>);
                  })}
                  <FieldLabel>Edges</FieldLabel>
                  {graphData.edges.map((edge, index) => {
                    const savedEdge = savedGraphSchema.edges.find((e) => e.key === edge.value.key);
                    return (savedEdge
                      ? (
                        <div key={index} className="space-y-1">
                          <DraftElementDataItem
                            kind="edge"
                            label={savedEdge.label}
                            color={savedEdge.color}
                            propertiesSchemas={savedEdge.properties}
                            propertiesData={edge.value.properties}
                          />
                          <FieldError>{graphDataErrors[edge.id]}</FieldError>
                        </div>
                      )
                      : <></>);
                  })}
                  <Button variant="outline" onClick={generateData}>
                    Re-generate Data
                  </Button>
                </Field>
              )}
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
            {currentStep === 0
              ? "Generate Schema"
              : currentStep === 1
              ? "Save Schema & Generate Data"
              : "Save Data & Close"}
          </Button>
        </div>
      </div>
    </div>
  );
};

export default GenerateContent;
