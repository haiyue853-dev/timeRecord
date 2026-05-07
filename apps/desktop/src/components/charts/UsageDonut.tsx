import ReactECharts from "echarts-for-react";
import type { AppUsageItem } from "../../types/stats";

export function UsageDonut({ items }: { items: AppUsageItem[] }) {
  if (import.meta.env.MODE === "test") {
    return <div aria-label="usage-donut">{items.length} apps</div>;
  }

  return (
    <ReactECharts
      style={{ height: 260 }}
      option={{
        tooltip: { trigger: "item" },
        series: [
          {
            type: "pie",
            radius: ["45%", "72%"],
            label: { formatter: "{b}" },
            data: items.map((item) => ({
              name: item.appName,
              value: item.seconds,
            })),
          },
        ],
      }}
    />
  );
}
