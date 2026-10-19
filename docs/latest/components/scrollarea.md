---
title: "Angular Scroll Area Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/scrollarea.md"
description: "ScrollArea is a cross browser, lightweight and themable alternative to native browser scrollbar."
---

# Angular Scroll Area Component

ScrollArea is a cross browser, lightweight and themable alternative to native browser scrollbar.

## Accessibility

ScrollArea uses a scrollbar role for the scrollbar elements with aria-orientation set to the scrollbar direction.

## Basic

A custom scrollable container with styled scrollbars.

```typescript
import { Component } from '@angular/core';

@Component({
    template: `
        <div class="max-w-56 w-full mx-auto">
            <div class="text-surface-500 dark:text-surface-400 font-medium text-sm uppercase font-mono tracking-tight mb-2">Automobiles</div>
            <p-scrollarea class="h-72">
                <p-scrollarea-viewport>
                    <p-scrollarea-content class="space-y-2">
                        @for (car of automobiles; track car.label) {
                            <span class="flex items-end gap-0.5">
                                <span class="text-sm hover:underline!">{{ car.label }}</span>
                                <span class="text-surface-500 text-[11px] font-mono mb-px tracking-tighter">({{ car.value }})</span>
                            </span>
                        }
                    </p-scrollarea-content>
                </p-scrollarea-viewport>
                <p-scrollarea-scrollbar orientation="vertical">
                    <p-scrollarea-handle />
                </p-scrollarea-scrollbar>
            </p-scrollarea>
        </div>
    `,
    standalone: true,
    imports: []
})
export class ScrollareaBasicDemo {
    automobiles = [
        { label: 'Audi', value: '200' },
        { label: 'BMW', value: '350' },
        { label: 'Mercedes-Benz', value: '420' },
        { label: 'Volkswagen', value: '510' },
        { label: 'Toyota', value: '610' },
        { label: 'Honda', value: '280' },
        { label: 'Hyundai', value: '390' },
        { label: 'Kia', value: '260' },
        { label: 'Ford', value: '470' },
        { label: 'Renault', value: '530' },
        { label: 'Peugeot', value: '410' },
        { label: 'Citroen', value: '190' },
        { label: 'Skoda', value: '360' },
        { label: 'Seat', value: '210' },
        { label: 'Opel', value: '440' },
        { label: 'Fiat', value: '620' },
        { label: 'Dacia', value: '380' },
        { label: 'Volvo', value: '140' },
        { label: 'Mazda', value: '120' },
        { label: 'Mitsubishi', value: '110' },
        { label: 'Nissan', value: '330' },
        { label: 'Subaru', value: '60' },
        { label: 'Suzuki', value: '170' },
        { label: 'Jeep', value: '95' },
        { label: 'Land Rover', value: '80' },
        { label: 'Porsche', value: '75' },
        { label: 'Ferrari', value: '12' },
        { label: 'Lamborghini', value: '9' },
        { label: 'Bentley', value: '15' },
        { label: 'Rolls-Royce', value: '6' },
        { label: 'Mini', value: '130' },
        { label: 'Tesla', value: '220' },
        { label: 'BYD', value: '55' },
        { label: 'Chery', value: '160' },
        { label: 'MG', value: '145' },
        { label: 'DS Automobiles', value: '40' },
        { label: 'Alfa Romeo', value: '50' },
        { label: 'Lancia', value: '8' },
        { label: 'Cadillac', value: '22' },
        { label: 'Chevrolet', value: '105' },
        { label: 'Dodge', value: '18' },
        { label: 'GMC', value: '14' },
        { label: 'Infiniti', value: '11' },
        { label: 'Acura', value: '7' },
        { label: 'Genesis', value: '35' },
        { label: 'Geely', value: '27' },
        { label: 'Proton', value: '19' },
        { label: 'Togg', value: '65' },
        { label: 'Rivian', value: '4' },
        { label: 'Lucid', value: '3' }
    ];
}
```

