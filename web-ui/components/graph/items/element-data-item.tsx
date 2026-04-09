import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import type { PropertiesData } from "@/types";

type GraphElementItemProps = {
  kind: "node" | "edge";
  id: string;
  label: string;
  color: string;
  properties: PropertiesData;
};

const GraphElementItem = ({ kind, id, label, color, properties }: GraphElementItemProps) => {
  return (
    <div className="w-80 max-w-full h-fit space-y-2 p-2 overflow-auto no-scrollbar bg-background/80 backdrop-blur-sm border border-border rounded-md">
      <div className="flex items-center space-x-2">
        <div
          className={kind === "node" ? "w-4 h-4 rounded-full" : "w-4 h-2 rounded-xs"}
          style={{ backgroundColor: color }}
        />
        <h2 className="scroll-m-20 text-xl font-semibold tracking-tight">{label}</h2>
      </div>
      <Table>
        <TableBody>
          <TableRow>
            <TableCell className="font-medium">ID</TableCell>
            <TableCell className="font-mono text-xs">{id}</TableCell>
          </TableRow>
          {Object.entries(properties).map(([key, value]) => (
            <TableRow key={key}>
              <TableCell className="font-medium">{key}</TableCell>
              <TableCell>{String(value)}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
};

export default GraphElementItem;
