import { describe, expect, it } from "vitest";
import { createUsageTrendOption } from "../components/charts/UsageTrend";

describe("createUsageTrendOption", () => {
  it("enables hover feedback without making the chart interactive by click", () => {
    const option = createUsageTrendOption([
      { label: "09:00", activeSeconds: 180 },
      { label: "09:05", activeSeconds: 240 },
    ]);
    const tooltip = Array.isArray(option.tooltip) ? option.tooltip[0] : option.tooltip;
    const series = Array.isArray(option.series) ? option.series[0] : option.series;
    const emphasis =
      series && "emphasis" in series && typeof series.emphasis === "object" ? series.emphasis : undefined;
    const hoverScale =
      emphasis && "scale" in emphasis && typeof emphasis.scale === "boolean" ? emphasis.scale : undefined;

    expect(tooltip?.show).toBe(true);
    expect(tooltip?.trigger).toBe("axis");
    expect(series?.silent).toBe(true);
    expect(hoverScale).toBe(true);
  });
});