## Both Scrollbars

Both vertical and horizontal scrollbars are displayed when the content overflows in both directions.

```typescript
import { Component } from '@angular/core';

@Component({
    template: `
        <p-scrollarea class="h-80 max-w-sm mx-auto w-full rounded-md overflow-hidden">
            <p-scrollarea-viewport class="p-0!">
                <p-scrollarea-content>
                    <table class="min-w-xl w-full text-sm border-collapse border-0">
                        <thead class="sticky top-0 z-10">
                            <tr class="bg-surface-100 dark:bg-surface-800 border-b border-surface">
                                @for (col of columns; track col.key) {
                                    <th class="border-b border-surface px-4 py-2 text-left text-sm whitespace-nowrap uppercase font-mono font-light">{{ col.label }}</th>
                                }
                            </tr>
                        </thead>
                        <tbody>
                            @for (city of usCities; track $index) {
                                <tr class="last:border-0 border-b border-surface odd:bg-surface-50 dark:odd:bg-surface-900/40 hover:bg-primary/5">
                                    <td class="px-4 py-2 font-medium whitespace-nowrap">{{ city.city }}</td>
                                    <td class="px-4 py-2 text-surface-500">{{ city.state }}</td>
                                    <td class="px-4 py-2 tabular-nums">{{ city.population.toLocaleString() }}</td>
                                    <td class="px-4 py-2 tabular-nums">{{ city.area_km2.toLocaleString() }}</td>
                                </tr>
                            }
                        </tbody>
                    </table>
                </p-scrollarea-content>
            </p-scrollarea-viewport>
            <p-scrollarea-scrollbar orientation="vertical" class="m-0! bg-transparent!">
                <p-scrollarea-handle />
            </p-scrollarea-scrollbar>
            <p-scrollarea-scrollbar orientation="horizontal" class="m-0! bg-transparent!">
                <p-scrollarea-handle />
            </p-scrollarea-scrollbar>
            <p-scrollarea-corner />
        </p-scrollarea>
    `,
    standalone: true,
    imports: []
})
export class ScrollareaBothScrollbarsDemo {
    columns = [
        { key: 'city', label: 'City' },
        { key: 'state', label: 'State' },
        { key: 'population', label: 'Population' },
        { key: 'area_km2', label: 'Area (km²)' }
    ];

    usCities = [
        { city: 'New York', state: 'NY', population: 8419600, area_km2: 783.8 },
        { city: 'Los Angeles', state: 'CA', population: 3980400, area_km2: 1214.9 },
        { city: 'Chicago', state: 'IL', population: 2716000, area_km2: 589.6 },
        { city: 'Houston', state: 'TX', population: 2328000, area_km2: 1651.1 },
        { city: 'Phoenix', state: 'AZ', population: 1690000, area_km2: 1340.6 },
        { city: 'Philadelphia', state: 'PA', population: 1584200, area_km2: 369.6 },
        { city: 'San Antonio', state: 'TX', population: 1547200, area_km2: 1194.0 },
        { city: 'San Diego', state: 'CA', population: 1423800, area_km2: 964.5 },
        { city: 'Dallas', state: 'TX', population: 1341100, area_km2: 882.9 },
        { city: 'San Jose', state: 'CA', population: 1035300, area_km2: 469.7 },
        { city: 'Austin', state: 'TX', population: 1010000, area_km2: 704.0 },
        { city: 'Jacksonville', state: 'FL', population: 949600, area_km2: 2265.3 },
        { city: 'Fort Worth', state: 'TX', population: 918900, area_km2: 920.9 },
        { city: 'Columbus', state: 'OH', population: 905700, area_km2: 577.9 },
        { city: 'Charlotte', state: 'NC', population: 885700, area_km2: 771.0 },
        { city: 'San Francisco', state: 'CA', population: 873900, area_km2: 121.4 },
        { city: 'Indianapolis', state: 'IN', population: 876400, area_km2: 953.0 },
        { city: 'Seattle', state: 'WA', population: 737000, area_km2: 217.0 },
        { city: 'Denver', state: 'CO', population: 715500, area_km2: 401.3 },
        { city: 'Washington', state: 'DC', population: 689500, area_km2: 177.0 },
        { city: 'Boston', state: 'MA', population: 675600, area_km2: 232.1 },
        { city: 'El Paso', state: 'TX', population: 678800, area_km2: 663.7 },
        { city: 'Nashville', state: 'TN', population: 689400, area_km2: 1362.2 },
        { city: 'Detroit', state: 'MI', population: 639100, area_km2: 370.0 },
        { city: 'Oklahoma City', state: 'OK', population: 681100, area_km2: 1608.8 }
    ];
}
```

