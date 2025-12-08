import { useState } from "react";
import type { Manga } from "../types";
import { Card, CardContent, CardHeader, CardTitle } from "@/shared/components/ui/card";
import { MangaForm } from "./MangaForm";
import { MangaTable } from "./MangaTable";

export function MangaPageContent() {
    const [editingManga, setEditingManga] = useState<Manga| null>(null);

    return (
        <div className="space-y-6">
            <div className="text-center">
                <h2 className="text-2xl font-semibold tracking-tight">
                    漫画の一覧と登録
                </h2>
                <p className="mt-1 text-sm text-muted-foreground">
                    登録されたWeb漫画の一覧を表示し、新規登録・編集・削除が行えます。
                </p>
            </div>

            <div className="grid gap-6 md:grid-cols-[minmax(0,1.1fr)_minmax(0,2fr)]">
                <Card className="shadow-sm">
                    <CardHeader className="pb-3">
                        <CardTitle className="text-base font-semibold">
                            { editingManga ? "漫画の編集" : "新規漫画の登録" }
                        </CardTitle>
                    </CardHeader>
                    <CardContent>
                        <MangaForm
                            mode={editingManga ? 'edit' : 'create'}
                            initialValue={editingManga ?? undefined}
                            onSubmitted={() => {
                                // 送信成功後は編集モードを解除してフォームをクリア
                                setEditingManga(null);
                            }}
                            onCancelEdit={() => {
                                setEditingManga(null);
                            }}
                        />

                    </CardContent>
                </Card>

                <Card className="shadow-sm">
                    <CardHeader className="pb-3">
                        <CardTitle className="text-base font-semibold">登録済み漫画一覧</CardTitle>
                    </CardHeader>
                    <CardContent className="p-0">
                        <MangaTable
                            onEdit={(manga) => setEditingManga(manga)}
                        />
                    </CardContent>
                </Card>
            </div>
        </div>
    );
}