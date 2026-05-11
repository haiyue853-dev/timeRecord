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
    expect(screen.getByRole("link", { name: /^概览/ })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: /^应用统计/ })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: /^会话走势/ })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: /^偏好设置/ })).toBeInTheDocument();
    expect(screen.queryByRole("link", { name: /^窗口焦点/ })).not.toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "今日概览" })).toBeInTheDocument();
  });
});
