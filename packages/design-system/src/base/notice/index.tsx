import type { ReactNode } from "react";

export function Notice({
  children,
  tone = "neutral",
  variant = "floating",
  action,
  announce = true,
  className = "",
}: {
  children: ReactNode;
  tone?: "neutral" | "error";
  variant?: "inline" | "floating";
  action?: ReactNode;
  announce?: boolean;
  className?: string;
}) {
  return (
    <div
      className={`rf-notice ${variant === "floating" ? "rf-paper-surface" : ""} ${className}`}
      data-notice-tone={tone}
      data-variant={variant}
    >
      <span
        className="rf-notice-text"
        role={announce ? (tone === "error" ? "alert" : "status") : undefined}
      >
        {children}
      </span>
      {action}
    </div>
  );
}
