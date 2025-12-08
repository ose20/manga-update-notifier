import { useQuery } from "@tanstack/react-query";
import { getMangas } from "../api/getMangas";
import type { Manga } from "../types";

export function useMangas() {
    return useQuery<Manga[], Error>({
        // queryKey でキャッシュを持ってくれる
        queryKey: ['mangas'] as const,
        queryFn: getMangas,
    });
}