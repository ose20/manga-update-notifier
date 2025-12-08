// レイアウト + ルータ呼び出しのみ

import { AppLayout } from "@/shared/components/layout/AppLayout";
import { AppRouter } from "./router";

export function App() {
    return (
        <AppLayout>
            <AppRouter />
        </AppLayout>
    )
}