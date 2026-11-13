import type { ComponentProps } from "react";

export function Panel({
  className = "",
  children,
  clipped = false,
  ...props
}: ComponentProps<"div"> & { clipped?: boolean }) {
  return (
    <div
      {...props}
      className={`rf-panel rf-paper-surface ${className}`}
      data-clipped={clipped ? "true" : undefined}
    >
      {children}
    </div>
  );
}
