import { config } from "@/lib/config";
import { mande } from "mande";
import { toast } from "sonner";
import type { GenericSchema, InferOutput } from "valibot";
import { safeParse } from "valibot";

const api = mande(config.env.API_URL);
api.options.headers.user_id = "019d0aff-70d9-7151-80e4-d480ead4c7a4";

function validate<T extends GenericSchema>(schema: T, data: unknown): InferOutput<T> {
  const result = safeParse(schema, data);
  if (!result.success) {
    throw new Error("API response validation failed");
  }
  return result.output;
}

export async function get<T extends GenericSchema>(path: string, schema: T): Promise<InferOutput<T>> {
  try {
    const data = await api.get(path);
    return validate(schema, data);
  } catch (error) {
    toast.error(`GET ${path} failed`);
    throw error;
  }
}

export async function post<T extends GenericSchema>(path: string, body: unknown, schema: T): Promise<InferOutput<T>> {
  try {
    const data = await api.post(path, body);
    return validate(schema, data);
  } catch (error) {
    toast.error(`POST ${path} failed`);
    throw error;
  }
}
