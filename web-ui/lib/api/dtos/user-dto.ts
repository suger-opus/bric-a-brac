import * as v from "valibot";

export const UserDto = v.object({
  user_id: v.string(),
  username: v.string(),
  email: v.string(),
  created_at: v.pipe(v.string(), v.isoTimestamp()),
  updated_at: v.pipe(v.string(), v.isoTimestamp())
});
