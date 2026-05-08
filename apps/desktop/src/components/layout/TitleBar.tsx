import { useEffect, useState } from "react";

type WindowApi = {
  minimize: () => Promise<void>;
  maximize: () => Promise<void>;
  unmaximize: () => Promise<void>;
  isMaximized: () => Promise<boolean>;
  close: () => Promise<void>;
};

async function getWindowApi(): Promise<WindowApi> {
  const { getCurrentWindow } = await import("@tauri-apps/api/window");
  return getCurrentWindow();
}

function MinimizeIcon() {
  return (
    <svg viewBox="0 0 14 14" aria-hidden="true" className="titlebar__icon">
      <path d="M3 10H11" />
    </svg>
  );
}

function MaximizeIcon({ maximized }: { maximized: boolean }) {
  if (maximized) {
    return (
      <svg viewBox="0 0 14 14" aria-hidden="true" className="titlebar__icon">
        <path d="M4.5 5V3.75C4.5 3.34 4.84 3 5.25 3H10.25C10.66 3 11 3.34 11 3.75V8.75C11 9.16 10.66 9.5 10.25 9.5H9" />
        <path d="M3.75 4.5H8.75C9.16 4.5 9.5 4.84 9.5 5.25V10.25C9.5 10.66 9.16 11 8.75 11H3.75C3.34 11 3 10.66 3 10.25V5.25C3 4.84 3.34 4.5 3.75 4.5Z" />
      </svg>
    );
  }

  return (
    <svg viewBox="0 0 14 14" aria-hidden="true" className="titlebar__icon">
      <rect x="3" y="3" width="8" height="8" />
    </svg>
  );
}

function CloseIcon() {
  return (
    <svg viewBox="0 0 14 14" aria-hidden="true" className="titlebar__icon">
      <path d="M4 4L10 10" />
      <path d="M10 4L4 10" />
    </svg>
  );
}

export function TitleBar() {
  const [isMaximized, setIsMaximized] = useState(false);

  useEffect(() => {
    let cancelled = false;

    void (async () => {
      try {
        const windowApi = await getWindowApi();
        const maximized = await windowApi.isMaximized();

        if (!cancelled) {
          setIsMaximized(maximized);
        }
      } catch (error) {
        console.error("读取窗口最大化状态失败", error);
      }
    })();

    return () => {
      cancelled = true;
    };
  }, []);

  const handleMinimize = async () => {
    try {
      const windowApi = await getWindowApi();
      await windowApi.minimize();
    } catch (error) {
      console.error("最小化窗口失败", error);
    }
  };

  const handleToggleMaximize = async () => {
    try {
      const windowApi = await getWindowApi();
      const maximized = await windowApi.isMaximized();

      if (maximized) {
        await windowApi.unmaximize();
        setIsMaximized(false);
      } else {
        await windowApi.maximize();
        setIsMaximized(true);
      }
    } catch (error) {
      console.error("切换窗口大小失败", error);
    }
  };

  const handleClose = async () => {
    try {
      const windowApi = await getWindowApi();
      await windowApi.close();
    } catch (error) {
      console.error("关闭窗口失败", error);
    }
  };

  return (
    <header className="titlebar">
      <div className="titlebar__drag-region" data-tauri-drag-region>
        <div className="titlebar__brand-mark" />
        <div className="titlebar__meta">
          <strong>TimeRecord</strong>
          <span>前台时长记录器</span>
        </div>
      </div>

      <div className="titlebar__actions">
        <button
          type="button"
          className="titlebar__button"
          aria-label="最小化"
          onClick={() => void handleMinimize()}
        >
          <MinimizeIcon />
        </button>
        <button
          type="button"
          className="titlebar__button"
          aria-label={isMaximized ? "还原窗口" : "最大化窗口"}
          onClick={() => void handleToggleMaximize()}
        >
          <MaximizeIcon maximized={isMaximized} />
        </button>
        <button
          type="button"
          className="titlebar__button titlebar__button--danger"
          aria-label="关闭"
          onClick={() => void handleClose()}
        >
          <CloseIcon />
        </button>
      </div>
    </header>
  );
}
