import type { ReactNode } from "react";

export function EmptyState({
  title,
  description,
  icon,
  variant = "plain",
  className = "",
  onActivate,
  disabled,
}: {
  title: string;
  description?: string;
  icon?: ReactNode;
  variant?: "plain" | "drop";
  className?: string;
  onActivate?: () => void;
  disabled?: boolean;
}) {
  const content = (
    <>
      {icon && (
        <span className="rf-empty-icon" aria-hidden="true">
          {icon}
        </span>
      )}
      <span className="rf-empty-title">{title}</span>
      {description && <span>{description}</span>}
    </>
  );
  const props = {
    className: `rf-empty-state ${className}`,
    "data-variant": variant,
  };
  return onActivate ? (
    <button {...props} type="button" onClick={onActivate} disabled={disabled}>
      {content}
    </button>
  ) : (
    <div {...props}>{content}</div>
  );
}
