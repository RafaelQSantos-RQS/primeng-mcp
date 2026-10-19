---
title: "Angular ProgressSpinner Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/progressspinner.md"
description: "ProgressSpinner is a process status indicator that supports both determinate and indeterminate modes."
---

# Angular ProgressSpinner Component

ProgressSpinner is a process status indicator that supports both determinate and indeterminate modes.

## Accessibility

Screen Reader ProgressSpinner components uses progressbar role. Value to describe the component can be defined using aria-labelledby and aria-label props.

## Indeterminate

An animated spinner indicating an indeterminate loading state is displayed by default when no value is provided.

```typescript
import { Component } from '@angular/core';
import { ProgressSpinnerModule } from 'primeng/progressspinner';

@Component({
    template: `
        <div class="flex justify-center">
            <p-progress-spinner ariaLabel="loading" />
        </div>
    `,
    standalone: true,
    imports: [ProgressSpinnerModule]
})
export class ProgressSpinnerBasicDemo {}
```

## Custom

ProgressSpinner can be customized with styling property like strokeWidth and fill .

```typescript
import { Component } from '@angular/core';
import { ProgressSpinnerModule } from 'primeng/progressspinner';

@Component({
    template: `
        <div class="flex justify-center">
            <p-progress-spinner strokeWidth="18" fill="transparent" animationDuration=".5s" [style]="{ width: '50px', height: '50px' }" />
        </div>
    `,
    standalone: true,
    imports: [ProgressSpinnerModule]
})
export class ProgressSpinnerCustomDemo {}
```

## Determinate

Set a numeric value property to display a determinate progress indicator with a track and range.

```typescript
import { Component } from '@angular/core';
import { ProgressSpinnerModule } from 'primeng/progressspinner';

@Component({
    template: `
        <div class="flex justify-center">
            <p-progress-spinner [value]="75" />
        </div>
    `,
    standalone: true,
    imports: [ProgressSpinnerModule]
})
export class ProgressSpinnerDeterminateDemo {}
```

## preview-doc

```typescript
import { Component, OnInit, inject, ChangeDetectorRef } from '@angular/core';
import { ProgressSpinnerModule } from 'primeng/progressspinner';

@Component({
    template: `
        <div class="max-w-2xs mx-auto aspect-square rounded-4xl bg-linear-to-br from-emerald-950 via-emerald-800 to-emerald-700 p-6 flex flex-col text-emerald-50 shadow-xl">
            <div class="text-emerald-200/80 text-sm">Game completed</div>
            <div class="flex items-baseline mt-1 leading-none">
                <span class="text-7xl font-bold tabular-nums tracking-tight">{{ value }}</span>
                <span class="text-3xl font-bold ml-0.5">%</span>
            </div>
            <div class="mt-auto flex items-end justify-between gap-3">
                <div class="text-base leading-snug">
                    <div class="text-emerald-100/90">The legend of <span class="font-bold text-emerald-50">Zelda</span></div>
                    <div class="text-emerald-200/70">Tears of the Kingdom</div>
                </div>
                <p-progress-spinner
                    [value]="value"
                    [strokeWidth]="10"
                    ariaLabel="Loading"
                    class="size-11! shrink-0! [&_.p-progressspinner-circle-range]:stroke-emerald-300! [&_.p-progressspinner-circle-track]:stroke-emerald-100/15! [&_.p-progressspinner-value]:hidden!"
                />
            </div>
        </div>
    `,
    standalone: true,
    imports: [ProgressSpinnerModule]
})
export class ProgressSpinnerPreviewDemo implements OnInit {
    value: number = 0;

    interval: any;

    private cd = inject(ChangeDetectorRef);

    private ngZone = inject(NgZone);

    ngOnInit() {
        this.ngZone.runOutsideAngular(() => {
            this.interval = setInterval(() => {
                this.ngZone.run(() => {
                    if (this.value >= 100) {
                        clearInterval(this.interval);
                        return;
                    }
                    this.value = Math.min(100, this.value + 1);
                });
            }, 80);
        });
    }

    ngOnDestroy() {
        if (this.interval) {
            clearInterval(this.interval);
        }
    }
}
```

## Progress Spinner

ProgressSpinner is a process status indicator.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, ProgressSpinnerPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| value | number \| null \| undefined | null | Current value of the progress. When null or undefined, the spinner is in indeterminate mode. |
| strokeWidth | number | 4 | Width of the circle stroke. |
| animationDuration | string | - | Duration of the rotate animation. |
| min | number | 0 | Minimum value. |
| max | number | 100 | Maximum value. |
| ariaLabel | string \| undefined | - | Used to define a aria label attribute the current element. |
| fill | string | - | Color for the background of the circle. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |
| circle | PassThroughOption<SVGSVGElement, I> | Used to pass attributes to the circle SVG element. |
| circleTrack | PassThroughOption<SVGCircleElement, I> | Used to pass attributes to the circle track element. |
| circleRange | PassThroughOption<SVGCircleElement, I> | Used to pass attributes to the circle range element. |
| value | PassThroughOption<SVGTextElement, I> | Used to pass attributes to the value text element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-progressspinner | Class name of the root element |
| p-progressspinner-circle | Class name of the circle element |
| p-progressspinner-circle-track | Class name of the circle track element |
| p-progressspinner-circle-range | Class name of the circle range element |
| p-progressspinner-value | Class name of the value element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| progressspinner.color.one | --p-progressspinner-color-one | Color one of root |
| progressspinner.color.two | --p-progressspinner-color-two | Color two of root |
| progressspinner.color.three | --p-progressspinner-color-three | Color three of root |
| progressspinner.color.four | --p-progressspinner-color-four | Color four of root |
