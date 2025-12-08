export const PORTAL_KINDS = [
  'KimiComi',
  'KadoComi',
  'TonarinoYJ',
  'HerosWeb',
  'JumpPlus',
  'YoungMagazine',
  'ComicDays',
  'ComicFuz',
  'ComicZenon',
] as const;

export type PortalKind = (typeof PORTAL_KINDS)[number];

export interface Manga {
  id: string;
  title: string;
  shortTitle: string;
  publicUrl: string;
  crawlUrl: string;
  portalKind: PortalKind;
}

export type CreateMangaInput = Omit<Manga, 'id'>;

export interface UpdateMangaInput extends CreateMangaInput {
  id: Manga['id'];
}
