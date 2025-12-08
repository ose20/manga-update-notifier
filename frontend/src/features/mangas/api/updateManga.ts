import { http } from "@/shared/lib/httpClient";
import type { UpdateMangaInput } from "../types";

export async function updateManga(input: UpdateMangaInput): Promise<void> {
    const {id, ...payload} = input;
    return http(`/mangas/${id}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
    });
}