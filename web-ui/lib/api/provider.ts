import { ApiGraphService, GraphService } from "./services/graph-service";
import { ApiUserService, UserService } from "./services/user-service";

export class ApiProvider {
  public static get userService(): UserService {
    return new ApiUserService();
  }

  public static get graphService(): GraphService {
    return new ApiGraphService();
  }
}
