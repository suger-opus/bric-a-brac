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
  // CommandShortcut
} from "@/components/ui/command";
import { useGraph } from "@/contexts/graph-context";
import { Role } from "@/types";
import {
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
  const { isLoaded, metadata } = useGraph();

  return (
    <CommandDialog open={isOpen} onOpenChange={onOpenChange} overlayClassName="backdrop-blur-xs">
      <Command>
        <CommandInput placeholder="Type a command or search..." />
        {isLoaded && (
          <CommandList>
            <CommandEmpty>No results found.</CommandEmpty>
            {/* <CommandGroup heading="Suggestions"> */}
            {/* <CommandShortcut>⌘P</CommandShortcut> */}
            {/* </CommandGroup> */}
            <CommandGroup heading="Search">
              <CommandItem>
                <PackageSearchIcon />
                <span>Find Node</span>
              </CommandItem>
              <CommandItem>
                <WorkflowIcon />
                <span>Find Path</span>
              </CommandItem>
              <CommandItem>
                <BrainIcon />
                <span>Ask AI</span>
              </CommandItem>
            </CommandGroup>
            <CommandSeparator />
            {[Role.OWNER, Role.ADMIN, Role.EDITOR].includes(metadata!.user_role) && (
              <CommandGroup heading="Nodes">
                <CommandItem>
                  <PackagePlusIcon />
                  <span>Insert Node</span>
                </CommandItem>
                <CommandItem>
                  <PackageOpenIcon />
                  <span>Manage Nodes</span>
                </CommandItem>
                <CommandItem>
                  <FileBoxIcon />
                  <span>New Node Type</span>
                </CommandItem>
                <CommandItem>
                  <FilePenIcon />
                  <span>Manage Node Types</span>
                </CommandItem>
              </CommandGroup>
            )}
            {[Role.OWNER, Role.ADMIN, Role.EDITOR].includes(metadata!.user_role) && (
              <CommandGroup heading="Edges">
                <CommandItem>
                  <SplineIcon />
                  <span>Insert Edge</span>
                </CommandItem>
                <CommandItem>
                  <SplinePointerIcon />
                  <span>Manage Edges</span>
                </CommandItem>
                <CommandItem>
                  <FileSymlinkIcon />
                  <span>New Edge Type</span>
                </CommandItem>
                <CommandItem>
                  <FilePenIcon />
                  <span>Manage Edge Types</span>
                </CommandItem>
              </CommandGroup>
            )}
            {[Role.OWNER, Role.ADMIN].includes(metadata!.user_role) && (
              <CommandGroup heading="Admin">
                <CommandItem>
                  <CaptionsIcon />
                  <span>Metadata</span>
                </CommandItem>
                <CommandItem>
                  <ShieldUserIcon />
                  <span>Accesses</span>
                </CommandItem>
                <CommandItem>
                  <HatGlassesIcon />
                  <span>Visibility</span>
                </CommandItem>
                <CommandItem>
                  <ChartAreaIcon />
                  <span>Analytics</span>
                </CommandItem>
              </CommandGroup>
            )}
            {[Role.OWNER].includes(metadata!.user_role) && (
              <CommandGroup heading="Owner">
                <CommandItem>
                  <Trash2Icon />
                  <span>Delete Graph</span>
                </CommandItem>
              </CommandGroup>
            )}
          </CommandList>
        )}
      </Command>
    </CommandDialog>
  );
};

export default GraphCommand;
