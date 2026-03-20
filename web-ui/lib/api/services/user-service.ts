import { get } from "@/lib/api/client";
import { UserDto } from "@/lib/api/dtos";

export const userService = {
  getCurrent: () => get("/users/me", UserDto),
};
