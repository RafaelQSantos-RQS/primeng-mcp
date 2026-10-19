---
title: "Angular Compare Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/compare.md"
description: "Compare two items side by side with a slider."
---

# Angular Compare Component

Compare two items side by side with a slider.

## Accessibility

Compare uses a hidden range input for keyboard accessibility. The input supports aria-label , aria-labelledby , aria-valuemin , aria-valuemax , and aria-valuenow attributes. Use arrow keys to adjust the slider position.

## With Chart

Combine slideOnHover with controlled value for interactive chart reveals.

```typescript
import { Component } from '@angular/core';

@Component({
    template: `
        <p-compare class="aspect-video max-w-lg mx-auto" [slideOnHover]="true" [value]="value" (onValueChange)="onValueChange($event)">
            <p-compare-item position="before">
                <svg class="absolute w-full h-full" viewBox="0 0 644 189" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <g clipPath="url(#clip0_1381_2302)">
                        <path
                            d="M0.5 118.499C0.5 118.499 82 102.999 113.5 89.4989C145 75.9989 188.444 87.7869 235 77.4989C272.684 69.1719 293.654 62.4939 329 46.9989C409.332 11.7849 479.5 86.5 510.5 78C541.5 69.5 635.951 0.848863 644 1.49886"
                            stroke="var(--p-primary-color)"
                            stroke-width="2"
                        />
                        <path
                            d="M113.5 89.5006C82 103.001 0.5 118.501 0.5 118.501V188.501H644V1.50065C635.951 0.850647 541.5 69.5 510.5 78C479.5 86.5 409.332 11.7866 329 47.0006C293.654 62.4956 272.684 69.1736 235 77.5006C188.444 87.7886 145 76.0006 113.5 89.5006Z"
                            fill="url(#paint0_linear_540_31)"
                        />
                    </g>
                    <defs>
                        <linearGradient id="paint0_linear_540_31" x1="322.25" x2="322.25" y1="1.477" y2="188.5" gradientUnits="userSpaceOnUse">
                            <stop stop-color="var(--p-primary-color)" stop-opacity="0.4"></stop>
                            <stop offset="1" stop-color="var(--p-primary-color)" stop-opacity="0"></stop>
                        </linearGradient>
                    </defs>
                </svg>
            </p-compare-item>
        </p-compare>
    `,
    standalone: true,
    imports: []
})
export class CompareChartDemo {
    value: number = 50;

    onValueChange(event: CompareValueChangeEvent) {
        this.value = event.value;
    }
}
```

## Custom Handle

The handle can be styled freely — give the indicator a rounded background, custom icon, and shadow for a more prominent look.

```typescript
import { Component } from '@angular/core';

@Component({
    template: `
        <p-compare class="aspect-video max-w-lg mx-auto">
            <p-compare-item position="before">
                <img src="https://primefaces.org/cdn/primevue/images/compare/island2.jpg" />
            </p-compare-item>
            <p-compare-item position="after">
                <img src="https://primefaces.org/cdn/primevue/images/compare/island1.jpg" />
            </p-compare-item>
            <p-compare-handle class="bg-transparent!">
                <p-compare-indicator class="size-5! rounded-full! bg-white/50! transition-transform duration-200! hover:transform-[translate(-50%,-50%)_scale(1.5)]!" />
            </p-compare-handle>
        </p-compare>
    `,
    standalone: true,
    imports: []
})
export class CompareHandleDemo {}
```

## Hover

Enable slideOnHover to update the slider position by hovering over the component.

