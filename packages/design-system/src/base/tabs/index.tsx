import { Tabs as BaseTabs } from "@base-ui/react/tabs";
import type { ComponentProps, ReactNode } from "react";

export function Tabs({
  label,
  items,
  ...props
}: Omit<ComponentProps<typeof BaseTabs.Root>, "children" | "className"> & {
  label: string;
  items: readonly {
    value: string;
    label: string;
    content: ReactNode;
    disabled?: boolean;
  }[];
}) {
  return (
    <BaseTabs.Root
      defaultValue={items[0]?.value}
      {...props}
      className="rf-tabs rf-paper-surface"
    >
      <BaseTabs.List
        className="rf-tabs-list"
        aria-label={label}
        activateOnFocus={false}
      >
        {items.map((item) => (
          <BaseTabs.Tab
            className="rf-tab"
            key={item.value}
            value={item.value}
            disabled={item.disabled}
          >
            {item.label}
          </BaseTabs.Tab>
        ))}
        <BaseTabs.Indicator className="rf-tab-indicator" aria-hidden="true" />
      </BaseTabs.List>
      {items.map((item) => (
        <BaseTabs.Panel
          className="rf-tab-panel rf-paper-surface"
          key={item.value}
          value={item.value}
        >
          {item.content}
        </BaseTabs.Panel>
      ))}
    </BaseTabs.Root>
  );
}
