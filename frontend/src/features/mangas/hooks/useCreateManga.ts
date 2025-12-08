import { useMutation, useQueryClient } from "@tanstack/react-query";
import { createManga } from "../api/createManga";
import type { CreateMangaInput } from "../types";

export function useCreateManga() {
    const queryClient = useQueryClient();

    return useMutation<void, Error, CreateMangaInput>({
        mutationFn: createManga,
        onSuccess: () => {
            // 作成に成功したら一覧を最新化
            queryClient.invalidateQueries({ queryKey: ['mangas'] })
        }
    })
}