## Custom

Scrollbar and thumb elements can be customized using classes on the sub-components.

```typescript
import { Component } from '@angular/core';

@Component({
    template: `
        <div class="max-w-56 w-full mx-auto">
            <div class="text-surface-500 dark:text-surface-400 font-medium text-sm uppercase font-mono tracking-tight mb-2">Automobiles</div>
            <p-scrollarea class="h-72">
                <p-scrollarea-viewport>
                    <p-scrollarea-content class="space-y-2">
                        @for (car of automobiles; track car.label) {
                            <span class="flex items-end gap-0.5">
                                <span class="text-sm hover:underline!">{{ car.label }}</span>
                                <span class="text-surface-500 text-[11px] font-mono mb-px tracking-tighter">({{ car.value }})</span>
                            </span>
                        }
                    </p-scrollarea-content>
                </p-scrollarea-viewport>
                <p-scrollarea-scrollbar class="rounded-full! bg-indigo-500/10!">
                    <p-scrollarea-handle class="relative bg-transparent! after:content-[''] after:rounded-full after:bg-indigo-500 after:absolute after:inset-0 active:after:scale-y-90 active:after:scale-x-75 after:transition-transform" />
                </p-scrollarea-scrollbar>
            </p-scrollarea>
        </div>
    `,
    standalone: true,
    imports: []
})
export class ScrollareaCustomDemo {
    automobiles = [
        { label: 'Audi', value: '200' },
        { label: 'BMW', value: '350' },
        { label: 'Mercedes-Benz', value: '420' },
        { label: 'Volkswagen', value: '510' },
        { label: 'Toyota', value: '610' },
        { label: 'Honda', value: '280' },
        { label: 'Hyundai', value: '390' },
        { label: 'Kia', value: '260' },
        { label: 'Ford', value: '470' },
        { label: 'Renault', value: '530' },
        { label: 'Peugeot', value: '410' },
        { label: 'Citroen', value: '190' },
        { label: 'Skoda', value: '360' },
        { label: 'Seat', value: '210' },
        { label: 'Opel', value: '440' },
        { label: 'Fiat', value: '620' },
        { label: 'Dacia', value: '380' },
        { label: 'Volvo', value: '140' },
        { label: 'Mazda', value: '120' },
        { label: 'Mitsubishi', value: '110' },
        { label: 'Nissan', value: '330' },
        { label: 'Subaru', value: '60' },
        { label: 'Suzuki', value: '170' },
        { label: 'Jeep', value: '95' },
        { label: 'Land Rover', value: '80' },
        { label: 'Porsche', value: '75' },
        { label: 'Ferrari', value: '12' },
        { label: 'Lamborghini', value: '9' },
        { label: 'Bentley', value: '15' },
        { label: 'Rolls-Royce', value: '6' },
        { label: 'Mini', value: '130' },
        { label: 'Tesla', value: '220' },
        { label: 'BYD', value: '55' },
        { label: 'Chery', value: '160' },
        { label: 'MG', value: '145' },
        { label: 'DS Automobiles', value: '40' },
        { label: 'Alfa Romeo', value: '50' },
        { label: 'Lancia', value: '8' },
        { label: 'Cadillac', value: '22' },
        { label: 'Chevrolet', value: '105' },
        { label: 'Dodge', value: '18' },
        { label: 'GMC', value: '14' },
        { label: 'Infiniti', value: '11' },
        { label: 'Acura', value: '7' },
        { label: 'Genesis', value: '35' },
        { label: 'Geely', value: '27' },
        { label: 'Proton', value: '19' },
        { label: 'Togg', value: '65' },
        { label: 'Rivian', value: '4' },
        { label: 'Lucid', value: '3' }
    ];
}
```

