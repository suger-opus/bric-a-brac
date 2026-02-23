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
      return v.parse(UserDto, response);
    } catch (error) {
      console.error("Failed to get user:", error);
      throw error;
    }
  }
}
