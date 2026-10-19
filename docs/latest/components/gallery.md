---
title: "Angular Gallery Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/gallery.md"
description: "Gallery is an image viewer with zoom, rotate, flip and download capabilities."
---

# Angular Gallery Component

Gallery is an image viewer with zoom, rotate, flip and download capabilities.

## Accessibility

Screen Reader Gallery uses semantic button elements for all interactive controls. The data-scope and data-part attributes identify each part of the gallery for assistive technologies. Use aria-label attributes on buttons to provide accessible names.

## Basic

Displays a collection of images with a lightbox viewer.

```typescript
import { Component } from '@angular/core';
import { GalleryModule } from 'primeng/gallery';
import { ChevronLeft } from '@primeicons/angular/chevron-left';
import { ChevronRight } from '@primeicons/angular/chevron-right';

@Component({
    template: `
        <p-gallery class="w-full not-data-fullscreen:h-150!">
            <p-gallery-backdrop></p-gallery-backdrop>
            <button pGalleryPrev>
                <svg data-p-icon="chevron-left"></svg>
            </button>
            <button pGalleryNext>
                <svg data-p-icon="chevron-right"></svg>
            </button>
            <p-gallery-content>
                @for (image of images; track image) {
                    <p-gallery-item>
                        <img [src]="image" alt="image" />
                    </p-gallery-item>
                }
            </p-gallery-content>
        </p-gallery>
    `,
    standalone: true,
    imports: [GalleryModule, ChevronLeft, ChevronRight]
})
export class GalleryBasicDemo {
    photos: [number, number, number][] = [
        [10, 1200, 800],
        [11, 800, 1200],
        [15, 1400, 700],
        [16, 700, 1050],
        [17, 1000, 1000]
    ];

    images = this.photos.map(([id, w, h]) => `https://picsum.photos/id/${id}/${w}/${h}`);
}
```

## Grid

Gallery can be used as a lightbox by combining it with a grid of thumbnails. Click on an image to open the gallery in fullscreen overlay.

```typescript
import { Component, signal } from '@angular/core';
import { GalleryModule } from 'primeng/gallery';
import { GalleryActiveIndexChangeEvent } from 'primeng/types/gallery';
import { ChevronLeft } from '@primeicons/angular/chevron-left';
import { ChevronRight } from '@primeicons/angular/chevron-right';
import { Replay } from '@primeicons/angular/replay';
import { Refresh } from '@primeicons/angular/refresh';
import { SearchPlus } from '@primeicons/angular/search-plus';
import { SearchMinus } from '@primeicons/angular/search-minus';
import { ArrowsH } from '@primeicons/angular/arrows-h';
import { ArrowsV } from '@primeicons/angular/arrows-v';
import { Download } from '@primeicons/angular/download';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <div class="grid grid-cols-2 md:grid-cols-4 2xl:grid-cols-6 gap-2">
            @for (image of images; track image; let i = $index) {
                <div class="aspect-square cursor-pointer hover:opacity-75 transition-opacity" (click)="handleOpen(i)">
                    <img [src]="image" alt="image" class="w-full h-full object-cover rounded-lg" />
                </div>
            }
        </div>
        @if (open()) {
            <p-gallery [fullscreen]="true" (onFullscreenChange)="open.set($event)" [(activeIndex)]="activeIndex" (onActiveIndexChange)="onActiveIndexChange($event)" class="opacity-100 transition-opacity duration-200">
                <p-gallery-backdrop></p-gallery-backdrop>
                <button pGalleryPrev>
                    <svg data-p-icon="chevron-left"></svg>
                </button>
                <button pGalleryNext>
                    <svg data-p-icon="chevron-right"></svg>
                </button>
                <p-gallery-header class="justify-end gap-0.5">
                    <button pGalleryRotateLeft>
                        <svg data-p-icon="replay"></svg>
                    </button>
                    <button pGalleryRotateRight>
                        <svg data-p-icon="refresh"></svg>
                    </button>
                    <button pGalleryZoomIn>
                        <svg data-p-icon="search-plus"></svg>
                    </button>
                    <button pGalleryZoomOut>
                        <svg data-p-icon="search-minus"></svg>
                    </button>
                    <button pGalleryFlipX>
                        <svg data-p-icon="arrows-h"></svg>
                    </button>
                    <button pGalleryFlipY>
                        <svg data-p-icon="arrows-v"></svg>
                    </button>
                    <button pGalleryDownload>
                        <svg data-p-icon="download"></svg>
                    </button>
                    <button class="p-gallery-action" (click)="close()">
                        <svg data-p-icon="times"></svg>
                    </button>
                </p-gallery-header>
                <p-gallery-content>
                    @for (image of images; track image) {
                        <p-gallery-item>
                            <img [src]="image" alt="image" class="scale-100 blur-none transition-[scale,filter] duration-300" />
                        </p-gallery-item>
                    }
                </p-gallery-content>
                <p-gallery-footer>
                    <p-gallery-thumbnail>
                        <p-gallery-thumbnail-content>
                            @for (image of images; track image; let i = $index) {
                                <p-gallery-thumbnail-item [index]="i">
                                    <img [attr.draggable]="false" [src]="image" class="h-full w-full object-cover" />
                                </p-gallery-thumbnail-item>
                            }
                        </p-gallery-thumbnail-content>
                    </p-gallery-thumbnail>
                </p-gallery-footer>
            </p-gallery>
        }
    `,
    standalone: true,
    imports: [GalleryModule, ChevronLeft, ChevronRight, Replay, Refresh, SearchPlus, SearchMinus, ArrowsH, ArrowsV, Download, Times]
})
export class GalleryGridDemo {
    photos: [number, number, number][] = [
        [10, 1200, 800],
        [11, 800, 1200],
        [15, 1400, 700],
        [16, 700, 1050],
        [17, 1000, 1000],
        [18, 1300, 650],
        [19, 600, 1200],
        [20, 1200, 900],
        [27, 750, 1125],
        [28, 1400, 800],
        [29, 800, 1100],
        [36, 1100, 700],
        [37, 650, 1300],
        [39, 1200, 750],
        [42, 900, 1200],
        [43, 1300, 800],
        [47, 700, 1400],
        [48, 1000, 800],
        [49, 800, 1000],
        [50, 1400, 600],
        [52, 600, 900],
        [53, 1200, 1200],
        [54, 900, 600],
        [55, 750, 1000],
        [56, 1100, 800],
        [57, 1400, 900],
        [58, 850, 1275],
        [59, 1000, 600],
        [60, 600, 1000],
        [64, 1300, 1300]
    ];

    images = this.photos.map(([id, w, h]) => `https://picsum.photos/id/${id}/${w}/${h}`);

    activeIndex: number = 0;

    open = signal(false);

    handleOpen(index: number) {
        this.activeIndex = index;
        this.open.set(true);
    }

    close() {
        this.open.set(false);
    }

    onActiveIndexChange(event: GalleryActiveIndexChangeEvent) {
        this.activeIndex = event.value;
    }
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { GalleryModule } from 'primeng/gallery';
import { ChevronLeft } from '@primeicons/angular/chevron-left';
import { ChevronRight } from '@primeicons/angular/chevron-right';
import { Replay } from '@primeicons/angular/replay';
import { Refresh } from '@primeicons/angular/refresh';
import { SearchPlus } from '@primeicons/angular/search-plus';
import { SearchMinus } from '@primeicons/angular/search-minus';
import { ArrowsH } from '@primeicons/angular/arrows-h';
import { ArrowsV } from '@primeicons/angular/arrows-v';
import { Download } from '@primeicons/angular/download';
import { ArrowUpRightAndArrowDownLeftFromCenter } from '@primeicons/angular/arrow-up-right-and-arrow-down-left-from-center';
import { ArrowDownLeftAndArrowUpRightToCenter } from '@primeicons/angular/arrow-down-left-and-arrow-up-right-to-center';

