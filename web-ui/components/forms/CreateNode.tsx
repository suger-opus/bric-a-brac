"use client";

import { createNode } from "@/lib/api";
import { useState } from "react";

type NodeFormProps = {
  graphId: string;
  onNodeCreated: () => void;
};

type PropertyValue = {
  key: string;
  value: string;
  type: "String" | "Integer" | "Float" | "Boolean";
};

const NodeForm = ({ graphId, onNodeCreated }: NodeFormProps) => {
  const [label, setLabel] = useState("");
  const [properties, setProperties] = useState<PropertyValue[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState("");

  const addProperty = () => {
    setProperties([...properties, { key: "", value: "", type: "String" }]);
  };

  const removeProperty = (index: number) => {
    setProperties(properties.filter((_, i) => i !== index));
  };

  const updateProperty = (
    index: number,
    field: keyof PropertyValue,
    value: string
  ) => {
    const updated = [...properties];
    updated[index] = { ...updated[index], [field]: value };
    setProperties(updated);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setIsLoading(true);

    try {
      const parsedProperties: Record<string, string | number | boolean> = {};

      for (const prop of properties) {
        if (prop.key.trim()) {
          if (prop.type === "Integer") {
            parsedProperties[prop.key] = parseInt(prop.value, 10);
          } else if (prop.type === "Float") {
            parsedProperties[prop.key] = parseFloat(prop.value);
          } else if (prop.type === "Boolean") {
            parsedProperties[prop.key] = prop.value === "true";
          } else {
            parsedProperties[prop.key] = prop.value;
          }
        }
      }

      await createNode({
        graph_id: graphId,
        label,
        properties: parsedProperties,
      });

      setLabel("");
      setProperties([]);
      onNodeCreated();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to create node");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-zinc-300 mb-1">
          Node Label
        </label>
        <input
          type="text"
          value={label}
          onChange={(e) => setLabel(e.target.value)}
          placeholder="Person, Company, etc."
          className="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-2 text-zinc-100 placeholder-zinc-500 focus:border-zinc-500 focus:outline-none"
          required
        />
      </div>

      <div>
        <div className="flex items-center justify-between mb-2">
          <label className="block text-sm font-medium text-zinc-300">
            Properties
          </label>
          <button
            type="button"
            onClick={addProperty}
            className="text-xs text-blue-400 hover:text-blue-300"
          >
            + Add Property
          </button>
        </div>

        {properties.length === 0 ? (
          <p className="text-xs text-zinc-500 italic">No properties added</p>
        ) : (
          <div className="space-y-2">
            {properties.map((prop, index) => (
              <div key={index} className="flex gap-2 items-start">
                <input
                  type="text"
                  value={prop.key}
                  onChange={(e) => updateProperty(index, "key", e.target.value)}
                  placeholder="Key"
                  className="flex-1 rounded-md border border-zinc-700 bg-zinc-900 px-2 py-1.5 text-sm text-zinc-100 placeholder-zinc-500 focus:border-zinc-500 focus:outline-none"
                />
                <select
                  value={prop.type}
                  onChange={(e) =>
                    updateProperty(index, "type", e.target.value)
                  }
                  className="rounded-md border border-zinc-700 bg-zinc-900 px-2 py-1.5 text-sm text-zinc-100 focus:border-zinc-500 focus:outline-none"
                >
                  <option value="String">Text</option>
                  <option value="Integer">Number</option>
                  <option value="Float">Decimal</option>
                  <option value="Boolean">True/False</option>
                </select>
                <input
                  type={
                    prop.type === "Boolean"
                      ? "text"
                      : prop.type === "Integer" || prop.type === "Float"
                      ? "number"
                      : "text"
                  }
                  value={prop.value}
                  onChange={(e) =>
                    updateProperty(index, "value", e.target.value)
                  }
                  placeholder={
                    prop.type === "Boolean" ? "true or false" : "Value"
                  }
                  step={prop.type === "Float" ? "any" : undefined}
                  className="flex-1 rounded-md border border-zinc-700 bg-zinc-900 px-2 py-1.5 text-sm text-zinc-100 placeholder-zinc-500 focus:border-zinc-500 focus:outline-none"
                />
                <button
                  type="button"
                  onClick={() => removeProperty(index)}
                  className="text-red-400 hover:text-red-300 px-2 py-1.5"
                >
                  ×
                </button>
              </div>
            ))}
          </div>
        )}
      </div>

      {error && (
        <div className="rounded-md bg-red-900/20 border border-red-900 px-3 py-2 text-sm text-red-400">
          {error}
        </div>
      )}

      <button
        type="submit"
        disabled={isLoading}
        className="w-full rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {isLoading ? "Creating..." : "Create Node"}
      </button>
    </form>
  );
};

export default NodeForm;
