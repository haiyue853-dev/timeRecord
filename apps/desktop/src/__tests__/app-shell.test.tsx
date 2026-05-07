import "@testing-library/jest-dom/vitest";
import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { describe, expect, it } from "vitest";
import App from "../App";

describe("App shell", () => {
  it("renders the application title, navigation, and dashboard heading", () => {
    render(
      <MemoryRouter>
        <App />
      </MemoryRouter>,
    );

    expect(screen.getByRole("heading", { name: "TimeRecord" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "仪表盘" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "软件统计" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "窗口明细" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "历史趋势" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "设置" })).toBeInTheDocument();
    expect(
      screen.getByRole("heading", { name: "本次开机概览" }),
    ).toBeInTheDocument();
  });
});
