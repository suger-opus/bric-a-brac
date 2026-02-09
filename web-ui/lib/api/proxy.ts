import { config } from "@/lib/config";
import { mande } from "mande";

export const proxy = (path: string) => {
  const api = mande(`${config.env.API_URL}${path}`);
  api.options.headers.user_id = "019c4329-091b-70c3-a96d-ab176005e68e";

  return api;
};
