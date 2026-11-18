import { Checkbox as BaseCheckbox } from "@base-ui/react/checkbox";
import { Check } from "lucide-react";
import { type ComponentProps, type ReactNode, useId } from "react";

export function Checkbox({
  label,
  markAlignment = "center",
  ...props
}: Omit<ComponentProps<typeof BaseCheckbox.Root>, "className" | "children"> & {
  label: ReactNode;
  markAlignment?: "center" | "start";
}) {
  const generatedId = useId();
  const id = props.id ?? generatedId;
  return (
    <label
      className="rf-checkbox-choice"
      data-mark-alignment={markAlignment}
      htmlFor={id}
    >
      <BaseCheckbox.Root {...props} id={id} className="rf-checkbox">
        <BaseCheckbox.Indicator
          className="rf-checkbox-indicator"
          aria-hidden="true"
        >
          <Check size={15} strokeWidth={3} aria-hidden="true" />
        </BaseCheckbox.Indicator>
      </BaseCheckbox.Root>
      <span>{label}</span>
    </label>
  );
}
