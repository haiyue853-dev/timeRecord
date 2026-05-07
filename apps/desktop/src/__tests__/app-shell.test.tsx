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

    expect(screen.getByText("TimeRecord")).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "仪表盘" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "应用" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "窗口" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "历史" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "设置" })).toBeInTheDocument();
    expect(
      screen.getByRole("heading", { name: "本次开机概览" }),
    ).toBeInTheDocument();
  });
});
