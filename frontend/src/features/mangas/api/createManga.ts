import { http } from "@/shared/lib/httpClient";
import type { CreateMangaInput } from "../types";

export async function createManga(input: CreateMangaInput): Promise<void> {
    return http('/mangas', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(input),
    });
}