## Scroll Fade

The mask property adds a gradient fade effect at the scroll edges.

```typescript
import { Component } from '@angular/core';

@Component({
    template: `
        <div class="max-w-56 w-full mx-auto">
            <div class="text-surface-500 dark:text-surface-400 font-medium text-sm uppercase font-mono tracking-tight mb-2">Automobiles</div>
            <p-scrollarea class="h-72" [mask]="true">
                <p-scrollarea-viewport>
                    <p-scrollarea-content class="space-y-2">
                        @for (car of automobiles; track car.label) {
                            <span class="flex items-end gap-0.5">
                                <span class="text-sm hover:underline!">{{ car.label }}</span>
                                <span class="text-surface-500 text-[11px] font-mono mb-px tracking-tighter">({{ car.value }})</span>
                            </span>
                        }
                    </p-scrollarea-content>
                </p-scrollarea-viewport>
                <p-scrollarea-scrollbar orientation="vertical">
                    <p-scrollarea-handle />
                </p-scrollarea-scrollbar>
            </p-scrollarea>
        </div>
    `,
    standalone: true,
    imports: []
})
export class ScrollareaFadeDemo {
    automobiles = [
        { label: 'Audi', value: '200' },
        { label: 'BMW', value: '350' },
        { label: 'Mercedes-Benz', value: '420' },
        { label: 'Volkswagen', value: '510' },
        { label: 'Toyota', value: '610' },
        { label: 'Honda', value: '280' },
        { label: 'Hyundai', value: '390' },
        { label: 'Kia', value: '260' },
        { label: 'Ford', value: '470' },
        { label: 'Renault', value: '530' },
        { label: 'Peugeot', value: '410' },
        { label: 'Citroen', value: '190' },
        { label: 'Skoda', value: '360' },
        { label: 'Seat', value: '210' },
        { label: 'Opel', value: '440' },
        { label: 'Fiat', value: '620' },
        { label: 'Dacia', value: '380' },
        { label: 'Volvo', value: '140' },
        { label: 'Mazda', value: '120' },
        { label: 'Mitsubishi', value: '110' },
        { label: 'Nissan', value: '330' },
        { label: 'Subaru', value: '60' },
        { label: 'Suzuki', value: '170' },
        { label: 'Jeep', value: '95' },
        { label: 'Land Rover', value: '80' },
        { label: 'Porsche', value: '75' },
        { label: 'Ferrari', value: '12' },
        { label: 'Lamborghini', value: '9' },
        { label: 'Bentley', value: '15' },
        { label: 'Rolls-Royce', value: '6' },
        { label: 'Mini', value: '130' },
        { label: 'Tesla', value: '220' },
        { label: 'BYD', value: '55' },
        { label: 'Chery', value: '160' },
        { label: 'MG', value: '145' },
        { label: 'DS Automobiles', value: '40' },
        { label: 'Alfa Romeo', value: '50' },
        { label: 'Lancia', value: '8' },
        { label: 'Cadillac', value: '22' },
        { label: 'Chevrolet', value: '105' },
        { label: 'Dodge', value: '18' },
        { label: 'GMC', value: '14' },
        { label: 'Infiniti', value: '11' },
        { label: 'Acura', value: '7' },
        { label: 'Genesis', value: '35' },
        { label: 'Geely', value: '27' },
        { label: 'Proton', value: '19' },
        { label: 'Togg', value: '65' },
        { label: 'Rivian', value: '4' },
        { label: 'Lucid', value: '3' }
    ];
}
```