```typescript
import { Component } from '@angular/core';
import { Code } from '@primeicons/angular/code';

@Component({
    template: `
        <p-compare class="aspect-video max-w-lg mx-auto" [slideOnHover]="true">
            <p-compare-item position="before">
                <img src="https://primefaces.org/cdn/primevue/images/compare/island2.jpg" />
            </p-compare-item>
            <p-compare-item position="after">
                <img src="https://primefaces.org/cdn/primevue/images/compare/island1.jpg" />
            </p-compare-item>
            <p-compare-handle>
                <p-compare-indicator class="group flex items-center justify-center">
                    <svg data-p-icon="code" class="group-data-[orientation=vertical]:rotate-90" />
                </p-compare-indicator>
            </p-compare-handle>
        </p-compare>
    `,
    standalone: true,
    imports: [Code]
})
export class CompareHoverDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { Code } from '@primeicons/angular/code';

@Component({
    template: `
        <p-compare class="aspect-video max-w-lg mx-auto">
            <p-compare-item position="before">
                <img src="https://primefaces.org/cdn/primevue/images/compare/island2.jpg" />
            </p-compare-item>
            <p-compare-item position="after">
                <img src="https://primefaces.org/cdn/primevue/images/compare/island1.jpg" />
            </p-compare-item>
            <p-compare-handle>
                <p-compare-indicator class="group flex items-center justify-center">
                    <svg data-p-icon="code" class="group-data-[orientation=vertical]:rotate-90" />
                </p-compare-indicator>
            </p-compare-handle>
        </p-compare>
    `,
    standalone: true,
    imports: [Code]
})
export class ComparePreviewDemo {}
```

## Template

Compare is not limited to images — any content can be placed inside the items for creative comparisons.

```typescript
import { Component } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { ProgressBarModule } from 'primeng/progressbar';
import { TagModule } from 'primeng/tag';
import { ButtonModule } from 'primeng/button';
import { Code } from '@primeicons/angular/code';

@Component({
    template: `
        <p-compare class="relative w-full max-w-md mx-auto h-80">
            <p-compare-item position="before">
                <div class="size-full bg-purple-100 dark:bg-purple-900/50 p-6 flex items-center justify-center">
                    <div class="w-full max-w-xs rounded-xl border bg-white dark:bg-purple-900 border-purple-100 dark:border-purple-800 p-5 space-y-4">
                        <div class="flex items-start justify-between">
                            <div class="flex items-center gap-3">
                                <p-avatar shape="circle" class="w-10 h-10">
                                    <img class="filter hue-rotate-260 saturate-150" src="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png" />
                                </p-avatar>
                                <div>
                                    <div class="font-medium text-purple-950 dark:text-purple-50">Amy Elsner</div>
                                    <div class="text-sm text-purple-600 dark:text-purple-400">Developer</div>
                                </div>
                            </div>
                            <p-tag class="bg-purple-100 dark:bg-purple-800 text-purple-700 dark:text-purple-200">Pro</p-tag>
                        </div>
                        <div class="space-y-2">
                            <div class="flex justify-between text-sm">
                                <span class="text-purple-600 dark:text-purple-400">Storage</span>
                                <span class="text-purple-950 dark:text-purple-50">7.2 GB / 10 GB</span>
                            </div>
                            <p-progressbar [value]="72" [showValue]="false" class="h-2 [&_.p-progressbar-indicator]:bg-purple-500 [&_.p-progressbar-track]:bg-purple-100 dark:[&_.p-progressbar-track]:bg-purple-800" />
                        </div>
                        <div class="flex gap-2 pt-2">
                            <button pButton severity="help" class="flex-1">Upgrade</button>
                            <button pButton severity="help" variant="outlined">Settings</button>
                        </div>
                    </div>
                </div>
            </p-compare-item>
            <p-compare-item position="after">
                <div class="size-full bg-primary-100 dark:bg-primary-900/50 p-6 flex items-center justify-center">
                    <div class="w-full max-w-xs rounded-xl border bg-white dark:bg-primary-900 border-primary-100 dark:border-primary-800 p-5 space-y-4">
                        <div class="flex items-start justify-between">
                            <div class="flex items-center gap-3">
                                <p-avatar shape="circle" class="w-10 h-10">
                                    <img src="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png" />
                                </p-avatar>
                                <div>
                                    <div class="font-medium text-primary-950 dark:text-primary-50">Amy Elsner</div>
                                    <div class="text-sm text-primary-600 dark:text-primary-400">Developer</div>
                                </div>
                            </div>
                            <p-tag class="bg-primary-100 dark:bg-primary-800 text-primary-700 dark:text-primary-200">Pro</p-tag>
                        </div>
                        <div class="space-y-2">
                            <div class="flex justify-between text-sm">
                                <span class="text-primary-600 dark:text-primary-400">Storage</span>
                                <span class="text-primary-950 dark:text-primary-50">7.2 GB / 10 GB</span>
                            </div>
                            <p-progressbar [value]="72" [showValue]="false" class="h-2 [&_.p-progressbar-indicator]:bg-primary [&_.p-progressbar-track]:bg-primary-100 dark:[&_.p-progressbar-track]:bg-primary-800" />
                        </div>
                        <div class="flex gap-2 pt-2">
                            <button pButton class="flex-1">Upgrade</button>
                            <button pButton variant="outlined">Settings</button>
                        </div>
                    </div>
                </div>
            </p-compare-item>
            <p-compare-handle>
                <p-compare-indicator class="group flex items-center justify-center border border-surface">
                    <svg data-p-icon="code" class="group-data-[orientation=vertical]:rotate-90" />
                </p-compare-indicator>
            </p-compare-handle>
        </p-compare>
    `,
    standalone: true,
    imports: [AvatarModule, ProgressBarModule, TagModule, ButtonModule, Code]
})
export class CompareTemplateDemo {}
```

