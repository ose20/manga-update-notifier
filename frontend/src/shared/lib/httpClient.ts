import { API_BASE_URL } from "./env";

export async function http(path: string, init?: RequestInit): Promise<void>;

export async function http<T>(path: string, init?: RequestInit): Promise<T>;

export async function http<T>(
    path: string,
    init? : RequestInit
): Promise<T | void> {
    const res = await fetch(`${API_BASE_URL}${path}`, init);

    if (!res.ok) {
        throw new Error(`HTTP error! status: ${res.status}`);
    }

    if (res.status == 204 || res.headers.get('Content-Length') === '0') {
        return;
    }

    return (await res.json()) as T;
}