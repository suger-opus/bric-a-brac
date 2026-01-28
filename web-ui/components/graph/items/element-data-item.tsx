"use client";

import { Button } from "@/components/ui/button";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { Property, PropertyValue, Role } from "@/types";

type GraphElementItemProps = {
  kind: "node" | "edge";
  id: string;
  label: string;
  color: string;
  schemaProperties: Property[];
  dataProperties: Record<string, PropertyValue>;
  role: Role;
};

const GraphElementItem = ({
  kind,
  id,
  label,
  color,
  schemaProperties,
  dataProperties,
  role
}: GraphElementItemProps) => {
  return (
    <div className="w-80 max-w-full h-fit space-y-2 p-2 overflow-auto no-scrollbar bg-transparent backdrop-blur-xs border border-black/10 rounded-md">
      <div className="flex items-center space-x-2">
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
      {[Role.OWNER, Role.ADMIN, Role.EDITOR].includes(role) && (
        <Button size="sm" variant="secondary" className="w-full">
          Manage
        </Button>
      )}
    </div>
  );
};

export default GraphElementItem;
