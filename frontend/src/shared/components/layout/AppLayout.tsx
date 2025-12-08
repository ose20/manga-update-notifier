import type { ReactNode } from 'react';

interface AppLayoutProps {
  children: ReactNode;
}

export function AppLayout({ children }: AppLayoutProps) {
  return (
    // ⬇ ここを変更（bg-muted → ベージュ系）
    <div className="min-h-screen bg-[#f6f03338]">
      <header className="border-b bg-background/80 backdrop-blur">
        <div className="container mx-auto flex items-center justify-between px-4 py-3">
          <h1 className="text-lg font-semibold tracking-tight">
            漫画最新話更新検知ツール
          </h1>
          <span className="text-xs text-muted-foreground">
            frontend preview
          </span>
        </div>
      </header>

      <main className="container mx-auto px-4 py-6">
        {children}
      </main>
    </div>
  );
}
