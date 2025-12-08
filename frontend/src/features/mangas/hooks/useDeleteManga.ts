import { useMutation, useQueryClient } from "@tanstack/react-query";
import { deleteManga } from "../api/deleteManga";

export function useDeleteManga() {
    const queryClient = useQueryClient();

    return useMutation<void, Error, string>({
        mutationFn: deleteManga,
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['mangas'] });
        },
    });
}