@Component({
    template: `
        <p-gallery class="not-data-fullscreen:h-150!">
            <p-gallery-backdrop></p-gallery-backdrop>
            <button pGalleryPrev>
                <svg data-p-icon="chevron-left"></svg>
            </button>
            <button pGalleryNext>
                <svg data-p-icon="chevron-right"></svg>
            </button>
            <p-gallery-header class="justify-end gap-0.5">
                <button pGalleryRotateLeft>
                    <svg data-p-icon="replay"></svg>
                </button>
                <button pGalleryRotateRight>
                    <svg data-p-icon="refresh"></svg>
                </button>
                <button pGalleryZoomIn>
                    <svg data-p-icon="search-plus"></svg>
                </button>
                <button pGalleryZoomOut>
                    <svg data-p-icon="search-minus"></svg>
                </button>
                <button pGalleryFlipX>
                    <svg data-p-icon="arrows-h"></svg>
                </button>
                <button pGalleryFlipY>
                    <svg data-p-icon="arrows-v"></svg>
                </button>
                <button pGalleryDownload>
                    <svg data-p-icon="download"></svg>
                </button>
                <button pGalleryFullScreen class="group">
                    <svg data-p-icon="arrow-up-right-and-arrow-down-left-from-center" class="group-data-fullscreen:hidden!"></svg>
                    <svg data-p-icon="arrow-down-left-and-arrow-up-right-to-center" class="hidden! group-data-fullscreen:block!"></svg>
                </button>
            </p-gallery-header>
            <p-gallery-content>
                @for (image of images; track image) {
                    <p-gallery-item>
                        <img [src]="image" alt="image" />
                    </p-gallery-item>
                }
            </p-gallery-content>
            <p-gallery-footer>
                <p-gallery-thumbnail>
                    <p-gallery-thumbnail-content>
                        @for (image of images; track image; let i = $index) {
                            <p-gallery-thumbnail-item [index]="i">
                                <img [attr.draggable]="false" [src]="image" class="h-full w-full object-cover" />
                            </p-gallery-thumbnail-item>
                        }
                    </p-gallery-thumbnail-content>
                </p-gallery-thumbnail>
            </p-gallery-footer>
        </p-gallery>
    `,
    standalone: true,
    imports: [GalleryModule, ChevronLeft, ChevronRight, Replay, Refresh, SearchPlus, SearchMinus, ArrowsH, ArrowsV, Download, ArrowUpRightAndArrowDownLeftFromCenter, ArrowDownLeftAndArrowUpRightToCenter]
})
export class GalleryPreviewDemo {
    photos: [number, number, number][] = [
        [10, 1200, 800],
        [11, 800, 1200],
        [15, 1400, 700],
        [16, 700, 1050],
        [17, 1000, 1000],
        [18, 1300, 650],
        [19, 600, 1200],
        [20, 1200, 900],
        [27, 750, 1125],
        [28, 1400, 800],
        [29, 800, 1100],
        [36, 1100, 700],
        [37, 650, 1300],
        [39, 1200, 750],
        [42, 900, 1200],
        [43, 1300, 800],
        [47, 700, 1400],
        [48, 1000, 800],
        [49, 800, 1000],
        [50, 1400, 600],
        [52, 600, 900],
        [53, 1200, 1200],
        [54, 900, 600],
        [55, 750, 1000],
        [56, 1100, 800],
        [57, 1400, 900],
        [58, 850, 1275],
        [59, 1000, 600],
        [60, 600, 1000],
        [64, 1300, 1300]
    ];

