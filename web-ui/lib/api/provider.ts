import { AccessService, ApiAccessService } from "./services/access-service";
import { ApiBookmarkService, BookmarkService } from "./services/bookmark-service";
import { ApiCheerService, CheerService } from "./services/cheer-service";
import { ApiGraphService, GraphService } from "./services/graph-service";
import { ApiSearchService, SearchService } from "./services/search-service";
import { ApiUserService, UserService } from "./services/user-service";

export class ApiProvider {
  public static get userService(): UserService {
    return new ApiUserService();
  }

  public static get graphService(): GraphService {
    return new ApiGraphService();
  }

  public static get searchService(): SearchService {
    return new ApiSearchService();
  }

  public static get bookmarkService(): BookmarkService {
    return new ApiBookmarkService();
  }

  public static get accessService(): AccessService {
    return new ApiAccessService();
  }

  public static get cheerService(): CheerService {
    return new ApiCheerService();
  }
}
