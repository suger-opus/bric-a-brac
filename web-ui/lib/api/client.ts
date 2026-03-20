import { config } from "@/lib/config";
import { mande } from "mande";
import type { GenericSchema, InferOutput } from "valibot";
import { safeParse } from "valibot";

const api = mande(config.env.API_URL);
api.options.headers.user_id = "019cfc3c-20c4-7aa2-a098-a547f9f13213";

function validate<T extends GenericSchema>(schema: T, data: unknown): InferOutput<T> {
  const result = safeParse(schema, data);
  if (!result.success) {
    throw new Error("API response validation failed");
  }
  return result.output;
}

export async function get<T extends GenericSchema>(path: string, schema: T): Promise<InferOutput<T>> {
  const data = await api.get(path);
  return validate(schema, data);
}

export async function post<T extends GenericSchema>(path: string, body: unknown, schema: T): Promise<InferOutput<T>> {
  const data = await api.post(path, body);
  return validate(schema, data);
}