    images = this.photos.map(([id, w, h]) => `https://picsum.photos/id/${id}/${w}/${h}`);
}
```

## Single

Click on the image to open it in a fullscreen dialog without thumbnails or navigation buttons.

```typescript
import { Component, signal } from '@angular/core';
import { GalleryModule } from 'primeng/gallery';
import { Replay } from '@primeicons/angular/replay';
import { Refresh } from '@primeicons/angular/refresh';
import { SearchPlus } from '@primeicons/angular/search-plus';
import { SearchMinus } from '@primeicons/angular/search-minus';
import { ArrowsH } from '@primeicons/angular/arrows-h';
import { ArrowsV } from '@primeicons/angular/arrows-v';
import { Download } from '@primeicons/angular/download';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="w-80 aspect-3/2 cursor-pointer hover:opacity-75 transition-opacity" (click)="open.set(true)">
                <img [src]="image" alt="image" class="w-full h-full object-cover rounded-lg" />
            </div>
            @if (open()) {
                <p-gallery [fullscreen]="true" (onFullscreenChange)="open.set($event)">
                    <p-gallery-backdrop></p-gallery-backdrop>
                    <p-gallery-header class="justify-end gap-0.5">
                        <button pGalleryRotateLeft>
                            <svg data-p-icon="replay"></svg>
                        </button>
                        <button pGalleryRotateRight>
                            <svg data-p-icon="refresh"></svg>
                        </button>
                        <button pGalleryZoomIn>
                            <svg data-p-icon="search-plus"></svg>
                        </button>
                        <button pGalleryZoomOut>
                            <svg data-p-icon="search-minus"></svg>
                        </button>
                        <button pGalleryFlipX>
                            <svg data-p-icon="arrows-h"></svg>
                        </button>
                        <button pGalleryFlipY>
                            <svg data-p-icon="arrows-v"></svg>
                        </button>
                        <button pGalleryDownload>
                            <svg data-p-icon="download"></svg>
                        </button>
                        <button class="p-gallery-action" (click)="open.set(false)">
                            <svg data-p-icon="times"></svg>
                        </button>
                    </p-gallery-header>
                    <p-gallery-content>
                        <p-gallery-item>
                            <img [src]="image" alt="image" />
                        </p-gallery-item>
                    </p-gallery-content>
                </p-gallery>
            }
        </div>
    `,
    standalone: true,
    imports: [GalleryModule, Replay, Refresh, SearchPlus, SearchMinus, ArrowsH, ArrowsV, Download, Times]
})
export class GallerySingleDemo {
    image: string = 'https://picsum.photos/id/10/1200/800';

    open = signal(false);
}
```

