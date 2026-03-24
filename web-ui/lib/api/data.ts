import type { ProcessedGraphData } from "@/types";

export const sampleProcessedGraphData: ProcessedGraphData = {
  nodes: [
    { id: "1", key: "Person", label: "Person", color: "#6366f1", properties: { name: "Alice", role: "Engineer" } },
    { id: "2", key: "Company", label: "Company", color: "#f59e0b", properties: { name: "Acme Corp" } },
    { id: "3", key: "Person", label: "Person", color: "#6366f1", properties: { name: "Bob", role: "Manager" } },
  ],
  links: [
    { id: "e1", source: "1", target: "2", key: "WORKS_AT", label: "Works At", color: "#94a3b8", properties: { since: 2020 } },
    { id: "e2", source: "3", target: "2", key: "WORKS_AT", label: "Works At", color: "#94a3b8", properties: { since: 2021 } },
  ],
};
