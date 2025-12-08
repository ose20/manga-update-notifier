import { http } from "@/shared/lib/httpClient";

export async function deleteManga(id: string): Promise<void> {
    await http(`/mangas/${id}`, {
        method: 'DELETE',
    })
}