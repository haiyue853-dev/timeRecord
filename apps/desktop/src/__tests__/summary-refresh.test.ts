import { describe, expect, it } from "vitest";
import { getTodayNote, shouldRefreshSummary, shouldRunForegroundRefresh } from "../App";

describe("summary cadence", () => {
  it("only refreshes encouragement after three hours", () => {
    const now = Date.UTC(2026, 4, 8, 12, 0, 0);
    expect(shouldRefreshSummary(null, now)).toBe(true);
    expect(shouldRefreshSummary(now - 60 * 60 * 1000, now)).toBe(false);
    expect(shouldRefreshSummary(now - 3 * 60 * 60 * 1000, now)).toBe(true);
  });

  it("keeps the same today note within the same day", () => {
    const morning = Date.UTC(2026, 4, 8, 9, 0, 0);
    const evening = Date.UTC(2026, 4, 8, 21, 0, 0);
    const nextDay = Date.UTC(2026, 4, 9, 9, 0, 0);

    expect(getTodayNote(morning)).toBe(getTodayNote(evening));
    expect(getTodayNote(nextDay)).not.toBe("");
  });

  it("only runs foreground polling when the window is visible and focused", () => {
    const originalVisibility = document.visibilityState;
    const originalFocus = document.hasFocus;

    Object.defineProperty(document, "visibilityState", {
      configurable: true,
      value: "visible",
    });
    document.hasFocus = () => true;
    expect(shouldRunForegroundRefresh()).toBe(true);

    Object.defineProperty(document, "visibilityState", {
      configurable: true,
      value: "hidden",
    });
    expect(shouldRunForegroundRefresh()).toBe(false);

    Object.defineProperty(document, "visibilityState", {
      configurable: true,
      value: originalVisibility,
    });
    document.hasFocus = originalFocus;
  });
});
