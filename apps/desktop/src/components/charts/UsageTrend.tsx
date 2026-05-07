import ReactECharts from "echarts-for-react";

export function UsageTrend({ values }: { values: number[] }) {
  if (import.meta.env.MODE === "test") {
    return <div aria-label="usage-trend">{values.length} points</div>;
  }

  return (
    <ReactECharts
      style={{ height: 220 }}
      option={{
        xAxis: {
          type: "category",
          data: values.map((_, index) => `T${index + 1}`),
        },
        yAxis: { type: "value" },
        series: [
          {
            type: "line",
            smooth: true,
            data: values,
          },
        ],
      }}
    />
  );
}
