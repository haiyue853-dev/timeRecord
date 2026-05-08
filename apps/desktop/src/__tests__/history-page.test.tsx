import "@testing-library/jest-dom/vitest";
import { render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { HistoryPage } from "../pages/HistoryPage";
import { useStatsStore } from "../store/useStatsStore";

describe("HistoryPage", () => {
  afterEach(() => {
    useStatsStore.setState({
      totalActiveSeconds: 0,
      currentAppName: "等待活动采集",
      currentWindowTitle: "等待检测到前台窗口",
      apps: [
        {
          appName: "等待活动采集",
          seconds: 0,
          category: "system",
        },
      ],
      summary: "昨天还没有记录到有效前台时长，今天先别急着追求完美，先让第一段专注发生。",
      encouragement: "先开始 25 分钟，状态就会慢慢回来。",
      summarySource: "local",
      trendPoints: [],
      weeklySummary: {
        currentWeekTotalSeconds: 0,
        previousWeekTotalSeconds: 0,
        currentWeekLearningSeconds: 0,
        currentWeekAverageSeconds: 0,
        deltaSeconds: 0,
        bestDay: {
          date: "2026-05-08",
          totalActiveSeconds: 0,
          learningSeconds: 0,
          developmentSeconds: 0,
        },
      },
      learningHeatmap: [],
    });
  });

  it("renders the persisted weekly summary and learning heatmap", () => {
    useStatsStore.setState({
      totalActiveSeconds: 12_000,
      currentAppName: "Microsoft Edge",
      currentWindowTitle: "课程播放页",
      apps: [
        {
          appName: "Microsoft Edge",
          seconds: 12_000,
          category: "learning",
        },
      ],
      summary: "昨天你累计活跃了 120 分钟，比前天多了 20 分钟。",
      encouragement: "继续保持，今天把昨天最关键的一点用出来。",
      summarySource: "local",
      trendPoints: [
        { label: "09:00", activeSeconds: 180 },
        { label: "09:05", activeSeconds: 240 },
      ],
      weeklySummary: {
        currentWeekTotalSeconds: 18_000,
        previousWeekTotalSeconds: 12_000,
        currentWeekLearningSeconds: 9_600,
        currentWeekAverageSeconds: 2_571,
        deltaSeconds: 6_000,
        bestDay: {
          date: "2026-05-06",
          totalActiveSeconds: 4_800,
          learningSeconds: 3_600,
          developmentSeconds: 1_200,
        },
      },
      learningHeatmap: Array.from({ length: 28 }, (_, index) => ({
        date: `2026-05-${String(index + 1).padStart(2, "0")}`,
        learningSeconds: index % 3 === 0 ? 3_600 : 0,
        level: index % 5,
      })),
    });

    render(<HistoryPage />);

    expect(screen.getByText("最近 7 天对比")).toBeInTheDocument();
    expect(screen.getByText("学习热力图")).toBeInTheDocument();
    expect(screen.getByLabelText("usage-trend")).toHaveTextContent("2 points");
    expect(screen.getAllByLabelText("heatmap-cell")).toHaveLength(28);
  });
});
