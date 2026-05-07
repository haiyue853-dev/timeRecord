import "@testing-library/jest-dom/vitest";
import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { SettingsPage } from "../pages/SettingsPage";

describe("SettingsPage", () => {
  it("shows idle minutes and DeepSeek toggle", () => {
    render(<SettingsPage />);

    expect(screen.getByText("空闲判定")).toBeInTheDocument();
    expect(screen.getByLabelText("启用 AI 总结")).toBeInTheDocument();
  });
});
