import { describe, expect, it } from "vitest";
import { createUsageDonutOption } from "../components/charts/UsageDonut";

describe("createUsageDonutOption", () => {
  it("enables hover enlarge and shows minute-and-percent tooltip", () => {
    const option = createUsageDonutOption([
      { appName: "Microsoft Edge", seconds: 600, category: "learning" },
      { appName: "VS Code", seconds: 1200, category: "development" },
    ]);
    const tooltip = Array.isArray(option.tooltip) ? option.tooltip[0] : option.tooltip;
    const series = Array.isArray(option.series) ? option.series[0] : option.series;
    const emphasis =
      series && "emphasis" in series && typeof series.emphasis === "object"
        ? series.emphasis
        : undefined;
    const scaleSize =
      emphasis && "scaleSize" in emphasis && typeof emphasis.scaleSize === "number"
        ? emphasis.scaleSize
        : undefined;
    const tooltipFormatter =
      tooltip && "formatter" in tooltip && typeof tooltip.formatter === "function"
        ? tooltip.formatter
        : undefined;
    const tooltipText =
      typeof tooltipFormatter === "function"
        ? tooltipFormatter(
            {
              name: "Microsoft Edge",
              value: 600,
              percent: 33,
            } as never,
            "" as never,
            () => "" as never,
          )
        : "";

    expect(tooltip?.trigger).toBe("item");
    expect(tooltipText).toContain("10 分钟");
    expect(tooltipText).toContain("33%");
    expect(series?.silent).toBe(false);
    expect(scaleSize).toBe(10);
  });
});
