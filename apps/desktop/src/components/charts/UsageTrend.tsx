import type { EChartsOption } from "echarts";
import ReactECharts from "echarts-for-react";
import type { TrendPoint } from "../../types/stats";

export function createUsageTrendOption(points: TrendPoint[]): EChartsOption {
  return {
    animation: false,
    tooltip: {
      show: true,
      trigger: "axis",
      backgroundColor: "#fff8de",
      borderColor: "#111111",
      borderWidth: 3,
      textStyle: {
        color: "#111111",
        fontWeight: 900,
      },
      formatter: (params: unknown) => {
        const first = Array.isArray(params) ? params[0] : undefined;
        if (!first || typeof first !== "object") {
          return "";
        }

        const label =
          "axisValueLabel" in first && typeof first.axisValueLabel === "string"
            ? first.axisValueLabel
            : "";
        const value =
          "value" in first && typeof first.value === "number" ? first.value : 0;

        return `${label}<br/>活跃 ${Math.round(value / 60)} 分钟`;
      },
      extraCssText: "box-shadow: 4px 4px 0 #111111; border-radius: 0;",
    },
    grid: {
      left: 8,
      right: 8,
      top: 24,
      bottom: 16,
      containLabel: true,
    },
    xAxis: {
      type: "category",
      data: points.map((point) => point.label),
      axisLine: { lineStyle: { color: "#111111", width: 3 } },
      axisTick: { show: false },
      axisLabel: { color: "#111111", fontWeight: 700 },
      axisPointer: {
        show: true,
        lineStyle: {
          color: "#111111",
          width: 2,
          type: "dashed",
        },
      },
    },
    yAxis: {
      type: "value",
      axisLine: { show: false },
      splitLine: { lineStyle: { color: "#111111", width: 1, type: "dashed" } },
      axisLabel: {
        color: "#111111",
        fontWeight: 700,
        formatter: (value: number) => `${Math.round(value / 60)}m`,
      },
    },
    series: [
      {
        type: "line",
        silent: true,
        smooth: false,
        data: points.map((point) => point.activeSeconds),
        symbol: "circle",
        symbolSize: 10,
        emphasis: {
          scale: true,
          focus: "series",
          itemStyle: {
            color: "#ff5a36",
            borderColor: "#111111",
            borderWidth: 3,
          },
          lineStyle: {
            width: 5,
          },
        },
        lineStyle: {
          width: 4,
          color: "#1167ff",
        },
        itemStyle: {
          color: "#ffd500",
          borderColor: "#111111",
          borderWidth: 3,
        },
      },
    ],
  };
}

export function UsageTrend({ points }: { points: TrendPoint[] }) {
  if (import.meta.env.MODE === "test") {
    return <div aria-label="usage-trend">{points.length} points</div>;
  }

  if (points.length === 0) {
    return <div className="history-empty">还在等待足够的会话数据，继续使用一会儿就会出现真实曲线。</div>;
  }

  return (
    <ReactECharts
      style={{ height: 280 }}
      opts={{ renderer: "canvas" }}
      option={createUsageTrendOption(points)}
    />
  );
}
