import { ChevronRight } from "lucide-react";
import type { ComponentProps, ReactNode } from "react";

export function SelectableRow({
  label,
  leading,
  detail,
  selected,
  ...props
}: Omit<
  ComponentProps<"button">,
  "className" | "children" | "aria-pressed" | "type"
> & {
  label: string;
  leading?: ReactNode;
  detail?: ReactNode;
  selected: boolean;
}) {
  return (
    <button
      {...props}
      type="button"
      className="rf-selectable-row"
      aria-pressed={selected}
      data-leading={leading !== undefined || undefined}
    >
      {leading !== undefined && (
        <span className="rf-selectable-row-leading">{leading}</span>
      )}
      <span className="rf-selectable-row-label">{label}</span>
      {detail !== undefined && (
        <span className="rf-selectable-row-detail">{detail}</span>
      )}
      <ChevronRight
        size={15}
        aria-hidden="true"
        className="rf-selectable-row-chevron"
      />
    </button>
  );
}
