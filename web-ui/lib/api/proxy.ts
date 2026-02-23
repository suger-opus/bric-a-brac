import { config } from "@/lib/config";
import { mande } from "mande";

export const proxy = (path: string) => {
  const api = mande(`${config.env.API_URL}${path}`);
  api.options.headers.user_id = "019c8b5a-85c0-7d13-b9cf-3e03b91ad0ed";

  return api;
};
