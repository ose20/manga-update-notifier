import { useForm, Controller } from "react-hook-form";
import type { CreateMangaInput, Manga, PortalKind, UpdateMangaInput } from "../types";
import { PORTAL_KINDS } from "../types";
import { useCreateManga } from "../hooks/useCreateManga";
import { useUpdateManga } from "../hooks/useUpdateManga";
import { useEffect } from "react";
import { Input } from "@/shared/components/ui/input";
import { Label } from "@/shared/components/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/components/ui/select";
import { Button } from "@/shared/components/ui/button";

type FormValues = Omit<CreateMangaInput, 'portalKind'> & { portalKind: PortalKind | '' };

interface MangaFormProps {
    mode: 'create' | 'edit';
    initialValue?: Manga;
    onSubmitted?: () => void;
    onCancelEdit?: () => void;
}

function toCreateInput(values: FormValues): CreateMangaInput | null {
    if (!values.portalKind) {
        return null;
    }

    const { portalKind, ...rest } = values;
    return {
        ...rest, 
        portalKind,
    };
}

function toUpdateInput(values: FormValues, id: Manga['id']): UpdateMangaInput | null {
    const base = toCreateInput(values);
    if (!base) return null;

    return { id, ...base };
}

export function MangaForm({
    mode,
    initialValue,
    onSubmitted,
    onCancelEdit,
}: MangaFormProps) {
    const {
        register,
        handleSubmit,
        reset,
        control,
        setError,
        formState: { isSubmitting, errors },
    } = useForm<FormValues>({
        defaultValues: {
            title: '',
            shortTitle: '',
            publicUrl: '',
            crawlUrl: '',
            portalKind: '',
        },
    });

    const createMangaMutation = useCreateManga();
    const updateMangaMutation = useUpdateManga();

    // 編集対象が変わったらフォームの値をリセット
    useEffect(() => {
        if (mode === 'edit' && initialValue) {
            reset({
                title: initialValue.title,
                shortTitle: initialValue.shortTitle,
                publicUrl: initialValue.publicUrl,
                crawlUrl: initialValue.crawlUrl,
                portalKind: initialValue.portalKind,
            });
        }
        if (mode === 'create') {
            reset({
                title: '',
                shortTitle: '',
                publicUrl: '',
                crawlUrl: '',
                portalKind: '',
            });
        }
    }, [mode, initialValue, reset]);

    const onSubmit = async (values: FormValues) => {
        try {
            if (mode === 'create') {
                const input = toCreateInput(values); // CreateMangaInput | null
                if (!input) {
                    setError('portalKind', {
                        type: 'required',
                        message: 'ポータルサイトを選択してください',
                    });
                    return;
                }

                await createMangaMutation.mutateAsync(input);
            }

            if (mode === 'edit' && initialValue) {
                const input = toUpdateInput(values, initialValue.id); // UpdateMangaInput | null
                if (!input) {
                    setError('portalKind', {
                        type: 'required',
                        message: 'ポータルサイトを選択してください',
                    });
                    return;
                }

                await updateMangaMutation.mutateAsync(input);
            }

            onSubmitted?.();
        } catch (e) {
            console.error("Failed", e);
        }
    };


    const isMutating =
        isSubmitting || createMangaMutation.isPending || updateMangaMutation.isPending;

    return (
        <form className="space-y-4" onSubmit={handleSubmit(onSubmit)}>
            <div className="space-y-2">
                <Label htmlFor="title">タイトル</Label>
                <Input
                    id="title"
                    placeholder="例: ふつうの軽音部"
                    {...register("title", { required: true })}
                />
            </div>

            <div className="space-y-2">
                <Label htmlFor="shortTitle">略称タイトル</Label>
                <Input
                    id="shortTitle"
                    placeholder="例: futsu-no-keionbu"
                    {...register("shortTitle", { required: true })}
                />
            </div>

            <div className="space-y-2">
                <Label htmlFor="publicUrl">閲覧用URL</Label>
                <Input
                    id="publicUrl"
                    placeholder="例: https://shonenjumpplus.com/episode/17107094912394187229"
                    {...register("publicUrl", { required: true })}
                ></Input>
            </div>

            <div className="space-y-2">
                <Label htmlFor="crawlUrl">クロール用URL</Label>
                <Input
                    id="crawlUrl"
                    placeholder="例: https://shonenjumpplus.com/rss/series/14079602755590623793"
                    {...register("crawlUrl", { required: true })}
                ></Input>
            </div>

            <div className="space-y-2">
                <Label>ポータルサイト</Label>
                <Controller
                    name="portalKind"
                    control={control}
                    rules={{ required: 'ポータルサイトを選択してください' }}
                    render={({ field }) => (
                        <Select
                            onValueChange={field.onChange}
                            defaultValue={field.value}
                        >
                            <SelectTrigger>
                                <SelectValue placeholder="ポータルを選択"></SelectValue>
                            </SelectTrigger>
                            <SelectContent>
                                {PORTAL_KINDS.map((kind) => (
                                    <SelectItem key={kind} value={kind}>
                                        {kind}
                                    </SelectItem>
                                ))}
                            </SelectContent>

                        </Select>
                    )}
                />
                {errors.portalKind && (
                    <p className="text-xs text-destructive">
                        {errors.portalKind.message}
                    </p>
                )}

            </div>

            <div className="flex gap-2 justify-end">
                {mode === 'edit' && (
                    <Button
                        type="button" 
                        variant="outline"
                        onClick={() => {
                            onCancelEdit?.();
                        }}
                    >
                        編集をキャンセル
                    </Button>
                )}
                <Button type="submit" disabled={isMutating}>
                    {mode === 'create' ? '登録' : '更新'}
                </Button>
            </div>
        </form>
    )
}


/* 
point
- mode と initialValue で「新規 or 編集」を切り替える
- useEffect で initialValue の変更を検知し、フォームを reset
- portalKind だけ Select で扱うため watch + setValue を使っている
- ミューテーションは mutateAsync を使って async/await で処理
*/