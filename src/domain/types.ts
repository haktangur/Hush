/** Hush v1 IPC reference types. Validate every input again in Rust. */
export type UUID = string;
export type ISODate = string; // YYYY-MM-DD, real calendar day
export type ISOInstant = string; // UTC RFC3339, milliseconds + Z
export type ItemKind = 'game' | 'album' | 'book';
export type Status = 'planned' | 'active' | 'completed' | 'paused' | 'dropped';
export type AlbumStatus = 'planned' | 'active' | 'completed';
export type Provider = 'open_library' | 'musicbrainz' | 'igdb';
export type Mood = 'calm' | 'happy' | 'focused' | 'tired' | 'melancholic' | 'excited';

export type ErrorCode =
  | 'VALIDATION' | 'NOT_FOUND' | 'CONFLICT' | 'DB_BUSY' | 'DISK_FULL' | 'IO'
  | 'TIMEOUT' | 'PROVIDER_UNAUTHORIZED' | 'PROVIDER_FORBIDDEN' | 'PROVIDER_NOT_FOUND' | 'PROVIDER_NOT_CONFIGURED' | 'OFFLINE' | 'RATE_LIMITED' | 'PROVIDER_UNAVAILABLE'
  | 'INVALID_BACKUP' | 'UNSUPPORTED_SCHEMA' | 'UNAVAILABLE' | 'INTERNAL';
export interface AppError {
 retryAfterSeconds?:number;
  code: ErrorCode;
  message: string;
  fieldErrors?: Record<string, string>;
  retryable: boolean;
  correlationId?: string;
}
export type ApiResult<T> = { ok: true; value: T } | { ok: false; error: AppError };
export interface Page<T> { items: T[]; total: number; offset: number; limit: number }
export interface Persisted { revision: number; createdAt: ISOInstant; updatedAt: ISOInstant }

export interface Provenance {
  provider: Provider;
  entityType: 'game' | 'release_group' | 'work' | 'edition';
  externalId: string;
  editionId: string | null;
  sourceUrl: string;
  fetchedAt: ISOInstant;
  attribution: string;
}
export interface CommonItemDraft {
  id: UUID;
  title: string;
  creator: string;
  releaseYear: number | null;
  coverAssetId: UUID | null;
  rating: number | null;
  favorite: boolean;
  notes: string;
  tagNames: string[];
  startedOn: ISODate | null;
  completedOn: ISODate | null;
  provenance: Provenance | null;
}
export interface GameDraft extends CommonItemDraft {
  kind: 'game'; status: Status;
  details: { platform: string; developer: string; progressPercent: number | null; returnNote: string };
}
export interface AlbumDraft extends CommonItemDraft {
  kind: 'album'; status: AlbumStatus;
  details: { firstListenedOn: ISODate | null; spotifyUrl: string | null; appleMusicUrl: string | null };
}
export interface BookDraft extends CommonItemDraft {
  kind: 'book'; status: Status;
  details: { isbn: string | null; publisher: string; language: string; totalPages: number | null; currentPage: number };
}
export type ItemDraft = GameDraft | AlbumDraft | BookDraft;
export type LibraryItem = ItemDraft & Persisted & { deletedAt: ISOInstant | null };
export interface ItemSummary {
  id: UUID; kind: ItemKind; title: string; creator: string; status: Status;
  coverAssetId: UUID | null; rating: number | null; favorite: boolean; updatedAt: ISOInstant;
}
export interface LibraryFilter {
  kind?: ItemKind; query?: string; status?: Status; tags?: string[];
  collectionId?: UUID; favorite?: boolean;
  sort: 'title_asc' | 'created_desc' | 'rating_desc' | 'updated_desc';
  offset: number; limit: number;
}
export interface SaveItemRequest {
  item: ItemDraft;
  expectedRevision: number | null;
  // Only for a new album from catalog import; committed with the item.
  initialTracks?: AlbumTrack[];
}

export interface SessionDraft { id: UUID; itemId: UUID; occurredAt: ISOInstant; durationMinutes: number; notes: string }
export type Session = SessionDraft & Persisted;
export interface AlbumTrack {
  id: UUID; discNumber: number; trackNumber: number; title: string;
  durationSeconds: number | null; favorite: boolean; notes: string;
}
export interface BookQuote {
  id: UUID; body: string; pageNumber: number | null; comment: string;
  createdAt: ISOInstant; updatedAt: ISOInstant;
}
export interface GameScreenshot { id: UUID; assetId: UUID; caption: string; sortOrder: number; createdAt: ISOInstant }
export interface CollectionDraft { id: UUID; name: string; description: string }
export type Collection = CollectionDraft & Persisted & { itemIds: UUID[] };
export interface JournalDraft { id: UUID; title: string; entryDate: ISODate; mood: Mood | null; body: string; itemIds: UUID[] }
export type JournalEntry = JournalDraft & Persisted & { deletedAt: ISOInstant | null };
export interface JournalEditorDraft { entry: JournalDraft; expectedRevision: number | null }

export interface Asset {
  id: UUID; relativePath: string; thumbnailPath: string; sha256: string;
  mimeType: 'image/webp'; byteSize: number; width: number; height: number; createdAt: ISOInstant;
}
export type SlotId = 'poster_left' | 'poster_right' | 'record_stand';
export type SlotSource =
  | { type: 'default' }
  | { type: 'item'; itemId: UUID }
  | { type: 'image'; assetId: UUID };
export type RoomSlots = Record<SlotId, SlotSource>;
export interface Settings {
  schemaVersion: 1;
  locale: 'tr';
  room: {
    preset: 'cozy_night' | 'violet_rain' | 'quiet_dawn';
    neonIntensity: number; // 0..100
    lampOn: boolean; rainOn: boolean; motion: 'system' | 'reduced' | 'full'; labelsVisible: boolean;
  };
  catalog: { enabled: boolean; contact: string | null };
  onboardingCompleted: boolean;
}
export const DEFAULT_SETTINGS: Settings = {
  schemaVersion: 1, locale: 'tr',
  room: { preset: 'cozy_night', neonIntensity: 60, lampOn: true, rainOn: true, motion: 'system', labelsVisible: true },
  catalog: { enabled: true, contact: null }, onboardingCompleted: false,
};
export interface SettingsSnapshot { value: Settings; revision: number }
export interface ProviderState {
  provider: Provider; state: 'ready' | 'not_configured' | 'disabled' | 'error'; message: string | null;
}
export interface CandidateKey { provider: Provider; entityType: Provenance['entityType']; externalId: string }
export interface CatalogCandidate {
  key: CandidateKey; title: string; creator: string; year: number | null;
  providerPageUrl: string; coverAvailable: boolean; existingItemId: UUID | null;
  existingState: 'active' | 'trashed' | null;
}
export interface CatalogPage { items: CatalogCandidate[]; page: number; hasMore: boolean; fromCache: boolean }
export interface CatalogDraft {
 existingItemId:string|null;existingState:string|null;
  item: ItemDraft; tracks: AlbumTrack[]; coverLeaseId: string | null; warnings: string[];
}
export interface BackupManifest {
  formatVersion: 1; schemaVersion: number; appVersion: string; createdAt: ISOInstant;
  files: { path: string; sha256: string; bytes: number }[];
}
export interface RestorePreview {
  inspectionToken: string; createdAt: ISOInstant; appVersion: string;
  counts: { items: number; journals: number; assets: number }; replacesCurrentData: true;
}
export interface RestoreResult { storageEpoch: string; restoredAt: ISOInstant }

export interface MetadataDiff {token:string;fields:{field:string;before:unknown;after:unknown}[];warnings:string[]}
