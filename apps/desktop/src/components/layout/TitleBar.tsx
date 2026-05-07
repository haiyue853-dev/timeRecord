async function withWindow(action: (windowApi: {
  minimize: () => Promise<void>;
  toggleMaximize: () => Promise<void>;
  close: () => Promise<void>;
}) => Promise<void>) {
  const { getCurrentWindow } = await import("@tauri-apps/api/window");
  await action(getCurrentWindow());
}

export function TitleBar() {
  return (
    <header className="titlebar">
      <div className="titlebar__drag-region" data-tauri-drag-region>
        <div className="titlebar__brand-mark" />
        <div className="titlebar__meta">
          <strong>TimeRecord</strong>
          <span>Windows Activity Intelligence</span>
        </div>
      </div>

      <div className="titlebar__actions">
        <button
          type="button"
          className="titlebar__button"
          aria-label="最小化"
          onClick={() => void withWindow((windowApi) => windowApi.minimize())}
        >
          <span className="titlebar__glyph titlebar__glyph--minimize" />
        </button>
        <button
          type="button"
          className="titlebar__button"
          aria-label="最大化"
          onClick={() => void withWindow((windowApi) => windowApi.toggleMaximize())}
        >
          <span className="titlebar__glyph titlebar__glyph--maximize" />
        </button>
        <button
          type="button"
          className="titlebar__button titlebar__button--danger"
          aria-label="关闭"
          onClick={() => void withWindow((windowApi) => windowApi.close())}
        >
          <span className="titlebar__glyph titlebar__glyph--close" />
        </button>
      </div>
    </header>
  );
}
