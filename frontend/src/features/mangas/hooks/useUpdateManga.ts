import { useMutation, useQueryClient } from "@tanstack/react-query";
import { updateManga } from "../api/updateManga";
import type { UpdateMangaInput } from "../types";

export function useUpdateManga() {
    const queryClient = useQueryClient();

    return useMutation<void, Error, UpdateMangaInput>({
        mutationFn: updateManga,
        onSuccess: () => {
            // 更新に成功したら一覧を最新化
            queryClient.invalidateQueries({ queryKey: ['mangas'] })
        }
    })
}