## Thumbnails

Add a p-gallery-footer with p-gallery-thumbnail to render a thumbnail strip for quick navigation.

```typescript
import { Component } from '@angular/core';
import { GalleryModule } from 'primeng/gallery';
import { ChevronLeft } from '@primeicons/angular/chevron-left';
import { ChevronRight } from '@primeicons/angular/chevron-right';

@Component({
    template: `
        <p-gallery class="h-150!">
            <p-gallery-backdrop></p-gallery-backdrop>
            <button pGalleryPrev>
                <svg data-p-icon="chevron-left"></svg>
            </button>
            <button pGalleryNext>
                <svg data-p-icon="chevron-right"></svg>
            </button>
            <p-gallery-content>
                @for (image of images; track image) {
                    <p-gallery-item>
                        <img [src]="image" alt="image" />
                    </p-gallery-item>
                }
            </p-gallery-content>
            <p-gallery-footer>
                <p-gallery-thumbnail>
                    <p-gallery-thumbnail-content>
                        @for (image of images; track image; let i = $index) {
                            <p-gallery-thumbnail-item [index]="i">
                                <img [attr.draggable]="false" [src]="image" class="h-full w-full object-cover" />
                            </p-gallery-thumbnail-item>
                        }
                    </p-gallery-thumbnail-content>
                </p-gallery-thumbnail>
            </p-gallery-footer>
        </p-gallery>
    `,
    standalone: true,
    imports: [GalleryModule, ChevronLeft, ChevronRight]
})
export class GalleryThumbnailsDemo {
    photos: [number, number, number][] = [
        [10, 1200, 800],
        [11, 800, 1200],
        [15, 1400, 700],
        [16, 700, 1050],
        [17, 1000, 1000]
    ];

    images = this.photos.map(([id, w, h]) => `https://picsum.photos/id/${id}/${w}/${h}`);
}
```

## Toolbar

Add a p-gallery-header with action sub-components like pGalleryRotateLeft , pGalleryZoomIn , pGalleryDownload , and pGalleryFullScreen to expose image controls.

```typescript
import { Component } from '@angular/core';
import { GalleryModule } from 'primeng/gallery';
import { ChevronLeft } from '@primeicons/angular/chevron-left';
import { ChevronRight } from '@primeicons/angular/chevron-right';
import { Replay } from '@primeicons/angular/replay';
import { Refresh } from '@primeicons/angular/refresh';
import { SearchPlus } from '@primeicons/angular/search-plus';
import { SearchMinus } from '@primeicons/angular/search-minus';
import { ArrowsH } from '@primeicons/angular/arrows-h';
import { ArrowsV } from '@primeicons/angular/arrows-v';
import { Download } from '@primeicons/angular/download';
import { ArrowUpRightAndArrowDownLeftFromCenter } from '@primeicons/angular/arrow-up-right-and-arrow-down-left-from-center';
import { ArrowDownLeftAndArrowUpRightToCenter } from '@primeicons/angular/arrow-down-left-and-arrow-up-right-to-center';

