"use client";

import { Button } from "@/components/ui/button";
import { Field, FieldDescription, FieldError, FieldLabel } from "@/components/ui/field";
import { InputGroup, InputGroupInput } from "@/components/ui/input-group";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
import { HexColorPicker } from "react-colorful";

type ColorPickerProps = {
  color: string;
  setColor: (color: string) => void;
  validationError?: string | null;
};

const ColorPickerField = ({ color, setColor, validationError }: ColorPickerProps) => {
  return (
    <Field>
      <FieldLabel htmlFor="new-color">Color</FieldLabel>
      <FieldDescription className="text-xs">
        Color used to display nodes of this type.
      </FieldDescription>
      <Popover>
        <PopoverTrigger asChild>
          <Button
            variant="outline"
            className="w-full justify-start pl-3"
          >
            <div
              className="w-6 h-6 rounded-md border-2 border-border"
              style={{ backgroundColor: color }}
            />
            <span className="text-sm">{color}</span>
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-fit">
          <HexColorPicker color={color} onChange={setColor} />
          <InputGroup className="mt-3">
            <InputGroupInput
              id="new-color"
              value={color}
              onChange={(e) => setColor(e.target.value)}
              placeholder="#000000"
              maxLength={7}
              className="font-mono w-49"
            />
          </InputGroup>
        </PopoverContent>
      </Popover>
      <FieldError>{validationError}</FieldError>
    </Field>
  );
};

// const ColorPicker = ({ color, setColor }: ColorPickerProps) => {
//   return (
//     <InputGroup>
//       <InputGroupAddon align="block-start">
//         <Label htmlFor="new-color" className="text-foreground">Color</Label>
//         <div className="ml-auto">
//           <CheckIcon size={16} />
//         </div>
//       </InputGroupAddon>
//       <Popover>
//         <PopoverTrigger className="border-none shadow-none mb-0.5 hover:bg-transparent" asChild>
//           <Button
//             variant="outline"
//             className="w-full justify-start gap-3 h-12"
//           >
//             <div
//               className="w-8 h-8 rounded-md border-2 border-border"
//               style={{ backgroundColor: color }}
//             />
//             <span className="text-sm">{color}</span>
//           </Button>
//         </PopoverTrigger>
//         <PopoverContent className="w-fit">
//           <HexColorPicker color={color} onChange={setColor} />
//           <InputGroup className="mt-3">
//             <InputGroupInput
//               id="new-color"
//               value={color}
//               onChange={(e) => setColor(e.target.value)}
//               placeholder="#000000"
//               maxLength={7}
//               className="font-mono w-49"
//             />
//           </InputGroup>
//         </PopoverContent>
//       </Popover>
//     </InputGroup>
//   );
// };

export default ColorPickerField;
