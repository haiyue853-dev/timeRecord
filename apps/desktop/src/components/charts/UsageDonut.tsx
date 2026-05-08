import type { EChartsOption } from "echarts";
import ReactECharts from "echarts-for-react";
import type { AppUsageItem } from "../../types/stats";

export function createUsageDonutOption(items: AppUsageItem[]): EChartsOption {
  return {
    animation: false,
    color: ["#ff5a36", "#ffd500", "#1167ff", "#00c853", "#ff79c6", "#7c4dff"],
    tooltip: {
      trigger: "item",
      backgroundColor: "#fff7db",
      borderColor: "#111111",
      borderWidth: 3,
      textStyle: {
        color: "#111111",
        fontWeight: 700,
      },
      formatter: (params: unknown) => {
        if (!params || typeof params !== "object") {
          return "";
        }

        const name = "name" in params && typeof params.name === "string" ? params.name : "";
        const value = "value" in params && typeof params.value === "number" ? params.value : 0;
        const percent =
          "percent" in params && typeof params.percent === "number" ? params.percent : 0;
        return `${name}<br/>${Math.round(value / 60)} 分钟 · ${Math.round(percent)}%`;
      },
      extraCssText: "box-shadow: 4px 4px 0 #111111; border-radius: 0;",
    },
    series: [
      {
        type: "pie",
        silent: true,
        selectedMode: false,
        radius: ["42%", "72%"],
        emphasis: {
          scale: true,
          scaleSize: 10,
          itemStyle: {
            borderColor: "#111111",
            borderWidth: 5,
          },
        },
        label: {
          color: "#111111",
          fontWeight: 700,
          formatter: "{b}",
        },
        itemStyle: {
          borderColor: "#111111",
          borderWidth: 4,
        },
        data: items.map((item) => ({
          name: item.appName,
          value: item.seconds,
        })),
      },
    ],
  };
}

export function UsageDonut({ items }: { items: AppUsageItem[] }) {
  if (import.meta.env.MODE === "test") {
    return <div aria-label="usage-donut">{items.length} apps</div>;
  }

  return (
    <ReactECharts
      style={{ height: 280 }}
      opts={{ renderer: "canvas" }}
      option={createUsageDonutOption(items)}
    />
  );
}
