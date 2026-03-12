"use client";

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import {
  Drawer,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle
} from "@/components/ui/drawer";
import { useGraph } from "@/contexts/graph-context";
import { useIsMobile } from "@/hooks/use-mobile";
import { Action } from "@/types";
import GenerateContent from "./contents/generate-content";
import NewEdgeDataContent from "./contents/new-edge-data-content";
import NewEdgeSchemaContent from "./contents/new-edge-schema-content";
import NewNodeDataContent from "./contents/new-node-data-content";
import NewNodeSchemaContent from "./contents/new-node-schema-content";
import GraphMenu from "./graph-menu";

const GraphDrawer = () => {
  const { action, setAction } = useGraph();
  const isDesktop = !useIsMobile();
  const isOpen = action !== null;
  const onClose = () => setAction(null);

  return isDesktop
    ? (
      <Dialog open={isOpen} onOpenChange={onClose}>
        <DialogContent className="flex flex-col justify-between h-[90vh] min-w-160">
          <DialogHeader className="sr-only h-fit">
            <DialogTitle>Graph Dialog Title</DialogTitle>
            <DialogDescription>Graph Dialog Description</DialogDescription>
          </DialogHeader>
          <GraphMenu />
          <div className="mt-2 grow no-scrollbar overflow-y-auto">
            {action === Action.BUILD_WITH_AI && <GenerateContent onClose={onClose} />}
            {action === Action.NEW_NODE_TYPE && <NewNodeSchemaContent />}
            {action === Action.NEW_EDGE_TYPE && <NewEdgeSchemaContent />}
            {action === Action.INSERT_NODE && <NewNodeDataContent />}
            {action === Action.INSERT_EDGE && <NewEdgeDataContent />}
          </div>
        </DialogContent>
      </Dialog>
    )
    : (
      <Drawer open={isOpen} onOpenChange={onClose}>
        <DrawerContent>
          <DrawerHeader className="sr-only h-fit">
            <DrawerTitle>Graph Drawer Title</DrawerTitle>
            <DrawerDescription>Graph Drawer Description</DrawerDescription>
          </DrawerHeader>
          <GraphMenu />
          <div className="grow no-scrollbar overflow-y-auto">
            {action === Action.BUILD_WITH_AI && <GenerateContent onClose={onClose} />}
            {action === Action.NEW_NODE_TYPE && <NewNodeSchemaContent />}
            {action === Action.NEW_EDGE_TYPE && <NewEdgeSchemaContent />}
            {action === Action.INSERT_NODE && <NewNodeDataContent />}
            {action === Action.INSERT_EDGE && <NewEdgeDataContent />}
          </div>
          <DrawerFooter>
          </DrawerFooter>
        </DrawerContent>
      </Drawer>
    );
};

export default GraphDrawer;
