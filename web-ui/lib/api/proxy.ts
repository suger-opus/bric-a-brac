import { config } from "@/lib/config";
import { mande } from "mande";

export const proxy = (path: string) => {
  const api = mande(`${config.env.API_URL}${path}`);
  api.options.headers.user_id = "019cfc3c-20c4-7aa2-a098-a547f9f13213";

  return api;
};
