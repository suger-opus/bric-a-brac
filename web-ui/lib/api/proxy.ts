import { config } from "@/lib/config";
import { mande } from "mande";

export const proxy = (path: string) => {
  const api = mande(`${config.env.API_URL}${path}`);
  api.options.headers.user_id = "019cf77a-3abb-7611-ad87-df3fb443dff6";

  return api;
};