@Component({
    template: `
        <p-gallery class="h-150!">
            <p-gallery-backdrop></p-gallery-backdrop>
            <button pGalleryPrev>
                <svg data-p-icon="chevron-left"></svg>
            </button>
            <button pGalleryNext>
                <svg data-p-icon="chevron-right"></svg>
            </button>
            <p-gallery-header class="justify-end gap-0.5">
                <button pGalleryRotateLeft>
                    <svg data-p-icon="replay"></svg>
                </button>
                <button pGalleryRotateRight>
                    <svg data-p-icon="refresh"></svg>
                </button>
                <button pGalleryZoomIn>
                    <svg data-p-icon="search-plus"></svg>
                </button>
                <button pGalleryZoomOut>
                    <svg data-p-icon="search-minus"></svg>
                </button>
                <button pGalleryFlipX>
                    <svg data-p-icon="arrows-h"></svg>
                </button>
                <button pGalleryFlipY>
                    <svg data-p-icon="arrows-v"></svg>
                </button>
                <button pGalleryDownload>
                    <svg data-p-icon="download"></svg>
                </button>
                <button pGalleryFullScreen class="group">
                    <svg data-p-icon="arrow-up-right-and-arrow-down-left-from-center" class="group-data-fullscreen:hidden!"></svg>
                    <svg data-p-icon="arrow-down-left-and-arrow-up-right-to-center" class="hidden! group-data-fullscreen:block!"></svg>
                </button>
            </p-gallery-header>
            <p-gallery-content>
                @for (image of images; track image) {
                    <p-gallery-item>
                        <img [src]="image" alt="image" />
                    </p-gallery-item>
                }
            </p-gallery-content>
        </p-gallery>
    `,
    standalone: true,
    imports: [GalleryModule, ChevronLeft, ChevronRight, Replay, Refresh, SearchPlus, SearchMinus, ArrowsH, ArrowsV, Download, ArrowUpRightAndArrowDownLeftFromCenter, ArrowDownLeftAndArrowUpRightToCenter]
})
export class GalleryToolbarDemo {
    photos: [number, number, number][] = [
        [10, 1200, 800],
        [11, 800, 1200],
        [15, 1400, 700],
        [16, 700, 1050],
        [17, 1000, 1000]
    ];

