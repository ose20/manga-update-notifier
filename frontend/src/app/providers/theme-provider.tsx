import { useEffect, type ReactNode } from "react";

type Theme = 'light' | 'dark' | 'system';

interface ThemeProviderProps {
    children: ReactNode;
    defaultTheme?: Theme;
    storageKey?: string;
}


export function ThemeProvider({
  children,
}: ThemeProviderProps) {
    useEffect(() => {
    // 常にダークモードにする
    document.documentElement.classList.add('dark');

    // 念のためクリーンアップ
    return () => {
      document.documentElement.classList.remove('dark');
    };
  }, []);

  return <>{children}</>;
}