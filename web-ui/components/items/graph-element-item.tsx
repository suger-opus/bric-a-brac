"use client";

import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { Property, PropertyValue } from "@/types";

type GraphElementItemProps = {
  kind: "node" | "edge";
  id: string;
  label: string;
  color: string;
  schemaProperties: Property[];
  dataProperties: Record<string, PropertyValue>;
};

const GraphElementItem = ({
  kind,
  id,
  label,
  color,
  schemaProperties,
  dataProperties
}: GraphElementItemProps) => {
  return (
    <div className="absolute top-2 left-2 w-80 max-w-full h-fit p-2 overflow-auto no-scrollbar bg-transparent backdrop-blur-xs border border-black/10 rounded-md">
      <div className="flex items-center space-x-2 mb-4">
        <div
          className={kind === "node" ? "w-4 h-4 rounded-full" : "w-4 h-2 rounded-xs"}
          style={{ "backgroundColor": color }}
        />
        <h2 className="scroll-m-20 text-xl font-semibold tracking-tight">
          {label}
        </h2>
      </div>
      <Table>
        <TableBody>
          <TableRow>
            <TableCell className="font-medium">ID</TableCell>
            <TableCell>{id}</TableCell>
          </TableRow>
          {Object.entries(dataProperties).map(([key, value]) => (
            <TableRow key={key}>
              <TableCell className="font-medium">
                {schemaProperties.find((p) => p.formatted_label === key)?.label || key}
              </TableCell>
              <TableCell>
                {String(value)}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
};

export default GraphElementItem;
