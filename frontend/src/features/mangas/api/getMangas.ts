import { http } from "@/shared/lib/httpClient";
import type { Manga } from "../types";

export async function getMangas(): Promise<Manga[]> {
    return http<Manga[]>('/mangas');
}