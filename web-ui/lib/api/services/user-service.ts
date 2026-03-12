import { UserDto } from "@/lib/api/dtos";
import { proxy } from "@/lib/api/proxy";
import { User } from "@/types";
import * as v from "valibot";

export interface UserService {
  get(): Promise<User>;
}

export class ApiUserService implements UserService {
  private get api() {
    return proxy("/users");
  }

  async get(): Promise<User> {
    try {
      const response = await this.api.get("/me");
      const user = v.safeParse(UserDto, response);
      if (!user.success) {
        console.error("Validation errors:", user.issues);
        throw new Error("Failed to parse user data");
      }
      return user.output;
    } catch (error) {
      console.error("Failed to get user:", error);
      throw error;
    }
  }
}
