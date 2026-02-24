import { config } from "@/lib/config";
import { mande } from "mande";

export const proxy = (path: string) => {
  const api = mande(`${config.env.API_URL}${path}`);
  api.options.headers.user_id = "019c8f7f-1893-7f72-9cdd-4719aec6af94";

  return api;
};
