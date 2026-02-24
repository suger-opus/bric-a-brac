"use client";

import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator
} from "@/components/ui/breadcrum";
import {
  Menubar,
  MenubarContent,
  MenubarMenu,
  MenubarRadioGroup,
  MenubarRadioItem,
  MenubarSeparator,
  MenubarTrigger
} from "@/components/ui/menubar";
import { useGraph } from "@/contexts/graph-context";
import { actionLabels, Category, categoryLabels } from "@/lib/actions";
import { Action, Role } from "@/types";
import { useEffect, useEffectEvent, useState } from "react";

const GraphMenu = () => {
  const { metadata, action, setAction } = useGraph();
  const [menuItem, setMenuItem] = useState<Action>(action!);

  const handleActionChange = useEffectEvent((newAction: Action) => {
    setMenuItem(newAction);
  });

  const handleItemChange = (value: string) => {
    setMenuItem(value as Action);
    setAction(value as Action);
  };

  useEffect(() => {
    if (action) {
      handleActionChange(action);
    }
  }, [action]);

  return (
    <div className="h-fit w-full space-y-1">
      <Menubar className="w-72">
        {[Role.OWNER, Role.ADMIN, Role.EDITOR, Role.VIEWER, Role.NONE].includes(metadata!.user_role)
          && (
            <MenubarMenu>
              <MenubarTrigger>{categoryLabels[Category.SEARCH]}</MenubarTrigger>
              <MenubarContent>
                <MenubarRadioGroup value={menuItem} onValueChange={handleItemChange}>
                  <MenubarRadioItem value={Action.ASK_AI as string} disabled>
                    {actionLabels[Action.ASK_AI]}
                  </MenubarRadioItem>
                  <MenubarSeparator />
                  <MenubarRadioItem value={Action.FIND_NODE as string} disabled>
                    {actionLabels[Action.FIND_NODE]}
                  </MenubarRadioItem>
                  <MenubarRadioItem value={Action.FIND_PATH as string} disabled>
                    {actionLabels[Action.FIND_PATH]}
                  </MenubarRadioItem>
                </MenubarRadioGroup>
              </MenubarContent>
            </MenubarMenu>
          )}
        {[Role.OWNER, Role.ADMIN, Role.EDITOR].includes(metadata!.user_role)
          && (
            <MenubarMenu>
              <MenubarTrigger>{categoryLabels[Category.EDIT]}</MenubarTrigger>
              <MenubarContent>
                <MenubarRadioGroup value={menuItem} onValueChange={handleItemChange}>
                  <MenubarRadioItem value={Action.BUILD_WITH_AI}>
                    {actionLabels[Action.BUILD_WITH_AI]}
                  </MenubarRadioItem>
                  <MenubarSeparator />
                  <MenubarRadioItem value={Action.NEW_NODE_TYPE}>
                    {actionLabels[Action.NEW_NODE_TYPE]}
                  </MenubarRadioItem>
                  <MenubarRadioItem value={Action.MANAGE_NODE_TYPES} disabled>
                    {actionLabels[Action.MANAGE_NODE_TYPES]}
                  </MenubarRadioItem>
                  <MenubarRadioItem value={Action.INSERT_NODE}>
                    {actionLabels[Action.INSERT_NODE]}
                  </MenubarRadioItem>
                  <MenubarRadioItem value={Action.MANAGE_NODES} disabled>
                    {actionLabels[Action.MANAGE_NODES]}
                  </MenubarRadioItem>
                  <MenubarSeparator />
                  <MenubarRadioItem value={Action.NEW_EDGE_TYPE}>
                    {actionLabels[Action.NEW_EDGE_TYPE]}
                  </MenubarRadioItem>
                  <MenubarRadioItem value={Action.MANAGE_EDGE_TYPES} disabled>
                    {actionLabels[Action.MANAGE_EDGE_TYPES]}
                  </MenubarRadioItem>
                  <MenubarRadioItem value={Action.INSERT_EDGE}>
                    {actionLabels[Action.INSERT_EDGE]}
                  </MenubarRadioItem>
                  <MenubarRadioItem value={Action.MANAGE_EDGES} disabled>
                    {actionLabels[Action.MANAGE_EDGES]}
                  </MenubarRadioItem>
                </MenubarRadioGroup>
              </MenubarContent>
            </MenubarMenu>
          )}
        {[Role.OWNER, Role.ADMIN].includes(metadata!.user_role)
          && (
            <MenubarMenu>
              <MenubarTrigger>{categoryLabels[Category.ADMIN]}</MenubarTrigger>
              <MenubarContent>
                <MenubarRadioGroup value={menuItem} onValueChange={handleItemChange}>
                  <MenubarRadioItem value={Action.METADATA} disabled>
                    {actionLabels[Action.METADATA]}
                  </MenubarRadioItem>
                  <MenubarRadioItem value={Action.ACCESSES} disabled>
                    {actionLabels[Action.ACCESSES]}
                  </MenubarRadioItem>
                  <MenubarRadioItem value={Action.VISIBILITY} disabled>
                    {actionLabels[Action.VISIBILITY]}
                  </MenubarRadioItem>
                  <MenubarRadioItem value={Action.ANALYTICS} disabled>
                    {actionLabels[Action.ANALYTICS]}
                  </MenubarRadioItem>
                </MenubarRadioGroup>
              </MenubarContent>
            </MenubarMenu>
          )}
        {[Role.OWNER].includes(metadata!.user_role)
          && (
            <MenubarMenu>
              <MenubarTrigger>{categoryLabels[Category.OWNER]}</MenubarTrigger>
              <MenubarContent>
                <MenubarRadioGroup value={menuItem} onValueChange={handleItemChange}>
                  <MenubarRadioItem value={Action.DELETE_GRAPH} disabled>
                    {actionLabels[Action.DELETE_GRAPH]}
                  </MenubarRadioItem>
                </MenubarRadioGroup>
              </MenubarContent>
            </MenubarMenu>
          )}
      </Menubar>
      <Breadcrumb className="ml-2">
        <BreadcrumbList className="sm:gap-0.5">
          <BreadcrumbItem>
            <BreadcrumbPage className="text-xs font-semibold">
              {[Action.FIND_NODE, Action.FIND_PATH, Action.ASK_AI].includes(action!)
                && "Search"}
              {[
                Action.BUILD_WITH_AI,
                Action.NEW_NODE_TYPE,
                Action.NEW_EDGE_TYPE,
                Action.MANAGE_NODE_TYPES,
                Action.MANAGE_EDGE_TYPES,
                Action.INSERT_NODE,
                Action.INSERT_EDGE,
                Action.MANAGE_NODES,
                Action.MANAGE_EDGES
              ].includes(action!) && "Edit"}
              {[Action.METADATA, Action.ACCESSES, Action.VISIBILITY, Action.ANALYTICS]
                .includes(action!) && "Admin"}
              {[Action.DELETE_GRAPH].includes(action!) && "Owner"}
            </BreadcrumbPage>
          </BreadcrumbItem>
          <BreadcrumbSeparator className="text-black" />
          <BreadcrumbItem>
            <BreadcrumbPage className="text-xs font-semibold">
              {actionLabels[action!]}
            </BreadcrumbPage>
          </BreadcrumbItem>
        </BreadcrumbList>
      </Breadcrumb>
    </div>
  );
};

export default GraphMenu;
