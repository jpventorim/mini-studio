import { invoke } from "@tauri-apps/api/core";

export const paperSizes = {
  a2: { label: "A2", width: 420, height: 594 },
  a3: { label: "A3", width: 297, height: 420 },
  a4: { label: "A4", width: 210, height: 297 },
  letter: { label: "Letter", width: 215.9, height: 279.4 },
} as const;

export type SheetSettings = {
  paper: keyof typeof paperSizes;
  dpi: number;
  marginMm: number;
  spacingMm: number;
  gridSizeMm: 20 | 25;
};

export function isPaperSize(value: unknown): value is SheetSettings["paper"] {
  return typeof value === "string" && Object.keys(paperSizes).includes(value);
}

export const defaultSettings: SheetSettings = {
  paper: "a4",
  dpi: 300,
  marginMm: 2,
  spacingMm: 0,
  gridSizeMm: 25,
};

export type ImportedImage = {
  id: number;
  name: string;
  width: number;
  height: number;
  preview: string;
};

export type ImageSize = { width: number; height: number };

export const standeeSizes = [0.25, 0.5, 1, 2, 3, 4, 5] as const;
export type StandeeSize = (typeof standeeSizes)[number];
export function isStandeeSize(value: number): value is StandeeSize {
  return standeeSizes.some((size) => size === value);
}

export type PrintImage = ImportedImage & {
  quantity: number;
  size: StandeeSize;
  numberCopies: boolean;
  borderPx: number;
  baseClearanceMm: number | null;
  artworkWidthRatio: number;
  trim: "auto" | "alpha" | "color" | "none";
  backgroundColor: string;
  alphaThreshold: number;
  colorTolerance: number;
  allowRotation: boolean;
};

export const defaultImageSettings = {
  quantity: 1,
  size: 1,
  numberCopies: false,
  borderPx: 3,
  baseClearanceMm: null,
  artworkWidthRatio: 0.9,
  trim: "auto",
  backgroundColor: "#ffffff",
  alphaThreshold: 0,
  colorTolerance: 8,
  allowRotation: true,
} satisfies Omit<PrintImage, keyof ImportedImage>;

export type PreparedPreviewImage = {
  imageId: number;
  preview: string;
  widthMm: number;
  heightMm: number;
  copyNumber: number | null;
  baseWidthMm: number;
  baseDepthMm: number;
};

export type PreparedLayout = {
  pages: PreviewPage[];
  images: PreparedPreviewImage[];
};

export type PreviewPage = {
  widthMm: number;
  heightMm: number;
  placements: {
    sourceIndex: number;
    imageId: number;
    copyIndex: number;
    xMm: number;
    yMm: number;
    widthMm: number;
    heightMm: number;
    rotated: boolean;
  }[];
};

export type PrintError = { message: string; imageId?: number | null };

type ImportResult = { images: ImportedImage[]; errors: string[] };

export const millimetres = new Intl.NumberFormat(undefined, {
  maximumFractionDigits: 2,
});

export function pageDimensions({ paper }: SheetSettings): ImageSize {
  const { width, height } = paperSizes[paper];
  return { width, height };
}

export function printRequest(images: PrintImage[], settings: SheetSettings) {
  const page = pageDimensions(settings);
  return {
    widthMm: page.width,
    heightMm: page.height,
    dpi: settings.dpi,
    marginMm: settings.marginMm,
    spacingMm: settings.spacingMm,
    gridSizeMm: settings.gridSizeMm,
    items: images.map((image) => {
      return {
        id: image.id,
        quantity: image.quantity,
        allowRotation: image.allowRotation,
        size: image.size,
        numberCopies: image.numberCopies,
        borderPx: image.borderPx,
        baseClearanceMm: image.baseClearanceMm,
        artworkWidthRatio: image.artworkWidthRatio,
        trim: image.trim,
        backgroundColor: [1, 3, 5].map((offset) =>
          Number.parseInt(image.backgroundColor.slice(offset, offset + 2), 16),
        ),
        alphaThreshold: image.alphaThreshold,
        colorTolerance: image.colorTolerance,
      };
    }),
  };
}

export type PrintRequest = ReturnType<typeof printRequest>;

export const desktop = {
  chooseImages: () => invoke<ImportResult>("choose_images"),
  importImages: (paths: string[]) =>
    invoke<ImportResult>("import_images", { paths }),
  removeImage: (id: number) => invoke<void>("remove_image", { id }),
  preview: (request: PrintRequest) =>
    invoke<PreparedLayout>("preview_layout", { request }),
  export: (request: PrintRequest) =>
    invoke<string | null>("export_pdf", { request }),
};

export function printError(error: unknown): PrintError {
  if (
    error &&
    typeof error === "object" &&
    "message" in error &&
    typeof error.message === "string"
  ) {
    return {
      message: error.message,
      imageId:
        "imageId" in error && typeof error.imageId === "number"
          ? error.imageId
          : undefined,
    };
  }
  return {
    message:
      typeof error === "string" ? error : "The print task could not finish.",
  };
}
