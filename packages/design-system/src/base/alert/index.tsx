import { CircleAlert, CircleCheck, Info } from "lucide-react";
import type { ComponentProps, ReactNode } from "react";
import type { Tone } from "../types";

export function Alert({
  title,
  children,
  tone = "sapphire",
  ...props
}: Omit<ComponentProps<"div">, "title"> & {
  title: string;
  children: ReactNode;
  tone?: Tone;
}) {
  const Icon =
    tone === "emerald"
      ? CircleCheck
      : tone === "ruby" || tone === "gold"
        ? CircleAlert
        : Info;
  return (
    <div
      {...props}
      className="rf-alert rf-paper-surface"
      data-tone={tone}
      role={tone === "ruby" ? "alert" : "status"}
    >
      <span className="rf-alert-icon" aria-hidden="true">
        <Icon aria-hidden="true" />
      </span>
      <div className="rf-alert-copy">
        <strong className="rf-alert-title">{title}</strong>
        <div className="rf-alert-description">{children}</div>
      </div>
    </div>
  );
}