## Horizontal

ScrollArea supports horizontal scrolling for content that extends beyond the horizontal viewport.

```typescript
import { Component, OnInit, inject } from '@angular/core';
import { PhotoService } from '@/service/photoservice';
import { Photo } from '@/domain/photo';

@Component({
    template: `
        <div class="flex justify-center">
            <p-scrollarea class="max-w-md mx-auto">
                <p-scrollarea-viewport class="p-3.5!">
                    <p-scrollarea-content>
                        <div class="flex gap-4 min-w-max">
                            @for (image of images; track $index) {
                                <figure class="shrink-0">
                                    <img [src]="image.itemImageSrc" [alt]="image.title" class="w-72 h-40 object-cover rounded-sm" />
                                    <figcaption class="mt-2 text-xs">
                                        <span class="opacity-60">Photo by</span> <span class="font-medium">{{ image.title }}</span>
                                    </figcaption>
                                </figure>
                            }
                        </div>
                    </p-scrollarea-content>
                </p-scrollarea-viewport>
                <p-scrollarea-scrollbar orientation="horizontal">
                    <p-scrollarea-handle />
                </p-scrollarea-scrollbar>
            </p-scrollarea>
        </div>
    `,
    standalone: true,
    imports: [],
    providers: [PhotoService]
})
export class ScrollareaHorizontalDemo implements OnInit {
    images: any[] = [];

    private photoService = inject(PhotoService);

    ngOnInit() {
        this.photoService.getImages().then((data) => (this.images = data));
    }
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';

@Component({
    template: `
        <div class="max-w-56 w-full mx-auto">
            <div class="text-surface-500 dark:text-surface-400 font-medium text-sm uppercase font-mono tracking-tight mb-2">Automobiles</div>
            <p-scrollarea class="h-72">
                <p-scrollarea-viewport>
                    <p-scrollarea-content class="space-y-2">
                        @for (car of automobiles; track car.label) {
                            <span class="flex items-end gap-0.5">
                                <span class="text-sm hover:underline!">{{ car.label }}</span>
                                <span class="text-surface-500 text-[11px] font-mono mb-px tracking-tighter">({{ car.value }})</span>
                            </span>
                        }
                    </p-scrollarea-content>
                </p-scrollarea-viewport>
                <p-scrollarea-scrollbar orientation="vertical">
                    <p-scrollarea-handle />
                </p-scrollarea-scrollbar>
            </p-scrollarea>
        </div>
    `,
    standalone: true,
    imports: []
})
export class ScrollareaPreviewDemo {
    automobiles = [
        { label: 'Audi', value: '200' },
        { label: 'BMW', value: '350' },
        { label: 'Mercedes-Benz', value: '420' },
        { label: 'Volkswagen', value: '510' },
        { label: 'Toyota', value: '610' },
        { label: 'Honda', value: '280' },
        { label: 'Hyundai', value: '390' },
        { label: 'Kia', value: '260' },
        { label: 'Ford', value: '470' },
        { label: 'Renault', value: '530' },
        { label: 'Peugeot', value: '410' },
        { label: 'Citroen', value: '190' },
        { label: 'Skoda', value: '360' },
        { label: 'Seat', value: '210' },
        { label: 'Opel', value: '440' },
        { label: 'Fiat', value: '620' },
        { label: 'Dacia', value: '380' },
        { label: 'Volvo', value: '140' },
        { label: 'Mazda', value: '120' },
        { label: 'Mitsubishi', value: '110' },
        { label: 'Nissan', value: '330' },
        { label: 'Subaru', value: '60' },
        { label: 'Suzuki', value: '170' },
        { label: 'Jeep', value: '95' },
        { label: 'Land Rover', value: '80' },
        { label: 'Porsche', value: '75' },
        { label: 'Ferrari', value: '12' },
        { label: 'Lamborghini', value: '9' },
        { label: 'Bentley', value: '15' },
        { label: 'Rolls-Royce', value: '6' },
        { label: 'Mini', value: '130' },
        { label: 'Tesla', value: '220' },
        { label: 'BYD', value: '55' },
        { label: 'Chery', value: '160' },
        { label: 'MG', value: '145' },
        { label: 'DS Automobiles', value: '40' },
        { label: 'Alfa Romeo', value: '50' },
        { label: 'Lancia', value: '8' },
        { label: 'Cadillac', value: '22' },
        { label: 'Chevrolet', value: '105' },
        { label: 'Dodge', value: '18' },
        { label: 'GMC', value: '14' },
        { label: 'Infiniti', value: '11' },
        { label: 'Acura', value: '7' },
        { label: 'Genesis', value: '35' },
        { label: 'Geely', value: '27' },
        { label: 'Proton', value: '19' },
        { label: 'Togg', value: '65' },
        { label: 'Rivian', value: '4' },
        { label: 'Lucid', value: '3' }
    ];
}
```

