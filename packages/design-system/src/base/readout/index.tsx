import type { ReactNode } from "react";

export function Readout({
  label,
  value,
  action,
}: {
  label: string;
  value: ReactNode;
  action?: ReactNode;
}) {
  return (
    <div className="rf-readout rf-paper-surface">
      <div>
        <span>{label}</span>
        <strong>{value}</strong>
      </div>
      {action}
    </div>
  );
}
