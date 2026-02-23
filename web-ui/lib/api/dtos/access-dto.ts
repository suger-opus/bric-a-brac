import * as v from "valibot";

export enum Role {
  OWNER = "Owner",
  ADMIN = "Admin",
  EDITOR = "Editor",
  VIEWER = "Viewer",
  NONE = "None"
}
export const RoleDto = v.enum(Role);