## Vertical

Set orientation to vertical for a vertical comparison layout.

```typescript
import { Component } from '@angular/core';
import { Code } from '@primeicons/angular/code';

@Component({
    template: `
        <p-compare class="aspect-video max-w-lg mx-auto" orientation="vertical">
            <p-compare-item position="before">
                <img src="https://primefaces.org/cdn/primevue/images/compare/island2.jpg" />
            </p-compare-item>
            <p-compare-item position="after">
                <img src="https://primefaces.org/cdn/primevue/images/compare/island1.jpg" />
            </p-compare-item>
            <p-compare-handle>
                <p-compare-indicator class="group flex items-center justify-center">
                    <svg data-p-icon="code" class="group-data-[orientation=vertical]:rotate-90" />
                </p-compare-indicator>
            </p-compare-handle>
        </p-compare>
    `,
    standalone: true,
    imports: [Code]
})
export class CompareVerticalDemo {}
```

## Compare

Compare allows comparing two items side by side with a slider.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, ComparePassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| value | number \| undefined | - | Current value of the slider position (0-100). |
| defaultValue | number | - | Default value when uncontrolled. |
| orientation | "vertical" \| "horizontal" | - | Orientation of the comparison slider. |
| min | number | - | Minimum value. |
| max | number | - | Maximum value. |
| step | number | - | Step increment. |
| slideOnHover | boolean | - | Whether the slider updates on hover. |
| disabled | boolean | - | Whether the component is disabled. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onValueChange | event: CompareValueChangeEvent | Callback to invoke when value changes. |
| onValueChangeEnd | event: CompareValueChangeEvent | Callback to invoke when dragging ends. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| item | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the item's DOM element. |
| handle | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the handle's DOM element. |
| indicator | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the indicator's DOM element. |
| input | PassThroughOption<HTMLInputElement, I> | Used to pass attributes to the input's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-compare | Class name of the root element |
| p-compare-item | Class name of the item element |
| p-compare-handle | Class name of the handle element |
| p-compare-indicator | Class name of the indicator element |
| p-compare-input | Class name of the input element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| compare.border.radius | --p-compare-border-radius | Border radius of root |
| compare.handle.background | --p-compare-handle-background | Background of handle (track) |
| compare.handle.size | --p-compare-handle-size | Size of handle (thickness; width when vertical, height when horizontal) |
| compare.indicator.size | --p-compare-indicator-size | Size of indicator (square width and height) |
| compare.indicator.background | --p-compare-indicator-background | Background of indicator |
| compare.indicator.border.radius | --p-compare-indicator-border-radius | Border radius of indicator |
| compare.indicator.focus.ring.width | --p-compare-indicator-focus-ring-width | Focus ring width of indicator |
| compare.indicator.focus.ring.style | --p-compare-indicator-focus-ring-style | Focus ring style of indicator |
| compare.indicator.focus.ring.color | --p-compare-indicator-focus-ring-color | Focus ring color of indicator |
| compare.indicator.focus.ring.offset | --p-compare-indicator-focus-ring-offset | Focus ring offset of indicator |
| compare.indicator.icon.color | --p-compare-indicator-icon-color | Icon color of indicator |
| compare.indicator.icon.size | --p-compare-indicator-icon-size | Icon size of indicator |
