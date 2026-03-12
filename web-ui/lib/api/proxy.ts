import { config } from "@/lib/config";
import { mande } from "mande";

export const proxy = (path: string) => {
  const api = mande(`${config.env.API_URL}${path}`);
  api.options.headers.user_id = "019ce2f5-2111-77b2-be2d-46ea843eb0d7";

  return api;
};
