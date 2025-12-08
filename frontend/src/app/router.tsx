import { Routes, Route } from "react-router-dom";
import { MangaPageContent } from "@/features/mangas/components/MangaPageContent";

export function AppRouter() {
    return (
        <Routes>
            <Route path="/" element={<MangaPageContent />} />
        </Routes>
    )
}