## Variant

Change the visibility behavior of scrollbars with the variant property.

```typescript
import { Component } from '@angular/core';
import { ButtonModule } from 'primeng/button';
import { ScrollAreaVariant } from 'primeng/types/scrollarea';

@Component({
    template: `
        <div class="flex flex-col items-center">
            <div class="flex flex-wrap items-center justify-center gap-0.5">
                @for (mode of modes; track mode) {
                    <button type="button" pButton size="small" [severity]="variant === mode ? undefined : 'secondary'" [text]="variant !== mode" class="capitalize cursor-pointer" (click)="variant = mode">{{ mode }}</button>
                }
            </div>
            <div class="mt-8 max-w-56 w-full mx-auto">
                <div class="text-surface-500 dark:text-surface-400 font-medium text-sm uppercase font-mono tracking-tight mb-2">Automobiles</div>
                <p-scrollarea class="h-72" [variant]="variant">
                    <p-scrollarea-viewport>
                        <p-scrollarea-content class="space-y-2">
                            @for (car of automobiles; track car.label) {
                                <span class="flex items-end gap-0.5">
                                    <span class="text-sm hover:underline!">{{ car.label }}</span>
                                    <span class="text-surface-500 text-[11px] font-mono mb-px tracking-tighter">({{ car.value }})</span>
                                </span>
                            }
                        </p-scrollarea-content>
                    </p-scrollarea-viewport>
                    <p-scrollarea-scrollbar orientation="vertical">
                        <p-scrollarea-handle />
                    </p-scrollarea-scrollbar>
                </p-scrollarea>
            </div>
        </div>
    `,
    standalone: true,
    imports: [ButtonModule]
})
export class ScrollareaVariantDemo {
    modes: ScrollAreaVariant[] = ['auto', 'hover', 'scroll', 'always', 'hidden'];

    variant: ScrollAreaVariant = 'auto';

    automobiles = [
        { label: 'Audi', value: '200' },
        { label: 'BMW', value: '350' },
        { label: 'Mercedes-Benz', value: '420' },
        { label: 'Volkswagen', value: '510' },
        { label: 'Toyota', value: '610' },
        { label: 'Honda', value: '280' },
        { label: 'Hyundai', value: '390' },
        { label: 'Kia', value: '260' },
        { label: 'Ford', value: '470' },
        { label: 'Renault', value: '530' },
        { label: 'Peugeot', value: '410' },
        { label: 'Citroen', value: '190' },
        { label: 'Skoda', value: '360' },
        { label: 'Seat', value: '210' },
        { label: 'Opel', value: '440' },
        { label: 'Fiat', value: '620' },
        { label: 'Dacia', value: '380' },
        { label: 'Volvo', value: '140' },
        { label: 'Mazda', value: '120' },
        { label: 'Mitsubishi', value: '110' },
        { label: 'Nissan', value: '330' },
        { label: 'Subaru', value: '60' },
        { label: 'Suzuki', value: '170' },
        { label: 'Jeep', value: '95' },
        { label: 'Land Rover', value: '80' },
        { label: 'Porsche', value: '75' },
        { label: 'Ferrari', value: '12' },
        { label: 'Lamborghini', value: '9' },
        { label: 'Bentley', value: '15' },
        { label: 'Rolls-Royce', value: '6' },
        { label: 'Mini', value: '130' },
        { label: 'Tesla', value: '220' },
        { label: 'BYD', value: '55' },
        { label: 'Chery', value: '160' },
        { label: 'MG', value: '145' },
        { label: 'DS Automobiles', value: '40' },
        { label: 'Alfa Romeo', value: '50' },
        { label: 'Lancia', value: '8' },
        { label: 'Cadillac', value: '22' },
        { label: 'Chevrolet', value: '105' },
        { label: 'Dodge', value: '18' },
        { label: 'GMC', value: '14' },
        { label: 'Infiniti', value: '11' },
        { label: 'Acura', value: '7' },
        { label: 'Genesis', value: '35' },
        { label: 'Geely', value: '27' },
        { label: 'Proton', value: '19' },
        { label: 'Togg', value: '65' },
        { label: 'Rivian', value: '4' },
        { label: 'Lucid', value: '3' }
    ];
}
```

