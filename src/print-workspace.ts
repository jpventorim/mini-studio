import { invoke } from "@tauri-apps/api/core";

export const paperSizes = {
  a4: { label: "A4", width: 210, height: 297 },
  letter: { label: "Letter", width: 215.9, height: 279.4 },
} as const;

export type SheetSettings = {
  paper: keyof typeof paperSizes;
  dpi: number;
  marginMm: number;
  spacingMm: number;
  allowRotation: boolean;
};

export const defaultSettings: SheetSettings = {
  paper: "a4",
  dpi: 300,
  marginMm: 5,
  spacingMm: 2,
  allowRotation: true,
};

export type ImportedImage = {
  id: number;
  name: string;
  width: number;
  height: number;
  preview: string;
};

export type ImageSize = { width: number; height: number };

export type PrintImage = ImportedImage & {
  quantity: number;
  scale: number;
  customSize?: ImageSize;
  lockRatio: boolean;
};

export type PreviewPage = {
  widthMm: number;
  heightMm: number;
  placements: {
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

export function imageDimensions(image: PrintImage, dpi: number): ImageSize {
  if (image.customSize) return image.customSize;
  const factor = (25.4 * image.scale) / (100 * dpi);
  return { width: image.width * factor, height: image.height * factor };
}

export function printRequest(images: PrintImage[], settings: SheetSettings) {
  const page = pageDimensions(settings);
  return {
    widthMm: page.width,
    heightMm: page.height,
    dpi: settings.dpi,
    marginMm: settings.marginMm,
    spacingMm: settings.spacingMm,
    allowRotation: settings.allowRotation,
    items: images.map((image) => {
      const size = imageDimensions(image, settings.dpi);
      return {
        id: image.id,
        quantity: image.quantity,
        widthMm: size.width,
        heightMm: image.customSize && !image.lockRatio ? size.height : null,
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
    invoke<PreviewPage[]>("preview_layout", { request }),
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
