"use client";

import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator
} from "@/components/ui/command";
import { useGraph } from "@/contexts/graph-context";
import { actionLabels, Category, categoryLabels } from "@/lib/actions";
import { Action, Role } from "@/types";
import {
  BrainCircuitIcon,
  BrainIcon,
  CaptionsIcon,
  ChartAreaIcon,
  FileBoxIcon,
  FilePenIcon,
  FileSymlinkIcon,
  HatGlassesIcon,
  PackageOpenIcon,
  PackagePlusIcon,
  PackageSearchIcon,
  ShieldUserIcon,
  SplineIcon,
  SplinePointerIcon,
  Trash2Icon,
  WorkflowIcon
} from "lucide-react";

type GraphCommandProps = {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
};

const GraphCommand = ({ isOpen, onOpenChange }: GraphCommandProps) => {
  const { metadata, isLoaded, setAction } = useGraph();

  const handleSelect = (value: string) => {
    setAction(value as Action);
    onOpenChange(false);
  };

  return (
    <CommandDialog open={isOpen} onOpenChange={onOpenChange} overlayClassName="backdrop-blur-xs">
      <Command>
        <CommandInput placeholder="Type a command or search..." />
        {isLoaded && (
          <CommandList>
            <CommandEmpty>No results found.</CommandEmpty>
            {[Role.OWNER, Role.ADMIN, Role.EDITOR, Role.VIEWER, Role.NONE].includes(
              metadata!.user_role
            ) && (
              <>
                <CommandGroup heading={categoryLabels[Category.SEARCH]}>
                  <CommandItem
                    key={actionLabels[Action.ASK_AI]}
                    disabled
                    value={Action.ASK_AI}
                    onSelect={handleSelect}
                  >
                    <BrainIcon />
                    <span>{actionLabels[Action.ASK_AI]}</span>
                  </CommandItem>
                  <CommandItem
                    key={actionLabels[Action.FIND_NODE]}
                    disabled
                    value={Action.FIND_NODE}
                    onSelect={handleSelect}
                  >
                    <PackageSearchIcon />
                    <span>{actionLabels[Action.FIND_NODE]}</span>
                  </CommandItem>
                  <CommandItem
                    key={actionLabels[Action.FIND_PATH]}
                    disabled
                    value={Action.FIND_PATH}
                    onSelect={handleSelect}
                  >
                    <WorkflowIcon />
                    <span>{actionLabels[Action.FIND_PATH]}</span>
                  </CommandItem>
                </CommandGroup>
                <CommandSeparator />
              </>
            )}
            {[Role.OWNER, Role.ADMIN, Role.EDITOR].includes(
              metadata!.user_role
            ) && (
              <>
                <CommandGroup heading={categoryLabels[Category.EDIT]}>
                  <CommandItem
                    key={actionLabels[Action.BUILD_WITH_AI]}
                    value={Action.BUILD_WITH_AI}
                    onSelect={handleSelect}
                  >
                    <BrainCircuitIcon />
                    <span>{actionLabels[Action.BUILD_WITH_AI]}</span>
                  </CommandItem>
                  <CommandItem
                    key={actionLabels[Action.NEW_NODE_TYPE]}
                    value={Action.NEW_NODE_TYPE}
                    onSelect={handleSelect}
                  >
                    <FileBoxIcon />
                    <span>{actionLabels[Action.NEW_NODE_TYPE]}</span>
                  </CommandItem>
                  <CommandItem
                    key={actionLabels[Action.MANAGE_NODE_TYPES]}
                    disabled
                    value={Action.MANAGE_NODE_TYPES}
                    onSelect={handleSelect}
                  >
                    <FilePenIcon />
                    <span>{actionLabels[Action.MANAGE_NODE_TYPES]}</span>
                  </CommandItem>
                  <CommandItem
                    key={actionLabels[Action.INSERT_NODE]}
                    value={Action.INSERT_NODE}
                    onSelect={handleSelect}
                  >
                    <PackagePlusIcon />
                    <span>{actionLabels[Action.INSERT_NODE]}</span>
                  </CommandItem>
                  <CommandItem
                    key={actionLabels[Action.MANAGE_NODES]}
                    disabled
                    value={Action.MANAGE_NODES}
                    onSelect={handleSelect}
                  >
                    <PackageOpenIcon />
                    <span>{actionLabels[Action.MANAGE_NODES]}</span>
                  </CommandItem>
                  <CommandItem
                    key={actionLabels[Action.NEW_EDGE_TYPE]}
                    value={Action.NEW_EDGE_TYPE}
                    onSelect={handleSelect}
                  >
                    <FileSymlinkIcon />
                    <span>{actionLabels[Action.NEW_EDGE_TYPE]}</span>
                  </CommandItem>
                  <CommandItem
                    key={actionLabels[Action.MANAGE_EDGE_TYPES]}
                    disabled
                    value={Action.MANAGE_EDGE_TYPES}
                    onSelect={handleSelect}
                  >
                    <FilePenIcon />
                    <span>{actionLabels[Action.MANAGE_EDGE_TYPES]}</span>
                  </CommandItem>
                  <CommandItem
                    key={actionLabels[Action.INSERT_EDGE]}
                    value={Action.INSERT_EDGE}
                    onSelect={handleSelect}
                  >
                    <SplineIcon />
                    <span>{actionLabels[Action.INSERT_EDGE]}</span>
                  </CommandItem>
                  <CommandItem
                    key={actionLabels[Action.MANAGE_EDGES]}
                    disabled
                    value={Action.MANAGE_EDGES}
                    onSelect={handleSelect}
                  >
                    <SplinePointerIcon />
                    <span>{actionLabels[Action.MANAGE_EDGES]}</span>
                  </CommandItem>
                </CommandGroup>
                <CommandSeparator />
              </>
            )}
            {[Role.OWNER, Role.ADMIN].includes(
              metadata!.user_role
            ) && (
              <>
                <CommandGroup heading={categoryLabels[Category.ADMIN]}>
                  <CommandItem
                    key={actionLabels[Action.METADATA]}
                    disabled
                    value={Action.METADATA}
                    onSelect={handleSelect}
                  >
                    <CaptionsIcon />
                    <span>{actionLabels[Action.METADATA]}</span>
                  </CommandItem>
                  <CommandItem
                    key={actionLabels[Action.ACCESSES]}
                    disabled
                    value={Action.ACCESSES}
                    onSelect={handleSelect}
                  >
                    <ShieldUserIcon />
                    <span>{actionLabels[Action.ACCESSES]}</span>
                  </CommandItem>
                  <CommandItem
                    key={actionLabels[Action.VISIBILITY]}
                    disabled
                    value={Action.VISIBILITY}
                    onSelect={handleSelect}
                  >
                    <HatGlassesIcon />
                    <span>{actionLabels[Action.VISIBILITY]}</span>
                  </CommandItem>
                  <CommandItem
                    key={actionLabels[Action.ANALYTICS]}
                    disabled
                    value={Action.ANALYTICS}
                    onSelect={handleSelect}
                  >
                    <ChartAreaIcon />
                    <span>{actionLabels[Action.ANALYTICS]}</span>
                  </CommandItem>
                </CommandGroup>
                <CommandSeparator />
              </>
            )}
            {[Role.OWNER, Role.ADMIN].includes(
              metadata!.user_role
            ) && (
              <>
                <CommandGroup heading={categoryLabels[Category.OWNER]}>
                  <CommandItem
                    key={actionLabels[Action.DELETE_GRAPH]}
                    disabled
                    value={Action.DELETE_GRAPH}
                    onSelect={handleSelect}
                  >
                    <Trash2Icon />
                    <span>{actionLabels[Action.DELETE_GRAPH]}</span>
                  </CommandItem>
                </CommandGroup>
              </>
            )}
          </CommandList>
        )}
      </Command>
    </CommandDialog>
  );
};

export default GraphCommand;