## Scroll Area

ScrollArea is a cross browser, lightweight and themable alternative to native browser scrollbar.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, ScrollAreaPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| variant | "auto" \| "hover" \| "scroll" \| "always" \| "hidden" | - | Scrollbar visibility variant. |
| mask | boolean | - | Whether to show gradient mask at scroll edges. |
| tabindex | number | 0 | Tabindex of the viewport when content overflows. |

### Methods

| Name | Parameters | Return Type | Description |
|------|------------|-------------|-------------|
| refresh |  | void | Refreshes the position and size of the scrollbar. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| viewport | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the viewport's DOM element. |
| content | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content's DOM element. |
| scrollbar | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the scrollbar's DOM element. |
| handle | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the handle's DOM element. |
| corner | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the corner's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-scrollarea | Class name of the root element |
| p-scrollarea-viewport | Class name of the viewport element |
| p-scrollarea-content | Class name of the content element |
| p-scrollarea-scrollbar | Class name of the scrollbar element |
| p-scrollarea-handle | Class name of the handle element |
| p-scrollarea-corner | Class name of the corner element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| scrollarea.background | --p-scrollarea-background | Background of root |
| scrollarea.border.color | --p-scrollarea-border-color | Border color of root |
| scrollarea.border.radius | --p-scrollarea-border-radius | Border radius of root |
| scrollarea.focus.ring.width | --p-scrollarea-focus-ring-width | Focus ring width of root |
| scrollarea.focus.ring.style | --p-scrollarea-focus-ring-style | Focus ring style of root |
| scrollarea.focus.ring.color | --p-scrollarea-focus-ring-color | Focus ring color of root |
| scrollarea.focus.ring.offset | --p-scrollarea-focus-ring-offset | Focus ring offset of root |
| scrollarea.focus.ring.shadow | --p-scrollarea-focus-ring-shadow | Focus ring shadow of root |
| scrollarea.viewport.padding | --p-scrollarea-viewport-padding | Padding of viewport |
| scrollarea.scrollbar.background | --p-scrollarea-scrollbar-background | Background of scrollbar |
| scrollarea.scrollbar.margin | --p-scrollarea-scrollbar-margin | Margin of scrollbar |
| scrollarea.scrollbar.size | --p-scrollarea-scrollbar-size | Size of scrollbar (track thickness) |
| scrollarea.scrollbar.transition.duration | --p-scrollarea-scrollbar-transition-duration | Transition duration of scrollbar |
| scrollarea.handle.background | --p-scrollarea-handle-background | Background of handle |
| scrollarea.mask.fade.size | --p-scrollarea-mask-fade-size | Fade size of mask gradient |
