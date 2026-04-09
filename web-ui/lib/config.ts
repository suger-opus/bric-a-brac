import * as v from "valibot";

const envSchema = v.object({
  API_URL: v.string(),
  USER_ID: v.string()
});

const parsedEnv = v.safeParse(envSchema, {
  API_URL: process.env.NEXT_PUBLIC_API_URL,
  USER_ID: process.env.NEXT_PUBLIC_USER_ID
});

if (!parsedEnv.success) {
  console.error("Invalid environment variables:", parsedEnv);
  throw new Error("Invalid environment variables");
}

export const config = {
  env: parsedEnv.output
};
