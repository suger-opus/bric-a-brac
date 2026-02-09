"use client";

import { Badge } from "@/components/ui/badge";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@/components/ui/hover-card";
import { Separator } from "@/components/ui/separator";
import { Property, PropertyType, RequestProperty } from "@/types";
import { Fragment } from "react/jsx-runtime";

type PropertyTypeBadgeProps = {
  property: Property | RequestProperty;
};

const PropertyTypeBadge = ({ property }: PropertyTypeBadgeProps) => {
  return property.property_type === PropertyType.SELECT
    ? (
      <HoverCard>
        <HoverCardTrigger className="cursor-pointer">
          <Badge variant="outline" className="font-bold text-[9px]">
            {property.property_type.toUpperCase()}{" "}
            ({property.metadata.options!.length})
          </Badge>
        </HoverCardTrigger>
        <HoverCardContent className="w-fit">
          <span className="text-sm font-semibold">
            {property.label} options ({property.metadata.options!.length})
          </span>
          <div className="mt-3 max-h-80 overflow-y-auto no-scrollbar">
            {property.metadata.options!.map((p, index) => (
              <Fragment key={index}>
                <div className="text-sm">{p}</div>
                {index < property.metadata.options!.length - 1 && (
                  <Separator className="my-2" />
                )}
              </Fragment>
            ))}
          </div>
        </HoverCardContent>
      </HoverCard>
    )
    : (
      <Badge variant="outline" className="font-bold text-[9px]">
        {property.property_type.toUpperCase()}
      </Badge>
    );
};

export default PropertyTypeBadge;
