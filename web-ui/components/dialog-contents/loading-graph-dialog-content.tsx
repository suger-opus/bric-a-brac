"use client";

import { Button } from "@/components/ui/button";
import {
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import { Field, FieldDescription, FieldError, FieldLabel } from "@/components/ui/field";
import { Progress } from "@/components/ui/progress";
import { Spinner } from "@/components/ui/spinner";
import { useGraph } from "@/contexts/graph-context";
import { CornerRightDownIcon, HomeIcon } from "lucide-react";
import { useEffect, useEffectEvent, useState } from "react";

type LoadingGraphDialogContentProps = {
  onClose: () => void;
};

const LoadingGraphDialogContent = ({ onClose }: LoadingGraphDialogContentProps) => {
  const [progress, setProgress] = useState(0);
  const { isLoading, isLoaded, error } = useGraph();

  const handleClose = () => {
    if (isLoaded) {
      onClose();
    }
  };

  useEffect(() => {
    if (progress >= 90 || !isLoading) { return; }

    const interval = setInterval(() => {
      setProgress((prev) => {
        if (prev >= 90) { return 90; }
        return prev + 1;
      });
    }, 50);

    return () => clearInterval(interval);
  }, [progress, isLoading]);

  const completeProgress = useEffectEvent(() => {
    if (isLoaded) {
      setProgress(100);
    }
  });

  useEffect(() => {
    completeProgress();
  }, [isLoaded]);

  return (
    <DialogContent showCloseButton={false}>
      <DialogHeader className="sr-only">
        <DialogTitle>Loading Graph</DialogTitle>
        <DialogDescription>Loading graph data, please wait.</DialogDescription>
      </DialogHeader>
      <div>
        <Field>
          <FieldLabel htmlFor="data-fetching">
            <span>
              {isLoaded ? "Data loaded" : isLoading ? "Data loading..." : "Data loading stopped"}
            </span>
            <span className="ml-auto">{progress}%</span>
          </FieldLabel>
          <Progress value={progress} id="data-fetching" />
          {error && <FieldError>{error}</FieldError>}
          {isLoaded && <FieldDescription>Graph data loaded successfully.</FieldDescription>}
        </Field>
      </div>
      <DialogFooter>
        <Button type="button" onClick={handleClose} disabled={isLoading}>
          {isLoaded
            ? (
              <>
                Access Graph<CornerRightDownIcon />
              </>
            )
            : isLoading
            ? <Spinner />
            : (
              <>
                <HomeIcon />Exit to Home
              </>
            )}
        </Button>
      </DialogFooter>
    </DialogContent>
  );
};

export default LoadingGraphDialogContent;
