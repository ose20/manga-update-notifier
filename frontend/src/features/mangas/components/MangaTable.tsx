import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/shared/components/ui/table";
import { useDeleteManga } from "../hooks/useDeleteManga";
import { useMangas } from "../hooks/useMangas";
import type { Manga } from "../types";
import { Button } from "@/shared/components/ui/button";

interface MangaTableProps {
    onEdit?: (manga: Manga) => void;
}

export function MangaTable({ onEdit }: MangaTableProps) {
    const { data, isLoading, isError, error } = useMangas();
    const deleteMangaMutation = useDeleteManga();

    const handleDelete = (manga: Manga) => {
        const ok = window.confirm(
            `"${manga.title}" を本当に削除しますか？\nこの操作は元に戻せません。`
        );
        if (!ok) return;

        deleteMangaMutation.mutate(manga.id, {
            onError: (error) => {
                console.error("漫画の削除に失敗しました:", error);
                // Todo: トーストなどでエラー表示したくなったらここ
            },
        });
    };

    if (isLoading) {
        return (
            <div className="p-4 text-sm text-muted-foreground">
                漫画一覧を読み込み中...
            </div>
        )
        // shadcn の Skeleton でローディング表示してもいい
    }

    if (isError) {
        return (
            <div className="space-y-2">
                <p className="text-sm text-destructive">
                    漫画一覧の取得に失敗しました。
                </p>
                <p className="text-xs text-muted-foreground">
                    {error instanceof Error ? error.message : String(error)}
                </p>
            </div>
        );
    }

    if (!data || data.length === 0) {
        return (
            <p className="text-sm text-muted-foreground">
                登録された漫画はまだありません。
            </p>
        );
    }

    return (
        <div className="overflow-hidden rounded-lg border border-border">
            <Table>
                <TableHeader className="bg-muted/60">
                    <TableRow>
                        <TableHead className="w-[22%]">タイトル</TableHead>
                        <TableHead className="w-[16%]">略称</TableHead>
                        <TableHead className="w-[12%]">ポータル</TableHead>
                        <TableHead className="w-[18%]">閲覧用URL</TableHead>
                        <TableHead className="w-[18%]">クロール用URL</TableHead>
                        <TableHead className="w-[18%] text-center">操作</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {data.map((manga) => (
                        <TableRow
                            key={manga.id}
                            className="hover:bg-muted/40 transition-colors"
                        >
                            <TableCell className="truncate text-sm">
                                {manga.title}
                            </TableCell>
                            <TableCell className="font-mono text-xs text-muted-foreground">
                                {manga.shortTitle}
                            </TableCell>
                            <TableCell className="text-xs">
                                {manga.portalKind}
                            </TableCell>
                            <TableCell className="text-xs">
                                <a
                                    href={manga.publicUrl}
                                    target="_blank"
                                    rel="noreferrer"
                                    className="text-blue-600 underline underline-offset-2"
                                >
                                    public
                                </a>
                            </TableCell>
                            <TableCell className="text-xs">
                                <a
                                href={manga.crawlUrl}
                                target="_blank"
                                rel="noreferrer"
                                className="text-blue-600 underline underline-offset-2"
                                >
                                crawl
                                </a>
                            </TableCell>
                            <TableCell className="flex justify-end gap-2">
                                <Button
                                    size="sm"
                                    variant="outline"
                                    className="h-7 px-2 text-xs"
                                    onClick={() => onEdit?.(manga)}
                                >
                                    編集
                                </Button>
                                <Button
                                    size="sm"
                                    variant="destructive"
                                    className="h-7 px-2 text-xs"
                                    onClick={() => handleDelete(manga)}
                                    disabled={deleteMangaMutation.isPending}
                                >
                                    削除
                                </Button>
                            </TableCell>
                            </TableRow>
                    ))}
                </TableBody>
            </Table>
        </div>
    )
}

/*
point
- useMangas で一覧取得
- ローディング／エラー／空状態を素直に分岐
- onEdit が渡されていれば、クリックで編集対象として渡す
- 削除はまず window.confirm で確認してから useDeleteManga を呼ぶ
（後で AlertDialog に差し替えるのは簡単）
*/