    images = this.photos.map(([id, w, h]) => `https://picsum.photos/id/${id}/${w}/${h}`);
}
```

## Gallery

Gallery is the main container component for the Gallery.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, GalleryPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| activeIndex | number | 0 | The index of the active item. |
| fullscreen | boolean \| undefined | - | When defined, puts the fullscreen state in controlled mode; the value drives whether the gallery is fullscreen and changes are reported via onFullscreenChange instead of being handled internally. |
| closeOnEscape | boolean | true | Whether pressing the Escape key exits fullscreen mode. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onActiveIndexChange | event: GalleryActiveIndexChangeEvent | Callback fired when the gallery's active index changes. |
| onFullscreenChange | value: boolean | Callback fired when the fullscreen state should change (controlled mode). |
| onZoomChange | value: number | Callback fired when the active item's zoom scale changes. |
| onRotateChange | value: number | Callback fired when the active item's rotation changes. |
| onFlipChange | value: { x: number; y: number } | Callback fired when the active item's flip state changes. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-gallery | Class name of the root element |
| p-gallery-backdrop | Class name of the backdrop element |
| p-gallery-header | Class name of the header element |
| p-gallery-footer | Class name of the footer element |
| p-gallery-content | Class name of the content element |
| p-gallery-item | Class name of the item element |
| p-gallery-next | Class name of the next button element |
| p-gallery-prev | Class name of the prev button element |
| p-gallery-toolbar | Class name of the toolbar element |
| p-gallery-toolbar-item | Class name of the toolbar item element |
| p-gallery-action | Class name of the action element |
| p-gallery-thumbnail | Class name of the thumbnail element |
| p-gallery-thumbnail-content | Class name of the thumbnail content element |
| p-gallery-thumbnail-item | Class name of the thumbnail item element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| gallery.backdrop.background | --p-gallery-backdrop-background | Background of backdrop |
| gallery.header.padding | --p-gallery-header-padding | Padding of header |
| gallery.header.background | --p-gallery-header-background | Background of header |
| gallery.footer.padding | --p-gallery-footer-padding | Padding of footer |
| gallery.footer.background | --p-gallery-footer-background | Background of footer |
| gallery.footer.border.color | --p-gallery-footer-border-color | Border color of footer |
| gallery.item.transition.duration | --p-gallery-item-transition-duration | Transition duration of item (transform and opacity) |
| gallery.action.size | --p-gallery-action-size | Size of action (shared by action / next / prev) |
| gallery.action.border.radius | --p-gallery-action-border-radius | Border radius of action |
| gallery.action.color | --p-gallery-action-color | Color of action |
| gallery.action.hover.background | --p-gallery-action-hover-background | Hover background of action |
| gallery.action.hover.color | --p-gallery-action-hover-color | Hover color of action |
| gallery.action.disabled.opacity | --p-gallery-action-disabled-opacity | Disabled opacity of action |
| gallery.action.transition.duration | --p-gallery-action-transition-duration | Transition duration of action |
| gallery.action.icon.size | --p-gallery-action-icon-size | Icon size of action |
| gallery.navigation.background | --p-gallery-navigation-background | Background of navigation (next / prev) |
| gallery.navigation.size | --p-gallery-navigation-size | Size of navigation |
| gallery.navigation.border.radius | --p-gallery-navigation-border-radius | Border radius of navigation |
| gallery.navigation.color | --p-gallery-navigation-color | Color of navigation |
| gallery.navigation.hover.background | --p-gallery-navigation-hover-background | Hover background of navigation |
| gallery.navigation.hover.color | --p-gallery-navigation-hover-color | Hover color of navigation |
| gallery.navigation.offset | --p-gallery-navigation-offset | Edge offset of navigation |
| gallery.navigation.transition.duration | --p-gallery-navigation-transition-duration | Transition duration of navigation |
| gallery.navigation.icon.size | --p-gallery-navigation-icon-size | Icon size of navigation |
| gallery.thumbnail.size | --p-gallery-thumbnail-size | Size of thumbnail item |
| gallery.thumbnail.padding | --p-gallery-thumbnail-padding | Padding of thumbnail item |
| gallery.thumbnail.background | --p-gallery-thumbnail-background | Background of thumbnail item |
| gallery.thumbnail.border.radius | --p-gallery-thumbnail-border-radius | Border radius of thumbnail item |
| gallery.thumbnail.border.width | --p-gallery-thumbnail-border-width | Border width of thumbnail item (outline width) |
| gallery.thumbnail.hover.border.color | --p-gallery-thumbnail-hover-border-color | Hover border color of thumbnail item |
| gallery.thumbnail.active.border.color | --p-gallery-thumbnail-active-border-color | Active border color of thumbnail item |
| gallery.thumbnail.active.scale | --p-gallery-thumbnail-active-scale | Active scale of thumbnail item |
| gallery.thumbnail.transition.duration | --p-gallery-thumbnail-transition-duration | Transition duration of thumbnail item |
| gallery.thumbnail.content.padding | --p-gallery-thumbnail-content-padding | Padding of thumbnail content |
