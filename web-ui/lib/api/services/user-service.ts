/* eslint-disable no-console */
import { users } from "@/lib/api/data";
import { User } from "@/types";

export interface UserService {
  getProfile(): Promise<User>;
}

export class ApiUserService implements UserService {
  async getProfile(): Promise<User> {
    console.log("Fetching user profile");
    await new Promise((resolve) => setTimeout(resolve, 1000));
    return users[0];
  }
}
