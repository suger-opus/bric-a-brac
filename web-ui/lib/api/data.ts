import { ProcessedGraphData } from "@/types";

export const sampleProcessedGraphData: ProcessedGraphData = {
  nodes: [
    {
      id: "node-data-1",
      key: "Person",
      color: "#808080",
      properties: { Name: "Alice", Job_Title: "Engineer" }
    },
    {
      id: "node-data-2",
      key: "Company",
      color: "#808080",
      properties: { Name: "Acme Corp" }
    },
    {
      id: "node-data-3",
      key: "Person",
      color: "#808080",
      properties: { Name: "Bob", Job_Title: "Manager", Is_Full_Time: true, Age: 30 }
    }
  ],
  links: [
    {
      id: "edge_data_1",
      source: "node-data-1",
      target: "node-data-2",
      key: "WORKS_AT",
      color: "#808080",
      properties: { Start_Year: 2020 }
    },
    {
      id: "edge_data_2",
      source: "node-data-3",
      target: "node-data-2",
      key: "WORKS_AT",
      color: "#808080",
      properties: { Start_Year: 2021 }
    }
  ]
};
